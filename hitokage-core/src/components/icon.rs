use super::base::Base;
use super::base::BaseProps;
use crate::components::base::BaseMsgHook;
use crate::generate_base_match_arms;
use crate::get_hitokage_asset;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::AsReactive;
use crate::structs::reactive::Reactive;
use crate::structs::reactive_string::ReactiveString;
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::Deserialize;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum IconMsgHook {
  BaseHook(BaseMsgHook),
  GetFile(Sender<String>),
  GetFileReactive(Sender<Reactive<String>>),
  SetFile(String),
}

#[derive(Debug, Clone)]
pub enum IconMsg {
  LuaHook(IconMsgHook),
  React,
}

#[derive(Debug, Deserialize)]
pub struct IconProps {
  #[serde(flatten)]
  base: BaseProps,
  file: ReactiveString,
}

#[tracker::track]
pub struct Icon {
  #[tracker::do_not_track]
  base: Base,
  #[tracker::do_not_track]
  file: Reactive<String>,
  react: bool,
}

#[relm4::component(pub)]
impl Component for Icon {
  type Input = IconMsg;
  type Output = ();
  type Init = IconProps;
  type Widgets = IconWidgets;
  type CommandOutput = ();

  view! {
    gtk::Image {
      #[track = "model.changed(Icon::react())"]
      set_file: Some(get_relative_path(model.file.get()).as_str()),
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let mut model = Icon {
      base: props.base.clone().into(),
      file: props
        .file
        .as_reactive(create_react_sender(sender.input_sender(), IconMsg::React)),
      react: false,
      tracker: 0,
    };

    prepend_css_class_to_model!("icon", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      IconMsg::LuaHook(hook) => match hook {
        IconMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "icon", root, base)
        }
        IconMsgHook::GetFile(tx) => {
          tx.send(self.file.get()).unwrap();
        }
        IconMsgHook::GetFileReactive(tx) => {
          tx.send(self.file.clone()).unwrap();
        }
        IconMsgHook::SetFile(icon) => {
          let arc = self.file.value.clone();
          let mut str = arc.lock().unwrap();
          *str = icon;
          self.set_react(!self.react);
        }
      },
      IconMsg::React => {
        self.set_react(!self.react);
      }
    }
  }
}

fn get_relative_path(s: String) -> String {
  get_hitokage_asset(s).into_os_string().into_string().unwrap()
}
