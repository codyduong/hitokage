use super::base::Base;
use super::base::BaseMsgHook;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::handlebar::register_hitokage_helpers;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::Reactive;
use crate::structs::reactive::ReactiveString;
use gtk4::prelude::*;
use handlebars::Handlebars;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
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
    let sender_clone = sender.clone();
    let source_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      sender_clone.input(ClockMsg::Tick);
      glib::ControlFlow::Continue
    });

    let mut model = Clock {
      current_time: format_time(&props.format.clone().into()),
      format: props.format.as_reactive_string(None),
      base: props.base.clone().into(),
      source_id: Some(source_id),
    };

    prepend_css_class_to_model!("clock", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      ClockMsg::Tick => self.current_time = format_time(&self.format.clone().get()),
      ClockMsg::LuaHook(hook) => match hook {
        ClockMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "clock", root, base)
        }
        ClockMsgHook::GetFormat(tx) => {
          tx.send(self.format.clone().get()).unwrap();
        }
        ClockMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        ClockMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
        }
      },
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _outputt: relm4::Sender<Self::Output>) {
    self.source_id.take().map(glib::SourceId::remove);
  }
}

fn format_time(format: &String) -> String {
  let time_string= chrono::Local::now().format(&format).to_string();

  let reg = register_hitokage_helpers(Handlebars::new());

  match reg.render_template(&time_string, &None::<()>) {
    Ok(name) => return name,
    Err(err) => {
      log::error!("{:?}", err);
    }
  };

  "".to_owned()
}
