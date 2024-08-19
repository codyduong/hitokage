use super::base::Base;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::handlebar::register_hitokage_helpers;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::Reactive;
use crate::structs::reactive::ReactiveString;
use crate::widgets::base::BaseMsgHook;
use gtk4::prelude::*;
use handlebars::Handlebars;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use systemstat::Platform;
use systemstat::System;

#[derive(Debug, Clone)]
pub enum MemoryMsgHook {
  BaseHook(BaseMsgHook),
  GetFormat(Sender<String>),
  GetFormatReactive(Sender<Reactive<String>>),
  SetFormat(String),
}

#[derive(Debug, Clone)]
pub enum MemoryMsg {
  LuaHook(MemoryMsgHook),
  React,
  Tick,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemoryProps {
  #[serde(flatten)]
  base: BaseProps,
  format: ReactiveString,
}

#[tracker::track]
pub struct Memory {
  #[tracker::do_not_track]
  base: Base,
  mem_and_swap: MemoryAndSwapWrapper,
  #[tracker::do_not_track]
  source_id: Option<glib::SourceId>,
  #[tracker::do_not_track]
  format: Reactive<String>,
  #[tracker::do_not_track]
  sys: System,
  react: bool,
}

#[relm4::component(pub)]
impl Component for Memory {
  type Input = MemoryMsg;
  type Output = ();
  type Init = MemoryProps;
  type Widgets = MemoryWidgets;
  type CommandOutput = ();

  view! {
    gtk::Label {
      #[track = "model.changed(Memory::react() | Memory::mem_and_swap())"]
      set_label: &handle_optional_sys_and_mem(&model.format.clone().get(), &model.mem_and_swap)
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let sender_clone = sender.clone();
    let source_id = glib::timeout_add_local(std::time::Duration::from_millis(1000), move || {
      sender_clone.input(MemoryMsg::Tick);
      glib::ControlFlow::Continue
    });

    let sys = System::new();

    let mem_and_swap: MemoryAndSwapWrapper = sys.memory_and_swap().into();

    let mut model = Memory {
      base: props.base.clone().into(),
      mem_and_swap,
      source_id: Some(source_id),
      format: props
        .format
        .as_reactive_string(create_react_sender(sender.input_sender(), MemoryMsg::React)),
      react: false,
      tracker: 0,
      sys,
    };

    prepend_css_class_to_model!("memory", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      MemoryMsg::LuaHook(hook) => match hook {
        MemoryMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "format", root, base)
        }
        MemoryMsgHook::GetFormat(tx) => {
          tx.send(self.format.clone().get()).unwrap();
        }
        MemoryMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        MemoryMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
        }
      },
      MemoryMsg::React => {
        self.set_react(!self.react);
      }
      MemoryMsg::Tick => {
        self.mem_and_swap = self.sys.memory_and_swap().into();
        self.set_react(!self.react);
      }
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
    self.source_id.take().map(glib::SourceId::remove);
  }
}

#[derive(Debug, Clone)]
struct MemoryAndSwapWrapper {
  memory: Option<systemstat::Memory>,
  swap: Option<systemstat::Swap>,
}

impl PartialEq for MemoryAndSwapWrapper {
  fn ne(&self, other: &Self) -> bool {
    self
      .memory
      .clone()
      .zip(other.memory.clone())
      .map_or(false, |(a, b)| a.free != b.free || a.total != b.total)
      || self
        .swap
        .clone()
        .zip(other.swap.clone())
        .map_or(false, |(a, b)| a.free != b.free || a.total != b.total)
  }

  fn eq(&self, other: &Self) -> bool {
    !self.ne(other)
  }
}

impl From<std::io::Result<(systemstat::Memory, systemstat::Swap)>> for MemoryAndSwapWrapper {
  fn from(value: std::io::Result<(systemstat::Memory, systemstat::Swap)>) -> Self {
    match value {
      Ok(value) => MemoryAndSwapWrapper {
        memory: Some(value.0),
        swap: Some(value.1),
      },
      Err(err) => {
        log::error!("Failed to fetch memory and swap information: {}", err);
        MemoryAndSwapWrapper {
          memory: None,
          swap: None,
        }
      }
    }
  }
}

fn handle_optional_sys_and_mem(format: &String, mem_and_swap: &MemoryAndSwapWrapper) -> String {
  mem_and_swap
    .memory
    .clone()
    .zip(mem_and_swap.swap.clone())
    .map_or(String::new(), |(mem, swap)| format_memory(format, &mem, &swap))
}

fn format_memory(format: &String, memory: &systemstat::Memory, swap: &systemstat::Swap) -> String {
  let reg = register_hitokage_helpers(Handlebars::new());

  let mut args = HashMap::new();

  const BYTES_TO_MB: f64 = 1_048_576.0;

  let free_mb = memory.free.as_u64() as f64 / BYTES_TO_MB;
  let total_mb = memory.total.as_u64() as f64 / BYTES_TO_MB;
  let used_mb = total_mb - free_mb;

  let swap_free_mb = swap.free.as_u64() as f64 / BYTES_TO_MB;
  let swap_total_mb = swap.total.as_u64() as f64 / BYTES_TO_MB;
  let swap_used_mb = swap_total_mb - swap_free_mb;

  args.insert("free".to_string(), free_mb);
  args.insert("total".to_string(), total_mb);
  args.insert("used".to_string(), used_mb);
  args.insert("swap_free".to_string(), swap_free_mb);
  args.insert("swap_total".to_string(), swap_total_mb);
  args.insert("swap_used".to_string(), swap_used_mb);

  match reg.render_template(format, &args) {
    Ok(name) => return name,
    Err(err) => {
      log::error!("{:?}", err);
    }
  };

  "".to_owned()
}
