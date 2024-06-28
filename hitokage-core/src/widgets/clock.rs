use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub enum ClockMsgHook {
  GetFormat(Sender<String>),
  SetFormat(String),
}

#[derive(Debug, Clone)]
pub enum ClockMsg {
  Tick,
  Destroy(Sender<()>),
  LuaHook(ClockMsgHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClockProps {
  format: String,
}

impl From<ClockProps> for Clock {
  fn from(props: ClockProps) -> Self {
    Clock {
      format: props.format.clone(),
      current_time: chrono::Local::now().format(&props.format).to_string(),
      destroyed: Arc::new(Mutex::new(false)),
      destroyed_sender: Arc::new(Mutex::new(None)),
    }
  }
}

#[derive(Clone)]
pub struct Clock {
  format: String,
  current_time: String,
  destroyed: Arc<Mutex<bool>>,
  destroyed_sender: Arc<Mutex<Option<Sender<()>>>>,
}

#[relm4::component(pub)]
impl SimpleComponent for Clock {
  type Input = ClockMsg;
  type Output = ();
  type Init = ClockProps;
  type Widgets = ClockWidgets;

  view! {
    gtk::Box {
      gtk::Label {
        set_hexpand: true,
        #[watch]
        set_label: &model.current_time,
      },
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model: Clock = props.into();

    // Timer
    let sender_clone = sender.clone();
    let destroyed = Arc::clone(&model.destroyed);
    let destroyed_sender = Arc::clone(&model.destroyed_sender);
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      let destroyed = *destroyed.lock().unwrap();
      if !destroyed {
        sender_clone.input(ClockMsg::Tick);
        glib::ControlFlow::Continue
      } else {
        log::warn!("clock stopped");
        let tx = {
          let guard = destroyed_sender.lock().unwrap();
          if let Some(ref tx) = *guard {
            Some(tx.clone())
          } else {
            None
          }
        };
        if let Some(tx) = tx {
          let _ = tx.send(());
        }
        glib::ControlFlow::Break
      }
    });

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      ClockMsg::Tick => {
        self.current_time = chrono::Local::now().format(&self.format).to_string();
      }
      ClockMsg::Destroy(tx) => {
        log::warn!("destroying clock");
        *self.destroyed_sender.lock().unwrap() = Some(tx);
        *self.destroyed.lock().unwrap() = true;
      }
      ClockMsg::LuaHook(hook) => match hook {
        ClockMsgHook::GetFormat(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        ClockMsgHook::SetFormat(format) => {
          self.format = format;
        }
      },
    }
  }
}
