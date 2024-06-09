use serde::{Deserialize, Serialize};
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;


#[derive(Debug, Deserialize, Serialize)]
pub struct ClockProps {
  format: String,
}


pub struct Clock {
  current_time: String,
}

#[derive(Debug)]
pub enum ClockMsg {
  Tick,
}

#[relm4::component(pub)]
impl SimpleComponent for Clock {
  type Input = ClockMsg;
  type Output = ();
  type Init = ();
  type Widgets = ClockWidgets;

  view! {
      gtk::Label {
          set_hexpand: true,
          #[watch]
          set_label: &model.current_time,
      }
  }

  fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model = Clock {
      current_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    // Timer
    let sender_clone = sender.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      sender_clone.input(ClockMsg::Tick);
      glib::ControlFlow::Continue
    });

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      ClockMsg::Tick => {
        self.current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      }
    }
  }
}
