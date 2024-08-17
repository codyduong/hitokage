use super::base::Base;
use super::base::BaseProps;
use super::WidgetUserData;
use crate::generate_base_match_arms;
use crate::generate_box_match_arms;
use crate::generate_box_widgets;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::set_initial_box_props;
use crate::structs::Monitor;
use crate::widgets::base::BaseMsgHook;
use crate::widgets::deserialize_empty_or_seq;
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
  BaseHook(BaseMsgHook),
  GetHomogeneous(Sender<bool>),
  SetHomogeneous(bool),
  GetWidgets(Sender<Vec<WidgetUserData>>),
}

#[derive(Debug, Clone)]
pub enum BoxMsg {
  LuaHook(BoxMsgHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoxProps {
  #[serde(flatten)]
  pub base: BaseProps,
  pub homogeneous: Option<bool>,

  #[serde(default, deserialize_with = "deserialize_empty_or_seq")]
  pub widgets: Option<Vec<WidgetProps>>,
}

pub struct BoxInner {
  pub base: Base,
  pub homogeneous: Option<bool>,
  pub widgets: Vec<WidgetController>,
}

pub struct HitokageBox {
  r#box: BoxInner,
}

#[relm4::component(pub)]
impl Component for HitokageBox {
  type Input = BoxMsg;
  type Output = ();
  type Init = (Monitor, BoxProps);
  type Widgets = BoxWidgets;
  type CommandOutput = ();

  view! {
    gtk::Box {
      set_orientation: gtk::Orientation::Horizontal,
    }
  }

  fn init(input: Self::Init, root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (monitor, props) = input;

    let mut model = HitokageBox {
      r#box: BoxInner {
        widgets: Vec::new(),
        base: Base {
          classes: props.base.class.unwrap_or_default().into(),
          halign: props.base.halign,
          hexpand: props.base.hexpand,
          valign: props.base.valign,
          vexpand: props.base.vexpand,
        },
        homogeneous: props.homogeneous,
      },
    };

    prepend_css_class_to_model!("box", model.r#box, root);
    set_initial_box_props!(model, root);
    let widgets = view_output!();
    generate_box_widgets!(props.widgets, model.r#box, monitor, root);

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      BoxMsg::LuaHook(hook) => {
        generate_box_match_arms!(self, "box", root, BoxMsgHook, hook)
      }
    }
  }
}

#[macro_export]
macro_rules! generate_box_widgets {
  ($widgets:expr, $model: expr, $monitor: expr, $root: expr) => {
    // let mut r#box = $model.r#box;
    for widget in $widgets.unwrap_or_default() {
      let monitor = $monitor.clone();
      match widget {
        WidgetProps::Box(inner_props) => {
          let controller = crate::widgets::r#box::HitokageBox::builder()
            .launch((monitor, inner_props))
            .detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Box(controller));
        }
        WidgetProps::Clock(inner_props) => {
          let controller = crate::widgets::clock::Clock::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Clock(controller));
        }
        WidgetProps::Cpu(inner_props) => {
          let controller = crate::widgets::cpu::Cpu::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Cpu(controller));
        }
        WidgetProps::Icon(inner_props) => {
          let controller = crate::widgets::icon::Icon::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Icon(controller));
        }
        WidgetProps::Label(inner_props) => {
          let controller = crate::widgets::label::Label::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Label(controller));
        }
        WidgetProps::Memory(inner_props) => {
          let controller = crate::widgets::memory::Memory::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Memory(controller));
        }
        WidgetProps::Workspace(inner_props) => {
          use crate::widgets::workspace::Workspace;
          let controller = Workspace::builder().launch((inner_props, monitor.id as u32)).detach();
          $root.append(controller.widget());
          $model.widgets.push(WidgetController::Workspace(controller));
        }
      }
    }
  };
}

#[macro_export]
macro_rules! generate_box_match_arms {
  ($self:expr, $css_name:expr, $root:expr, $msg_type:ident, $hook:expr) => {
    match $hook {
      BoxMsgHook::BaseHook(base) => {
        generate_base_match_arms!($self.r#box, $css_name, $root, base)
      }
      BoxMsgHook::GetHomogeneous(tx) => {
        if let Some(homogeneous) = $self.r#box.homogeneous {
          tx.send(homogeneous).unwrap();
        } else {
          let homogeneous: bool = $root.is_homogeneous().into();
          tx.send(homogeneous).unwrap();
        }
      }
      BoxMsgHook::SetHomogeneous(homogeneous) => {
        $self.r#box.homogeneous = Some(homogeneous);
        $root.set_homogeneous(homogeneous);
      }
      BoxMsgHook::GetWidgets(tx) => {
        tx.send($self.r#box.widgets.iter().map(WidgetUserData::from).collect())
          .unwrap();
      }
    }
  };
}

#[macro_export]
macro_rules! set_initial_box_props {
  ($self: expr,$root:expr) => {
    set_initial_base_props!($self.r#box, $root);
    if let Some(homogeneous) = $self.r#box.homogeneous {
      $root.set_homogeneous(homogeneous);
    }
  };
}
