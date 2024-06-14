use super::{Widget, WidgetController};
use crate::lua::monitor::MonitorGeometry;
use crate::widgets::clock::Clock;
use crate::widgets::workspace::Workspace;
use crate::win_utils;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::SharedState;
use relm4::SimpleComponent;
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static BAR: SharedState<HashMap<u32, ComponentSender<Bar>>> = SharedState::new();

fn setup_window_size(window: ApplicationWindow, geometry: &MonitorGeometry) -> anyhow::Result<()> {
  window.set_size_request(geometry.width, crate::common::HITOKAGE_STATUSBAR_HEIGHT);

  Ok(())
}

fn setup_window_pos(window: ApplicationWindow, geometry: &MonitorGeometry) -> anyhow::Result<()> {
  // https://discourse.gnome.org/t/set-absolut-window-position-in-gtk4/8552/4
  let native = window.native().expect("Failed to get native");
  let surface = native.surface().expect("Failed to get surface");

  // specifically for windows -> https://discourse.gnome.org/t/how-to-center-gtkwindows-in-gtk4/3112/13
  let handle = surface
    .downcast::<gdk4_win32::Win32Surface>()
    .expect("Failed to get Win32Surface")
    .handle();
  let win_handle = windows::Win32::Foundation::HWND(handle.0);

  log::debug!("Attempting to move {:?}", win_handle);

  unsafe {
    windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
      win_handle,
      windows::Win32::UI::WindowsAndMessaging::HWND_TOPMOST,
      geometry.x,
      geometry.y,
      0,
      0,
      windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE,
    )
    .ok();
  }

  Ok(())
}

#[derive(Debug)]
pub enum BarMsg {
  Tick,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BarPosition {
  Top,
  Bottom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarProps {
  pub position: Option<BarPosition>,
  pub geometry: Option<MonitorGeometry>,
  pub widgets: Vec<Widget>,
}

pub struct Bar {
  position: Option<BarPosition>,
  geometry: MonitorGeometry,
  widgets: Vec<WidgetController>,
  id: u32,
}

#[relm4::component(pub)]
impl SimpleComponent for Bar {
  type Input = BarMsg;
  type Output = ();
  type Init = (BarProps, u32, Box<dyn Fn(ComponentSender<Bar>) -> () + Send>);
  type Widgets = AppWidgets;

  view! {
    gtk::ApplicationWindow {
      set_default_size: (1920, crate::common::HITOKAGE_STATUSBAR_HEIGHT),
      set_resizable: false,
      set_display: &gdk4::Display::default().expect("Failed to get default display"),
      set_decorated: false,
      set_visible: false, // We can't instantiate before we have hooked our connect_* on, so this should always be false

      #[name = "main_box"]
      gtk::Box {
        set_orientation: gtk::Orientation::Horizontal,
        set_hexpand: true,
        set_vexpand: true,
        set_homogeneous: true,
      },

      connect_realize => move |window| {
        let _ = setup_window_size(window.clone(), &model.geometry);
      },

      connect_show => move |window| {
        // Surfaces aren't ready in realize, but they are ready for consumption here
        let _ = setup_window_pos(window.clone(), &model.geometry);
        // reserve_space(&model);
      }
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (props, id, callback) = input;

    callback(sender.clone());

    let mut model = Bar {
      position: props.position,
      geometry: props.geometry.unwrap_or(MonitorGeometry {
        x: 0,
        y: 0,
        width: win_utils::get_primary_width(),
        height: win_utils::get_primary_height(),
      }),
      widgets: Vec::new(),
      id: id,
    };

    let mut sswg = BAR.write();
    sswg.insert(id, sender);
    drop(sswg);

    let widgets = view_output!();

    for widget in props.widgets {
      match widget {
        Widget::Clock(props) => {
          let mut connector = Clock::builder().launch(props);
          widgets.main_box.append(connector.widget());
          connector.detach_runtime();
          // Clock does not need to communicate so detach_runtime();
        }
        Widget::Workspace(props) => {
          let controller = Workspace::builder().launch(props).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Workspace(controller));
        }
        Widget::Box(props) => {
          let mut connector = crate::widgets::r#box::Box::builder().launch(props);
          widgets.main_box.append(connector.widget());
          connector.detach_runtime();
        }
      }
    }

    // manually realize/show
    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}
