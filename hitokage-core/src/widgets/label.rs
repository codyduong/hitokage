use super::base::Base;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::Reactive;
use crate::structs::reactive::ReactiveString;
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
  GetLabelReactive(Sender<Reactive<String>>),
  SetLabel(String),
}

#[derive(Debug, Clone)]
pub enum LabelMsg {
  LuaHook(LabelMsgHook),
  React,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelProps {
  #[serde(flatten)]
  base: BaseProps,
  label: ReactiveString,
}

#[tracker::track]
pub struct Label {
  #[tracker::do_not_track]
  base: Base,
  #[tracker::do_not_track]
  label: Reactive<String>,
  react: bool,
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
      #[track = "model.changed(Label::react())"]
      set_label: &model.label.clone().get(),
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let mut model = Label {
      base: props.base.clone().into(),
      label: props
        .label
        .as_reactive_string(create_react_sender(sender.input_sender(), LabelMsg::React)),
      react: false,
      tracker: 0,
    };

    prepend_css_class_to_model!("label", model, root);
    set_initial_base_props!(model, root, props.base);

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
          tx.send(self.label.clone().get()).unwrap();
        }
        LabelMsgHook::GetLabelReactive(tx) => {
          tx.send(self.label.clone()).unwrap();
        }
        LabelMsgHook::SetLabel(label) => {
          let arc = self.label.value.clone();
          let mut str = arc.lock().unwrap();
          *str = label;
        }
      },
      LabelMsg::React => {
        self.set_react(!self.react);
      }
    }
  }
}
