use crate::lua::monitor::Monitor;
use crate::widgets::clock::Clock;
use crate::widgets::workspace::Workspace;
use crate::widgets::WidgetController;
use crate::widgets::WidgetProps;
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BoxProps {
  widgets: Option<Vec<WidgetProps>>,
}

pub struct Box {
  monitor: Monitor,
  widgets: Vec<WidgetController>,
}

#[relm4::component(pub)]
impl SimpleComponent for Box {
  type Input = ();
  type Output = ();
  type Init = (Monitor, BoxProps);
  type Widgets = BoxWidgets;

  view! {
    #[name = "main_box"]
    gtk::Box {
      set_orientation: gtk::Orientation::Horizontal,
      set_hexpand: true,
      set_vexpand: true,
      set_homogeneous: true,
    }
  }

  fn init(input: Self::Init, root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
    println!("bar: {:?}", input);

    let (monitor, props) = input;

    let mut model = Box {
      monitor,
      widgets: Vec::new(),
    };

    let widgets = view_output!();

    for widget in props.widgets.unwrap_or_default() {
      let monitor = model.monitor.clone();
      match widget {
        WidgetProps::Clock(inner_props) => {
          let controller = Clock::builder().launch(inner_props).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Clock(controller));
        }
        WidgetProps::Workspace(inner_props) => {
          let controller = Workspace::builder().launch((inner_props, monitor.id as u32)).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Workspace(controller));
        }
        WidgetProps::Box(inner_props) => {
          let controller = crate::widgets::r#box::Box::builder().launch((monitor, inner_props)).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Box(controller));
        }
      }
    }

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      _ => {}
    }
  }
}
