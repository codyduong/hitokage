use super::base::{Base, BaseMsgHook, BaseProps};
use super::WidgetUserData;
use super::{WidgetController, WidgetProps};
use crate::structs::{Monitor, MonitorGeometry, MonitorScaleFactor};
use crate::widgets::clock::Clock;
use crate::widgets::workspace::Workspace;
use crate::win_utils::get_windows_version;
use crate::{generate_base_match_arms, prepend_css_class, prepend_css_class_to_model, set_initial_base_props};
use gtk4::prelude::*;
use gtk4::Box as GtkBox;
use gtk4::Window;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, HWND_TOP, SWP_NOSIZE};

fn setup_window_size(
  window: Window,
  geometry: &MonitorGeometry,
  scale_factor: &MonitorScaleFactor,
) -> anyhow::Result<()> {
  let mut width = geometry.width;
  let mut height = geometry.height;

  if get_windows_version() > 10 {
    width *= scale_factor.x as i32;
    height *= scale_factor.y as i32;
  }

  window.set_size_request(width, height);

  Ok(())
}

fn setup_window_surface(window: Window, geometry: &MonitorGeometry) -> anyhow::Result<()> {
  // https://discourse.gnome.org/t/set-absolut-window-position-in-gtk4/8552/4
  let native = window.native().expect("Failed to get native");
  let surface = native.surface().expect("Failed to get surface");

  // specifically for windows -> https://discourse.gnome.org/t/how-to-center-gtkwindows-in-gtk4/3112/13
  let handle = surface
    .downcast::<gdk4_win32::Win32Surface>()
    .expect("Failed to get Win32Surface")
    .handle();
  let win_handle = windows::Win32::Foundation::HWND(handle.0);

  unsafe {
    SetWindowPos(
      win_handle, // TODO @codyduong, set this up for user configuration
      HWND_TOP, geometry.x, geometry.y, 0, 0, SWP_NOSIZE,
    )
    .ok();
  }

  Ok(())
}

#[derive(Debug)]
pub enum BarLuaHook {
  BaseHook(BaseMsgHook),
  GetGeometry(Sender<MonitorGeometry>),
  GetWidgets(Sender<Vec<WidgetUserData>>),
}

#[derive(Debug)]
pub enum BarMsg {
  LuaHook(BarLuaHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BarPosition {
  Top,
  Bottom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarOffset {
  pub x: Option<i32>,
  pub y: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarProps {
  pub position: Option<BarPosition>,
  pub widgets: Vec<WidgetProps>,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub offset: Option<BarOffset>,
  #[serde(flatten)]
  base: BaseProps,
}

pub struct Bar {
  position: Option<BarPosition>,
  geometry: MonitorGeometry,
  widgets: Vec<WidgetController>,
  index: usize,
  scale_factor: MonitorScaleFactor,
  offset_x: i32,
  offset_y: i32,
  base: Base,
}

#[relm4::component(pub)]
impl Component for Bar {
  type Input = BarMsg;
  type Output = ();
  type Init = (
    Monitor,
    BarProps,
    Box<dyn Fn(relm4::Sender<BarMsg>) + Send>,
    gtk::ApplicationWindow,
  );
  type Widgets = AppWidgets;
  type CommandOutput = ();

  view! {
    Window {
      set_default_size: (500, crate::common::HITOKAGE_STATUSBAR_HEIGHT),
      set_resizable: false,
      set_display: &gdk4::Display::default().expect("Failed to get default display"),
      set_decorated: false,
      set_visible: false, // We can't instantiate before we have hooked our connect_* on, so this should always be false

      #[name = "main_box"]
      gtk::Box {
        set_orientation: gtk::Orientation::Horizontal,
      },

      connect_realize => move |window| {
        let _ = setup_window_size(window.clone(), &model.geometry, &model.scale_factor);
      },

      connect_show => move |window| {
        // Surfaces aren't ready in realize, but they are ready for consumption here
        let _ = setup_window_surface(window.clone(), &model.geometry);
        // regardless of win version komorebi is consistent unlike gdk4
        let height = ((model.geometry.height + model.offset_y) as f32 * model.scale_factor.y) as i32;

        // println!("{:?} {:?}", (model.geometry.height + &model.offset_y), height);

        let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
          model.index,
          komorebi_client::Rect { left: 0, top: height, right: 0, bottom: height }
        ));
      }
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (monitor, props, callback, application_root) = input;

    callback(sender.clone().input_sender().clone());

    let mut geometry = monitor.geometry;
    let mut offset_x = 0;
    let mut offset_y = 0;

    // by default use 32
    // TODO @codyduong, this is assuming a horizontal bar, otherwise change defaults
    geometry.height = crate::common::HITOKAGE_STATUSBAR_HEIGHT;

    geometry.width = props.width.unwrap_or(geometry.width);
    geometry.height = props.height.unwrap_or(geometry.height);

    if let Some(offset) = props.offset {
      if let Some(x) = offset.x {
        geometry.x += x;
        offset_x = x;
      }
      if let Some(y) = offset.y {
        geometry.y += y;
        offset_y = y;
      }
    }

    let mut model = Bar {
      position: props.position,
      geometry,
      widgets: Vec::new(),
      index: monitor.index,
      scale_factor: monitor.scale_factor,
      offset_x,
      offset_y,
      base: Base {
        classes: props.base.class.unwrap_or_default().into(),
        halign: props.base.halign,
        hexpand: props.base.hexpand,
        homogeneous: props.base.homogeneous.or(Some(true)),
        valign: props.base.valign,
        vexpand: props.base.vexpand,
      },
    };

    prepend_css_class_to_model!("bar", model, root);
    root.set_transient_for(Some(&application_root));

    // root.connect_scale_factor_notify(move |win| {
    //   // todo @codyduong, needed for if users change scaling on the go
    // });

    let widgets = view_output!();
    set_initial_base_props!(model, widgets.main_box);

    for widget in props.widgets {
      let monitor = monitor.clone();
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
          let controller = crate::widgets::r#box::Box::builder()
            .launch((monitor, inner_props))
            .detach();
          widgets.main_box.append(controller.widget());
          model.widgets.push(WidgetController::Box(controller));
        }
      }
    }

    // manually realize/show
    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      BarMsg::LuaHook(hook) => match hook {
        BarLuaHook::BaseHook(base) => {
          // TODO @codyduong... this sucks... LOL! the `view_output!();` macro modifies whats available
          generate_base_match_arms!(
            self,
            "bar",
            root.child().unwrap().downcast::<GtkBox>().unwrap(),
            BaseMsgHook,
            base
          )
        }
        BarLuaHook::GetGeometry(tx) => {
          tx.send(self.geometry).unwrap();
        }
        BarLuaHook::GetWidgets(tx) => {
          tx.send(self.widgets.iter().map(WidgetUserData::from).collect())
            .unwrap();
        }
      },
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _outputt: relm4::Sender<Self::Output>) {
    let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
      self.index,
      komorebi_client::Rect {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
      },
    ));
  }
}
