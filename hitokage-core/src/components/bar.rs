use super::app::AppMsg;
use super::base::BaseMsgHook;
use super::r#box::{BoxInner, BoxMsgHook, BoxProps};
use super::ChildUserData;
use crate::structs::{Monitor, MonitorGeometry, MonitorScaleFactor};
use crate::win_utils::get_windows_version;
use crate::{
  generate_base_match_arms, generate_box_children, generate_box_match_arms, prepend_css_class,
  prepend_css_class_to_model, set_initial_base_props, set_initial_box_props,
};
use bon::builder;
use gdk4_win32::windows::Win32::UI::WindowsAndMessaging::SIZE_MINIMIZED;
use gtk4::prelude::*;
use gtk4::Box as GtkBox;
use gtk4::Window;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Shell::{SHAppBarMessage, ABE_BOTTOM, ABE_TOP, ABM_NEW, ABM_REMOVE, ABM_SETPOS, APPBARDATA};
use windows::Win32::UI::WindowsAndMessaging::{
  CallWindowProcW, DefWindowProcW, SetWindowLongPtrW, SetWindowPos, ShowWindow, GWL_EXSTYLE, HWND_NOTOPMOST,
  SWP_NOSENDCHANGING, SWP_NOSIZE, SW_RESTORE, WM_SIZE, WS_EX_NOACTIVATE,
};

#[derive(Copy, Clone)]
struct HwndWrapper(HWND);

impl PartialEq for HwndWrapper {
  fn eq(&self, other: &Self) -> bool {
    self.0 .0 == other.0 .0
  }
}

impl Eq for HwndWrapper {}

impl Hash for HwndWrapper {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0 .0.hash(state);
  }
}

impl From<HWND> for HwndWrapper {
  fn from(value: HWND) -> Self {
    Self(value)
  }
}

type WndProcMap = Arc<Mutex<HashMap<HwndWrapper, unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>>>;

lazy_static::lazy_static! {
  static ref WND_PROC_MAP: WndProcMap = Arc::new(Mutex::new(HashMap::new()));
}

unsafe extern "system" fn new_wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  let wnd_proc_map = WND_PROC_MAP.clone();

  if msg == WM_SIZE && wparam.0 as u32 == SIZE_MINIMIZED {
    log::error!("Hitokage was minimized! Restoring hitokage");
    let _ = ShowWindow(hwnd, SW_RESTORE);
    return LRESULT(0);
  }

  let original_wnd_proc = wnd_proc_map.lock().unwrap().get(&hwnd.into()).copied();
  if let Some(proc) = original_wnd_proc {
    CallWindowProcW(Some(proc), hwnd, msg, wparam, lparam)
  } else {
    DefWindowProcW(hwnd, msg, wparam, lparam)
  }
}

#[builder]
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

fn get_hwnd(window: &Window) -> windows::Win32::Foundation::HWND {
  // https://discourse.gnome.org/t/set-absolut-window-position-in-gtk4/8552/4
  let native = window.native().expect("Failed to get native");
  let surface = native.surface().expect("Failed to get surface");

  // specifically for windows -> https://discourse.gnome.org/t/how-to-center-gtkwindows-in-gtk4/3112/13
  let handle = surface
    .downcast::<gdk4_win32::Win32Surface>()
    .expect("Failed to get Win32Surface")
    .handle();
  windows::Win32::Foundation::HWND(handle.0)
}

#[builder]
fn setup_window_surface(
  window: &Window,
  geometry: &MonitorGeometry,
  scale_factor: &MonitorScaleFactor,
  offset_x: i32,
  offset_y: i32,
  position: &BarPosition,
) -> anyhow::Result<()> {
  let height = ((geometry.height + offset_y) as f32) as i32;
  let width = ((geometry.width + offset_x) as f32) as i32;
  let u_edge = match position {
    BarPosition::Top => ABE_TOP,
    BarPosition::Bottom => ABE_BOTTOM,
  };

  let rc = windows::Win32::Foundation::RECT {
    left: geometry.x,
    right: geometry.x + width,
    top: geometry.y,
    bottom: geometry.y + height,
  };

  let hwnd = get_hwnd(window);
  let mut appbar_data = APPBARDATA {
    hWnd: hwnd,
    rc,
    uEdge: u_edge,
    ..Default::default()
  };

  unsafe {
    SHAppBarMessage(ABM_NEW, &mut appbar_data);
    SHAppBarMessage(ABM_SETPOS, &mut appbar_data);

    SetWindowPos(
      hwnd,
      HWND_NOTOPMOST,
      geometry.x,
      geometry.y,
      0,
      0,
      SWP_NOSIZE | SWP_NOSENDCHANGING,
    )?;
    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_NOACTIVATE.0 as _);
    // TODO @codyduong setup preventing movement via live-caption or other movements
    // let original_wnd_proc = std::mem::transmute(GetWindowLongPtrW(hwnd, GWLP_WNDPROC));
    // WND_PROC_MAP.lock().unwrap().insert(hwnd.into(), original_wnd_proc);
    // SetWindowLongPtrW(hwnd, GWLP_WNDPROC, new_wnd_proc as isize);
  }

  Ok(())
}

