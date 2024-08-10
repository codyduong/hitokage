use super::base::Base;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::widgets::base::BaseMsgHook;
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum LabelMsgHook {
  BaseHook(BaseMsgHook),
  GetLabel(Sender<String>),
  SetLabel(String),
}

#[derive(Debug, Clone)]
pub enum LabelMsg {
  LuaHook(LabelMsgHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelProps {
  #[serde(flatten)]
  base: BaseProps,
  label: String,
}

pub struct Label {
  base: Base,
  label: String,
}

#[relm4::component(pub)]
impl Component for Label {
  type Input = LabelMsg;
  type Output = ();
  type Init = LabelProps;
  type Widgets = LabelWidgets;
  type CommandOutput = ();

  view! {
    gtk::Label {
      #[watch]
      set_label: &model.label,
    }
  }

  fn init(props: Self::Init, root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let mut model = Label {
      base: Base {
        classes: props.base.class.unwrap_or_default().into(),
        halign: props.base.halign,
        hexpand: props.base.hexpand.or(Some(true)),
        valign: props.base.valign,
        vexpand: props.base.vexpand.or(Some(true)),
      },
      label: props.label,
    };

    prepend_css_class_to_model!("label", model, root);
    set_initial_base_props!(model, root);

    let widgets = view_output!();

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      LabelMsg::LuaHook(hook) => match hook {
        LabelMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "label", root, base)
        }
        LabelMsgHook::GetLabel(tx) => {
          tx.send(self.label.clone()).unwrap();
        }
        LabelMsgHook::SetLabel(label) => {
          self.label = label;
        }
      },
    }
  }
}
