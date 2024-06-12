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
use crate::RelmContainerExtManual;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceProps {}

pub struct Workspace {}

#[relm4::component(pub)]
impl SimpleComponent for Workspace {
  type Input = ();
  type Output = ();
  type Init = WorkspaceProps;
  type Widgets = WorkspaceWidgets;

  view! {
    gtk::FlowBox {
      set_height_request: 16,

      gtk::FlowBoxChild {
        set_width_request: 12,
        gtk::Label {
          set_label: "1"
        },
      },
      gtk::Label {
        set_label: "2"
      },
      gtk::Label {
        set_label: "3"
      },
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model = Workspace {};

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      _ => {}
    }
  }
}
