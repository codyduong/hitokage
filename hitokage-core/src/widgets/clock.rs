use super::base::Base;
use super::base::BaseMsgHook;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::Reactive;
use crate::structs::reactive::ReactiveString;
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
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

impl From<ClockProps> for Clock {
  fn from(props: ClockProps) -> Self {
    Clock {
      current_time: chrono::Local::now().format(&props.format.as_str()).to_string(),
      destroyed: Rc::new(RefCell::new(false)),
      format: props.format.as_reactive_string(None), // we don't need a reactive sender since we reload this regularly
      base: props.base.into(),
    }
  }
}

pub struct Clock {
  current_time: String,
  destroyed: Rc<RefCell<bool>>,
  format: Reactive<String>,
  base: Base,
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
    let mut model: Clock = props.into();

    prepend_css_class_to_model!("clock", model, root);
    set_initial_base_props!(model, root);

    let sender_clone = sender.clone();
    let destroyed = Rc::clone(&model.destroyed);
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      let destroyed = *destroyed.borrow();
      if !destroyed {
        sender_clone.input(ClockMsg::Tick);
        glib::ControlFlow::Continue
      } else {
        glib::ControlFlow::Break
      }
    });

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      ClockMsg::Tick => {
        self.current_time = chrono::Local::now().format(&self.format.clone().get()).to_string();
      }
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
    *self.destroyed.borrow_mut() = true;
  }
}