fn shutdown_window_surface(window: &Window) {
  let hwnd = get_hwnd(window);

  let mut appbar_data = APPBARDATA {
    hWnd: hwnd,
    ..Default::default()
  };

  unsafe { SHAppBarMessage(ABM_REMOVE, &mut appbar_data) };
}

#[derive(Debug)]
pub enum BarLuaHook {
  BoxHook(BoxMsgHook),
  GetGeometry(Sender<MonitorGeometry>),
}

#[derive(Debug)]
pub enum BarMsg {
  LuaHook(BarLuaHook),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
pub enum BarPosition {
  Top,
  Bottom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarOffset {
  pub x: Option<i32>,
  pub y: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BarProps {
  pub position: Option<BarPosition>,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub offset: Option<BarOffset>,
  #[serde(flatten)]
  pub r#box: BoxProps,
}

pub struct Bar {
  position: BarPosition,
  geometry: MonitorGeometry,
  index: usize,
  scale_factor: MonitorScaleFactor,
  offset_x: i32,
  offset_y: i32,
  r#box: BoxInner,
  bars_destroyed_condvar: Arc<(Mutex<usize>, std::sync::Condvar)>,
}

#[relm4::component(pub)]
impl Component for Bar {
  type Input = BarMsg;
  type Output = AppMsg;
  type Init = (
    Monitor,
    BarProps,
    Box<dyn Fn(relm4::Sender<BarMsg>) + Send>,
    gtk::ApplicationWindow,
    Arc<(Mutex<usize>, std::sync::Condvar)>,
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
        let _ = setup_window_size()
          .window(window.clone())
          .geometry(&model.geometry)
          .scale_factor(&model.scale_factor)
          .call();
      },

      connect_show => move |window| {
        // Surfaces aren't ready in realize, but they are ready for consumption here
        let _ = setup_window_surface()
          .window(window)
          .geometry(&model.geometry)
          .scale_factor( &model.scale_factor)
          .offset_x(model.offset_x)
          .offset_y(model.offset_y)
          .position(&model.position)
          .call();
        // regardless of win version komorebi is consistent unlike gdk4
        // let height = ((model.geometry.height + model.offset_y) as f32 * model.scale_factor.y) as i32;

        // println!("{:?} {:?}", (model.geometry.height + &model.offset_y), height);

        // if model.position.is_some_and(|pos| pos == BarPosition::Bottom) {
        //   let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
        //     model.index,
        //     komorebi_client::Rect { left: 0, top: 0, right: 0, bottom: height }
        //   ));
        // } else {
        //   let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
        //     model.index,
        //     komorebi_client::Rect { left: 0, top: height, right: 0, bottom: height }
        //   ));
        // }
      },

      connect_unrealize[bars_destroyed_condvar = Arc::clone(&model.bars_destroyed_condvar)] => move |window| {
        // let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::MonitorWorkAreaOffset(
        //   model.index,
        //   komorebi_client::Rect {
        //     left: 0,
        //     top: 0,
        //     right: 0,
        //     bottom: 0,
        //   },
        // ));
        shutdown_window_surface(window);
        let (lock, cvar) = &*bars_destroyed_condvar;
        let mut bars_destroyed = lock.lock().unwrap();
        *bars_destroyed += 1;
        cvar.notify_one();
      },
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (monitor, props, callback, application_root, bars_destroyed_condvar) = input;

    callback(sender.clone().input_sender().clone());

    let mut geometry = monitor.geometry;
    let position = props.position.unwrap_or(BarPosition::Top);

    let mut offset_x = 0;
    let mut offset_y = 0;

    geometry.width = props.width.unwrap_or(geometry.width);
    geometry.height = props.height.unwrap_or(crate::common::HITOKAGE_STATUSBAR_HEIGHT);

    match position {
      BarPosition::Top => (),
      BarPosition::Bottom => geometry.y += monitor.geometry.height - geometry.height,
    };

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
      position,
      geometry,
      index: monitor.index,
      scale_factor: monitor.scale_factor,
      offset_x,
      offset_y,
      r#box: BoxInner {
        homogeneous: props.r#box.homogeneous,
        children: Vec::new(),
        base: props.r#box.base.clone().into(),
      },
      bars_destroyed_condvar,
    };

    root.set_transient_for(Some(&application_root));
    prepend_css_class_to_model!("bar", model.r#box, root);
    let widgets = view_output!();
    set_initial_box_props!(model, widgets.main_box, props.r#box.base);
    generate_box_children!(
      props.r#box.children,
      model.r#box,
      monitor,
      widgets.main_box,
      sender.output_sender()
    );

    // manually realize/show
    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      BarMsg::LuaHook(hook) => match hook {
        // BarLuaHook::BaseHook(base) => {
        //   // TODO @codyduong... this sucks... LOL! the `view_output!();` macro modifies whats available
        //   generate_base_match_arms!(
        //     self,
        //     "bar",
        //     root.child().unwrap().downcast::<GtkBox>().unwrap(),
        //     BaseMsgHook,
        //     base
        //   )
        // }
        BarLuaHook::BoxHook(hook) => {
          let r2 = root.child().unwrap().downcast::<GtkBox>().unwrap();
          generate_box_match_arms!(self, "bar", r2, BoxMsgHook, hook)
        }
        BarLuaHook::GetGeometry(tx) => {
          tx.send(self.geometry).unwrap();
        }
      },
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {}
}
