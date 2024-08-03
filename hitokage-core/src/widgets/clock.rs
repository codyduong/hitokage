use crate::common::Align;
use crate::common::CssClass;
use crate::prepend_css_class;
use gtk4::prelude::*;
use indexmap::IndexSet;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub enum ClockMsgHook {
  GetFormat(Sender<String>),
  SetFormat(String),
  GetHalign(Sender<Align>),
  SetHalign(Align),
}

#[derive(Debug, Clone)]
pub enum ClockMsg {
  Tick,
  LuaHook(ClockMsgHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClockProps {
  format: String,
  halign: Option<Align>,
  class: Option<CssClass>,
}

impl From<ClockProps> for Clock {
  fn from(props: ClockProps) -> Self {
    Clock {
      current_time: chrono::Local::now().format(&props.format).to_string(),
      destroyed: Arc::new(Mutex::new(false)),
      format: props.format.clone(),
      halign: props.halign.unwrap_or(Align::Start),
      classes: prepend_css_class!("clock", props.class.unwrap_or_default()),
    }
  }
}

pub struct Clock {
  current_time: String,
  destroyed: Arc<Mutex<bool>>,

  format: String,
  halign: Align,
  classes: Vec<String>,
}

#[relm4::component(pub)]
impl Component for Clock {
  type Input = ClockMsg;
  type Output = ();
  type Init = ClockProps;
  type Widgets = ClockWidgets;
  type CommandOutput = ();

  view! {
    gtk::Box {
      gtk::Label {
        set_hexpand: false,
        #[watch]
        set_label: &model.current_time,
      },
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model: Clock = props.into();

    let classes_ref: Vec<&str> = model.classes.iter().map(AsRef::as_ref).collect();
    root.set_css_classes(&classes_ref);
    root.set_halign(model.halign.into());

    // Timer
    let sender_clone = sender.clone();
    let destroyed = Arc::clone(&model.destroyed);
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      let destroyed = *destroyed.lock().unwrap();
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
        self.current_time = chrono::Local::now().format(&self.format).to_string();
      }
      ClockMsg::LuaHook(hook) => match hook {
        ClockMsgHook::GetFormat(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        ClockMsgHook::SetFormat(format) => {
          self.format = format;
        }
        ClockMsgHook::GetHalign(tx) => {
          tx.send(self.halign.clone()).unwrap();
        }
        ClockMsgHook::SetHalign(halign) => {
          root.set_halign(halign.clone().into());
          self.halign = halign
        }
      },
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _outputt: relm4::Sender<Self::Output>) {
    *self.destroyed.lock().unwrap() = true;
  }
}
