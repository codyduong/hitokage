use super::WidgetUserData;
use crate::common::CssClass;
use crate::lua::monitor::Monitor;
use crate::prepend_css_class;
use crate::widgets::clock::Clock;
use crate::widgets::workspace::Workspace;
use crate::widgets::WidgetController;
use crate::widgets::WidgetProps;
use gtk4::prelude::*;
use indexmap::IndexSet;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum BoxMsgHook {
  GetClass(Sender<Vec<String>>),
  SetClass(Option<CssClass>),
  GetWidgets(Sender<Vec<WidgetUserData>>),
}

#[derive(Debug, Clone)]
pub enum BoxMsg {
  LuaHook(BoxMsgHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoxProps {
  widgets: Option<Vec<WidgetProps>>,
  class: Option<CssClass>,
}

pub struct Box {
  monitor: Monitor,

  widgets: Vec<WidgetController>,
  classes: Vec<String>,
}

#[relm4::component(pub)]
impl Component for Box {
  type Input = BoxMsg;
  type Output = ();
  type Init = (Monitor, BoxProps);
  type Widgets = BoxWidgets;
  type CommandOutput = ();

  view! {
    gtk::Box {
      set_orientation: gtk::Orientation::Horizontal,
      set_hexpand: true,
      set_vexpand: true,
      set_homogeneous: true,
    }
  }

  fn init(input: Self::Init, root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (monitor, props) = input;

    let mut model = Box {
      monitor,
      widgets: Vec::new(),
      classes: prepend_css_class!("box", props.class.unwrap_or_default()),
    };

    let classes_ref: Vec<&str> = model.classes.iter().map(AsRef::as_ref).collect();
    root.set_css_classes(&classes_ref);

    let widgets = view_output!();

    for widget in props.widgets.unwrap_or_default() {
      let monitor = model.monitor.clone();
      match widget {
        WidgetProps::Clock(inner_props) => {
          let controller = Clock::builder().launch(inner_props).detach();
          root.append(controller.widget());
          model.widgets.push(WidgetController::Clock(controller));
        }
        WidgetProps::Workspace(inner_props) => {
          let controller = Workspace::builder().launch((inner_props, monitor.id as u32)).detach();
          root.append(controller.widget());
          model.widgets.push(WidgetController::Workspace(controller));
        }
        WidgetProps::Box(inner_props) => {
          let controller = crate::widgets::r#box::Box::builder()
            .launch((monitor, inner_props))
            .detach();
          root.append(controller.widget());
          model.widgets.push(WidgetController::Box(controller));
        }
      }
    }

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      BoxMsg::LuaHook(hook) => match hook {
        BoxMsgHook::GetClass(tx) => {
          tx.send(self.classes.clone()).unwrap();
        }
        BoxMsgHook::SetClass(classes) => {
          self.classes = prepend_css_class!("box", classes.unwrap_or_default());
          let classes_ref: Vec<&str> = self.classes.iter().map(AsRef::as_ref).collect();
          root.set_css_classes(&classes_ref);
        }
        BoxMsgHook::GetWidgets(tx) => {
          tx.send(self.widgets.iter().map(|i| WidgetUserData::from(i)).collect())
            .unwrap();
        }
      },
    }
  }
}
