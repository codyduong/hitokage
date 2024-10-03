use super::app::AppMsg;
use super::base::Base;
use super::base::BaseProps;
use super::r#box::BoxMsg;
use crate::components::base::BaseMsgHook;
use crate::generate_base_match_arms;
use crate::handlebar::register_hitokage_helpers;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::lua_fn::LuaFn;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::AsReactive;
use crate::structs::reactive::Reactive;
use crate::structs::reactive_string_fn::ReactiveStringFn;
use gtk4::prelude::*;
use handlebars::Handlebars;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use systemstat::Platform;
use systemstat::System;

const BYTES_TO_MB: f64 = 1_048_576.0;

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
  Callback(std::sync::mpsc::Sender<mlua::Value>),
}

#[derive(Debug)]
pub enum MemoryMsgOut {
  RequestLuaAction(
    Arc<mlua::RegistryKey>,
    serde_json::Value,
    std::sync::mpsc::Sender<mlua::Value>,
  ),
}

impl From<MemoryMsgOut> for AppMsg {
  fn from(value: MemoryMsgOut) -> Self {
    match value {
      MemoryMsgOut::RequestLuaAction(a, b, c) => AppMsg::RequestLuaAction(a, b, c),
      #[allow(unreachable_patterns)]
      _ => AppMsg::NoOp,
    }
  }
}

impl From<MemoryMsgOut> for BoxMsg {
  fn from(value: MemoryMsgOut) -> Self {
    match value {
      MemoryMsgOut::RequestLuaAction(a, b, c) => BoxMsg::AppMsg(AppMsg::RequestLuaAction(a, b, c)),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct MemoryProps {
  #[serde(flatten)]
  base: BaseProps,
  format: ReactiveStringFn,
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
  #[tracker::do_not_track]
  callback: Option<LuaFn>,
}

#[relm4::component(pub)]
impl Component for Memory {
  type Input = MemoryMsg;
  type Output = MemoryMsgOut;
  type Init = MemoryProps;
  type Widgets = MemoryWidgets;
  type CommandOutput = ();

  view! {
    gtk::Label {
      #[track = "model.changed(Memory::react() | Memory::mem_and_swap())"]
      set_label: &handle_optional_sys_and_mem(&model.format.get(), &model.mem_and_swap)
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let callback = props.format.as_fn();
    let reactive = props
      .format
      .as_reactive(create_react_sender(sender.input_sender(), MemoryMsg::React));

    let source_id = {
      let sender = sender.clone();
      let reactive = reactive.clone();

      let res = match callback {
        Some(_) => glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
          sender.input(MemoryMsg::Tick);
          let (tx, rx) = std::sync::mpsc::channel::<_>();
          sender.input(MemoryMsg::Callback(tx.clone()));
          match rx.try_recv() {
            Ok(v) => match v {
              mlua::Value::String(s) => {
                reactive.set(s.to_string_lossy());
              }
              _ => {
                log::error!("Expected string for memory callback, received: {:?}", v);
              }
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
              log::error!("Memory callback dropped");
            }
          }
          glib::ControlFlow::Continue
        }),
        None => glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
          sender.input(MemoryMsg::Tick);
          glib::ControlFlow::Continue
        }),
      };

      res
    };

    let sys = System::new();

    let mem_and_swap: MemoryAndSwapWrapper = sys.memory_and_swap().into();

    let mut model = Memory {
      base: props.base.clone().into(),
      mem_and_swap,
      source_id: Some(source_id),
      format: reactive,
      callback,
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

  fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      MemoryMsg::LuaHook(hook) => match hook {
        MemoryMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "format", root, base)
        }
        MemoryMsgHook::GetFormat(tx) => {
          tx.send(self.format.get()).unwrap();
        }
        MemoryMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        MemoryMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
          self.set_react(!self.react);
        }
      },
      MemoryMsg::React => {
        self.set_react(!self.react);
      }
      MemoryMsg::Tick => {
        self.mem_and_swap = self.sys.memory_and_swap().into();
        self.set_react(!self.react);
      }
      MemoryMsg::Callback(tx) => {
        if let Some(callback) = &self.callback {
          let _ = sender.output(MemoryMsgOut::RequestLuaAction(
            callback.r.clone(),
            serde_json::to_value(&self.mem_and_swap.as_lua_args()).unwrap(),
            tx.clone(),
          ));
        }
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

impl MemoryAndSwapWrapper {
  fn as_lua_args(&self) -> MemoryInfo {
    let free_mb = self.memory.clone().unwrap().free.as_u64() as f64 / BYTES_TO_MB;
    let total_mb = self.memory.clone().unwrap().total.as_u64() as f64 / BYTES_TO_MB;
    let used_mb = total_mb - free_mb;

    let swap_free_mb = self.swap.clone().unwrap().free.as_u64() as f64 / BYTES_TO_MB;
    let swap_total_mb = self.swap.clone().unwrap().total.as_u64() as f64 / BYTES_TO_MB;
    let swap_used_mb = swap_total_mb - swap_free_mb;

    MemoryInfo {
      free: free_mb,
      total: total_mb,
      used: used_mb,
      swap_free: swap_free_mb,
      swap_total: swap_total_mb,
      swap_used: swap_used_mb,
    }
  }
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

#[derive(Debug, Clone, Serialize)]
struct MemoryInfo {
  free: f64,
  total: f64,
  used: f64,
  swap_free: f64,
  swap_total: f64,
  swap_used: f64,
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
