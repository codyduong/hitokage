use super::WidgetSender;
use super::{WidgetController, WidgetProps};
use crate::lua::monitor::{MonitorGeometry, MonitorScaleFactor};
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
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

pub static BAR: SharedState<HashMap<u32, ComponentSender<Bar>>> = SharedState::new();

fn setup_window_size(window: ApplicationWindow, geometry: &MonitorGeometry, scale_factor: &MonitorScaleFactor) -> anyhow::Result<()> {
  window.set_size_request(geometry.width, (crate::common::HITOKAGE_STATUSBAR_HEIGHT as f32 * scale_factor.y) as i32);

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
      // TODO @codyduong, set this up for user configuration
      windows::Win32::UI::WindowsAndMessaging::HWND_TOP,
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

pub enum BarLuaHook {
  RequestWidgets(Arc<Mutex<Vec<WidgetSender>>>, Sender<()>),
  RequestGeometry(Arc<Mutex<MonitorGeometry>>, Sender<()>),
}

impl std::fmt::Debug for BarLuaHook {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BarLuaHook::RequestWidgets(_, _) => write!(f, "RequestWidgets"),
      BarLuaHook::RequestGeometry(_, _) => write!(f, "RequestGeometry"),
    }
  }
}

#[derive(Debug)]
pub enum BarMsg {
  Destroy,
  LuaHook(BarLuaHook),
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
  pub widgets: Vec<WidgetProps>,
  pub monitor: usize,
  pub scale_factor: MonitorScaleFactor,
  pub id: u32, // win id
}

pub struct Bar {
  position: Option<BarPosition>,
  geometry: MonitorGeometry,
  widgets: Vec<WidgetController>,
  id: u32,
  index: usize,
  scale_factor: MonitorScaleFactor,
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
        let _ = setup_window_size(window.clone(), &model.geometry, &model.scale_factor);
      },

      connect_show => move |window| {
        // Surfaces aren't ready in realize, but they are ready for consumption here
        let _ = setup_window_pos(window.clone(), &model.geometry);
        // reserve_space(&model);
        let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
          model.index,
          komorebi_client::Rect { left: 0, top: crate::common::HITOKAGE_STATUSBAR_HEIGHT, right: 0, bottom: crate::common::HITOKAGE_STATUSBAR_HEIGHT }
        ));
      }
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (props, id, callback) = input;

    // root.connect_scale_factor_notify(move |win| {
    //   // todo @codyduong, needed for if users change scaling on the go
    // });

    callback(sender.clone());

    let mut model = Bar {
      position: props.position,
      geometry: props.geometry.unwrap_or(MonitorGeometry::default()),
      widgets: Vec::new(),
      id: id, //hitokage id
      // windows id
      index: props.monitor,
      scale_factor: props.scale_factor,
    };

    let mut sswg = BAR.write();
    sswg.insert(id, sender);
    drop(sswg);

    let widgets = view_output!();

    for widget in props.widgets {
      match widget {
        WidgetProps::Clock(inner_props) => {
          let controller = Clock::builder().launch(inner_props).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Clock(controller));
        }
        WidgetProps::Workspace(inner_props) => {
          let controller = Workspace::builder().launch((inner_props, props.id)).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Workspace(controller));
        }
        WidgetProps::Box(inner_props) => {
          let controller = crate::widgets::r#box::Box::builder().launch(inner_props).detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Box(controller));
        }
      }
    }

    // manually realize/show
    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      BarMsg::Destroy => {} // todo
      BarMsg::LuaHook(hook) => match hook {
        BarLuaHook::RequestWidgets(arc, tx) => {
          let mut lock = arc.lock().unwrap();
          *lock = self.widgets.iter().map(|i| WidgetSender::from(i)).collect();
          drop(lock);
          tx.send(()).unwrap();
        }
        BarLuaHook::RequestGeometry(arc, tx) => {
          let mut lock = arc.lock().unwrap();
          *lock = self.geometry;
          drop(lock);
          tx.send(()).unwrap();
        }
      },
    }
  }
}
