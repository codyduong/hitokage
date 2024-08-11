use super::base::Base;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::Reactive;
use crate::structs::reactive::ReactiveString;
use crate::widgets::base::BaseMsgHook;
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IconProps {
  #[serde(flatten)]
  base: BaseProps,
  file: ReactiveString,
}

pub struct Icon {
  base: Base,
  file: Reactive<String>,
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
      #[watch]
      set_file: Some(get_relative_path(model.file.clone().into_inner()).as_str()),
    }
  }

  fn init(props: Self::Init, root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let mut model = Icon {
      base: Base {
        classes: props.base.class.unwrap_or_default().into(),
        halign: props.base.halign,
        hexpand: props.base.hexpand.or(Some(true)),
        valign: props.base.valign,
        vexpand: props.base.vexpand.or(Some(true)),
      },
      file: props.file.into(),
    };

    log::debug!("{:?}", get_relative_path(model.file.clone().into_inner()));

    prepend_css_class_to_model!("icon", model, root);
    set_initial_base_props!(model, root);

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
          tx.send(self.file.clone().into_inner()).unwrap();
        }
        IconMsgHook::GetFileReactive(tx) => {
          tx.send(self.file.clone()).unwrap();
        }
        IconMsgHook::SetFile(icon) => {
          let arc = self.file.value.clone();
          let mut str = arc.lock().unwrap();
          *str = icon;
        }
      },
    }
  }
}

fn get_relative_path(s: String) -> String {
  let file_path = if cfg!(feature = "development") {
    let mut path = Path::new(file!()).parent().unwrap().to_path_buf();
    path.push("../../../example/");
    path.push(s);
    fs::canonicalize(path).expect("Failed to canonicalize path")
  } else {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".config/hitokage/");
    path.push(s);
    path
  };
  file_path.into_os_string().into_string().unwrap()
}
