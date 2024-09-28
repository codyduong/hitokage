// use super::app::AppMsg;
// use super::base::{Base, BaseMsgHook, BaseProps};
// use super::r#box::{BoxInner, BoxMsgHook, BoxProps};
// use super::ChildUserData;
// use crate::structs::{Monitor, MonitorGeometry, MonitorScaleFactor};
// use crate::win_utils::get_windows_version;
// use crate::{
//   generate_base_match_arms, prepend_css_class,
//   prepend_css_class_to_model, set_initial_base_props,
// };
// use gtk4::prelude::*;
// use relm4::prelude::*;
// use relm4::ComponentParts;
// use relm4::{Component, ComponentSender};
// use serde::{Deserialize, Serialize};

// #[derive(Debug)]
// pub(crate) enum MediaMsgHook {
//   BaseHook(BaseMsgHook),
// }

// #[derive(Debug)]
// pub(crate) enum MediaMsg {
//   LuaHook(MediaMsgHook),
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub(crate) struct MediaProps {
//   #[serde(flatten)]
//   pub base: BaseProps,
// }

// pub(crate) struct Media {
//   base: Base,
// }

// #[relm4::component(pub(crate))]
// impl Component for Media {
//   type Input = MediaMsg;
//   type Output = AppMsg;
//   type Init = MediaProps;
//   type Widgets = AppWidgets;
//   type CommandOutput = ();

//   view! {
//     gtk::Box {
      
//     }
//   }

//   fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
//     let mut model = Media {
//       base: props.base.clone().into(),
//     };

//     prepend_css_class_to_model!("media", model, root);
//     set_initial_base_props!(model, root, props.base);

//     let widgets = view_output!();

//     root.show();

//     ComponentParts { model, widgets }
//   }

//   fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
//     match msg {
//       MediaMsg::LuaHook(hook) => match hook {
//         MediaMsgHook::BaseHook(base) => {
//           generate_base_match_arms!(self, "format", root, base)
//         }
//       },
//     }
//   }
// }