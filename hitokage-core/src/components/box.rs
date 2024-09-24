use super::app::AppMsg;
use super::base::Base;
use super::base::BaseProps;
use super::ChildUserData;
use crate::components::base::BaseMsgHook;
use crate::components::deserialize_empty_or_seq;
use crate::components::ChildController;
use crate::generate_base_match_arms;
use crate::generate_box_children;
use crate::generate_box_match_arms;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::set_initial_box_props;
use crate::structs::Monitor;
use gtk4::prelude::*;
use gtk4::Widget;
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
  GetChildren(Sender<Vec<ChildUserData>>),
}

#[derive(Debug, Clone)]
pub enum ChildMsg {
  Remove(Widget),
}

#[derive(Debug)]
pub enum BoxMsg {
  LuaHook(BoxMsgHook),
  ChildMsg(ChildMsg),
  AppMsg(AppMsg),
}

#[derive(Debug)]
pub enum BoxMsgPortable {
  LuaHook(BoxMsgHook),
}

impl From<BoxMsgPortable> for BoxMsg {
  fn from(value: BoxMsgPortable) -> Self {
    match value {
      BoxMsgPortable::LuaHook(luahook) => BoxMsg::LuaHook(luahook),
    }
  }
}

impl Into<BoxMsg> for AppMsg {
  fn into(self) -> BoxMsg {
    BoxMsg::AppMsg(self)
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoxProps {
  #[serde(flatten)]
  pub base: BaseProps,
  pub homogeneous: Option<bool>,

  #[serde(default, deserialize_with = "deserialize_empty_or_seq", alias = "widgets")]
  pub children: Option<Vec<super::Child>>,
}

impl Default for BoxProps {
  fn default() -> Self {
    Self {
      base: BaseProps::default(),
      homogeneous: None,
      children: Some(Vec::new()),
    }
  }
}

pub struct BoxInner {
  pub base: Base,
  pub homogeneous: Option<bool>,
  pub children: Vec<ChildController>,
}

pub struct HitokageBox {
  r#box: BoxInner,
}

#[relm4::component(pub)]
impl Component for HitokageBox {
  type Input = BoxMsg;
  type Output = AppMsg;
  type Init = (Monitor, BoxProps);
  type Widgets = BoxWidgets;
  type CommandOutput = ();

  view! {
    gtk::Box {
      set_orientation: gtk::Orientation::Horizontal,
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (monitor, props) = input;

    let mut model = HitokageBox {
      r#box: BoxInner {
        children: Vec::new(),
        base: props.base.clone().into(),
        homogeneous: props.homogeneous,
      },
    };

    prepend_css_class_to_model!("box", model.r#box, root);
    set_initial_box_props!(model, root, props.base);
    let widgets = view_output!();
    generate_box_children!(props.children, model.r#box, monitor, root, sender.input_sender());

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      BoxMsg::LuaHook(hook) => {
        generate_box_match_arms!(self, "box", root, BoxMsgHook, hook)
      }
      BoxMsg::AppMsg(msg) => {
        sender.output(msg).unwrap();
      }
      BoxMsg::ChildMsg(msg) => match msg {
        ChildMsg::Remove(child) => {
          root.remove(&child);
          self.r#box.children.retain(|w| w.widget() != child);
        }
      },
    };
  }
}

#[macro_export]
macro_rules! generate_box_children {
  ($children:expr, $model: expr, $monitor: expr, $root: expr, $input_sender: expr) => {
    for child in $children.unwrap_or_default() {
      use crate::components::Child;
      use crate::components::ChildController;
      let monitor = $monitor.clone();
      match child {
        Child::Battery(inner_props) => {
          let controller = crate::components::battery::Battery::builder()
            .launch(inner_props)
            .forward($input_sender, |m| m.into());
          $root.append(controller.widget());
          $model.children.push(ChildController::Battery(controller));
        }
        Child::Box(inner_props) => {
          let controller = crate::components::r#box::HitokageBox::builder()
            .launch((monitor, inner_props))
            .forward($input_sender, |m| m.into());
          $root.append(controller.widget());
          $model.children.push(ChildController::Box(controller));
        }
        Child::Clock(inner_props) => {
          let controller = crate::components::clock::Clock::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.children.push(ChildController::Clock(controller));
        }
        Child::Cpu(inner_props) => {
          let controller = crate::components::cpu::Cpu::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.children.push(ChildController::Cpu(controller));
        }
        Child::Icon(inner_props) => {
          let controller = crate::components::icon::Icon::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.children.push(ChildController::Icon(controller));
        }
        Child::Label(inner_props) => {
          let controller = crate::components::label::Label::builder().launch(inner_props).detach();
          $root.append(controller.widget());
          $model.children.push(ChildController::Label(controller));
        }
        Child::Memory(inner_props) => {
          let controller = crate::components::memory::Memory::builder()
            .launch(inner_props)
            .detach();
          $root.append(controller.widget());
          $model.children.push(ChildController::Memory(controller));
        }
        Child::Weather(inner_props) => {
          let controller = crate::components::weather::Weather::builder()
            .launch(inner_props)
            .forward($input_sender, |m| m.into());
          $root.append(controller.widget());
          $model.children.push(ChildController::Weather(controller));
        }
        Child::Workspace(inner_props) => {
          use crate::components::workspace::Workspace;
          let controller = Workspace::builder().launch((inner_props, monitor.id as u32)).detach();
          $root.append(controller.widget());
          $model.children.push(ChildController::Workspace(controller));
        }
      };
    }
    $model.children.shrink_to_fit();
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
      BoxMsgHook::GetChildren(tx) => {
        tx.send($self.r#box.children.iter().map(ChildUserData::from).collect())
          .unwrap();
      }
    }
  };
}

#[macro_export]
macro_rules! set_initial_box_props {
  ($self: expr,$root:expr,$base_props:expr) => {
    set_initial_base_props!($self.r#box, $root, $base_props);
    if let Some(homogeneous) = $self.r#box.homogeneous {
      $root.set_homogeneous(homogeneous);
    }
  };
}
