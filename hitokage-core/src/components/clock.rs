use super::base::Base;
use super::base::BaseMsgHook;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::handlebar::register_hitokage_helpers;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::AsReactive;
use crate::structs::reactive::Reactive;
use crate::structs::reactive_string::ReactiveString;
use gtk4::prelude::*;
use handlebars::Handlebars;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::Deserialize;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum ClockMsgHook {
  BaseHook(BaseMsgHook),
  GetFormat(Sender<String>),
  GetFormatReactive(Sender<Reactive<String>>),
  SetFormat(String),
}

#[derive(Debug, Clone)]
pub enum ClockMsg {
  Tick,
  LuaHook(ClockMsgHook),
}

#[derive(Debug, Deserialize)]
pub struct ClockProps {
  format: ReactiveString,
  #[serde(flatten)]
  base: BaseProps,
}

pub struct Clock {
  current_time: String,
  format: Reactive<String>,
  base: Base,
  source_id: Option<glib::SourceId>,
}

#[relm4::component(pub)]
impl Component for Clock {
  type Input = ClockMsg;
  type Output = ();
  type Init = ClockProps;
  type Widgets = ClockWidgets;
  type CommandOutput = ();

  view! {
    gtk::Label {
      set_hexpand: false,
      #[watch]
      set_label: &model.current_time,
    },
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let mut model = Clock {
      current_time: "".to_string(),
      format: props.format.as_reactive(None),
      base: props.base.clone().into(),
      source_id: None,
    };
    format_time(&mut model, sender);

    prepend_css_class_to_model!("clock", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      ClockMsg::Tick => format_time(self, sender),
      ClockMsg::LuaHook(hook) => match hook {
        ClockMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "clock", root, base)
        }
        ClockMsgHook::GetFormat(tx) => {
          tx.send(self.format.get()).unwrap();
        }
        ClockMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        ClockMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
          format_time(self, sender);
        }
      },
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _outputt: relm4::Sender<Self::Output>) {
    if let Some(a) = self.source_id.take() {
      glib::SourceId::remove(a)
    }
  }
}

fn format_time(model: &mut Clock, sender: ComponentSender<Clock>) {
  let time_string = chrono::Local::now().format(&model.format.get()).to_string();

  let reg = register_hitokage_helpers(Handlebars::new());

  match reg.render_template(&time_string, &None::<()>) {
    Ok(name) => {
      if model.source_id.is_none() {
        let source_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
          sender.input(ClockMsg::Tick);
          glib::ControlFlow::Continue
        });
        model.source_id = Some(source_id);
      }
      model.current_time = name.clone();
    }
    Err(err) => {
      log::error!("{:?}", err);
      if let Some(a) = model.source_id.take() {
        glib::SourceId::remove(a)
      };
    }
  };
}
