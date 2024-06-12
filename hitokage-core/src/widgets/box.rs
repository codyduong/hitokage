// Wrapper around gtk4 box

use crate::lua::monitor::MonitorGeometry;
use crate::win_utils;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::SharedState;
use relm4::SimpleComponent;
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BoxProps {}

pub struct Box {}

#[relm4::component(pub)]
impl SimpleComponent for Box {
  type Input = ();
  type Output = ();
  type Init = BoxProps;
  type Widgets = BoxWidgets;

  view! {
    gtk::Box {

    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model = Box {};

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      _ => {}
    }
  }
}
