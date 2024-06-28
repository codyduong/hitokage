use super::clock::ClockMsg;
use super::WidgetUserData;
use super::{WidgetController, WidgetProps};
use crate::lua::monitor::{Monitor, MonitorGeometry, MonitorScaleFactor};
use crate::widgets::clock::Clock;
use crate::widgets::workspace::Workspace;
use crate::win_utils::get_windows_version;
use gtk4::prelude::*;
use gtk4::Window;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::SharedState;
use relm4::SimpleComponent;
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub static BAR: SharedState<HashMap<u32, ComponentSender<Bar>>> = SharedState::new();

fn setup_window_size(
  window: Window,
  geometry: &MonitorGeometry,
  scale_factor: &MonitorScaleFactor,
) -> anyhow::Result<()> {
  let mut height = (crate::common::HITOKAGE_STATUSBAR_HEIGHT as f32 * scale_factor.y) as i32;

  if get_windows_version() < 11 {
    height = (crate::common::HITOKAGE_STATUSBAR_HEIGHT as f32 / scale_factor.y).round() as i32;
  }

  window.set_size_request(geometry.width, height);

  Ok(())
}

fn setup_window_pos(window: Window, geometry: &MonitorGeometry) -> anyhow::Result<()> {
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

#[derive(Debug)]
pub enum BarLuaHook {
  GetWidgets(Sender<Vec<WidgetUserData>>),
  GetGeometry(Sender<MonitorGeometry>),
}

#[derive(Debug)]
pub enum BarMsg {
  Destroy(Sender<()>),
  DestroyActual,
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
  pub widgets: Vec<WidgetProps>,
}

pub struct Bar {
  position: Option<BarPosition>,
  geometry: MonitorGeometry,
  pub widgets: Vec<WidgetController>,
  id: u32,
  index: usize,
  scale_factor: MonitorScaleFactor,
}

#[relm4::component(pub)]
impl SimpleComponent for Bar {
  type Input = BarMsg;
  type Output = ();
  type Init = (
    Monitor,
    BarProps,
    u32,
    Box<dyn Fn(relm4::Sender<BarMsg>) -> () + Send>,
    gtk::ApplicationWindow,
  );
  type Widgets = AppWidgets;

  view! {
    Window {
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
        // regardless of win version komorebi is consistent unlike gdk4
        let height = (crate::common::HITOKAGE_STATUSBAR_HEIGHT as f32 * &model.scale_factor.y) as i32;

        let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
          model.index,
          komorebi_client::Rect { left: 0, top: height, right: 0, bottom: height }
        ));
      }
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (monitor, props, id, callback, application_root) = input;

    root.set_transient_for(Some(&application_root));

    // root.connect_scale_factor_notify(move |win| {
    //   // todo @codyduong, needed for if users change scaling on the go
    // });

    callback(sender.clone().input_sender().clone());

    let mut model = Bar {
      position: props.position,
      geometry: monitor.geometry,
      widgets: Vec::new(),
      id: id, //hitokage id
      // windows id
      index: monitor.index,
      scale_factor: monitor.scale_factor,
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
          let controller = Workspace::builder().launch((inner_props, monitor.id as u32)).detach();
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
      BarMsg::LuaHook(hook) => match hook {
        BarLuaHook::GetWidgets(tx) => {
          tx.send(self.widgets.iter().map(|i| WidgetUserData::from(i)).collect())
            .unwrap();
        }
        BarLuaHook::GetGeometry(tx) => {
          tx.send(self.geometry).unwrap();
        }
      },
      BarMsg::Destroy(tx) => {
        let mut rxv: Vec<std::sync::mpsc::Receiver<()>> = vec![];

        for widget in &self.widgets {
          match widget {
            WidgetController::Clock(component) => {
              let sender = component.sender().clone();
              let (inner_tx, inner_rx) = channel::<()>();

              let _ = sender.send(ClockMsg::Destroy(inner_tx));

              rxv.push(inner_rx)
            }
            // @codyduong only clock needs a dedicated cleanup, since it has a timer
            WidgetController::Workspace(_component) => {
              // let _ = component.sender().send(WorkspaceMsg::Destroy);
            }
            WidgetController::Box(_component) => {
              // let _ = component.sender().send(BoxMsg::Destroy);
            }
          }
        }

        thread::spawn(move || {
          for rx in &rxv {
            // we can sometimes close the channel early so don't unwrap...
            let _ = rx.recv().unwrap();
          }
          let _ = tx.send(());
        });
      }
      BarMsg::DestroyActual => {
        log::error!("deprecated @codyduong")
      }
    }
  }
}
