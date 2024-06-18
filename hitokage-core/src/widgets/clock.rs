use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ClockMsg {
  Tick,
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
    }
  }
}

#[derive(Clone)]
pub struct Clock {
  format: String,
  current_time: String,
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
        self.current_time = chrono::Local::now().format(&self.format).to_string();
      }
    }
  }
}
