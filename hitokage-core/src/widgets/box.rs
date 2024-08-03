use super::base::Base;
use super::base::BaseProps;
use super::WidgetUserData;
use crate::prepend_css_class_to_model;
use crate::structs::{CssClass, Monitor};
use crate::widgets::clock::Clock;
use crate::widgets::workspace::Workspace;
use crate::widgets::WidgetController;
use crate::widgets::WidgetProps;
use gtk4::prelude::*;
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
  #[serde(flatten)]
  base: BaseProps,
}

pub struct Box {
  monitor: Monitor,
  widgets: Vec<WidgetController>,
  base: Base,
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
      base: props.base.into(),
    };

    prepend_css_class_to_model!("box", model, root);

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
          tx.send(self.base.classes.clone()).unwrap();
        }
        BoxMsgHook::SetClass(classes) => {
          prepend_css_class_to_model!(self, "box", classes, root);
        }
        BoxMsgHook::GetWidgets(tx) => {
          tx.send(self.widgets.iter().map(|i| WidgetUserData::from(i)).collect())
            .unwrap();
        }
      },
    }
  }
}
