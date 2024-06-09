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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum BarPosition {
  Top,
  Bottom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarProps {
  pub position: Option<BarPosition>,
  pub geometry: Option<MonitorGeometry>,
  pub widgets: Vec<super::Widgets>,
}

#[derive(Debug)]
pub enum BarMsg {
  Tick,
}

pub struct BarLuaShared<'a, C>
where
  C: Component<Input = BarMsg>,
  <C as Component>::Output: std::marker::Send,
{
  sender: &'a ComponentSender<C>,
}

pub static BAR: SharedState<Vec<BarLuaShared<Bar>>> = SharedState::new();

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

  // Set to the correct display according to props
  // let displays = displays.iter().find(|d| d.name() == '');
  // let monitors = gdk4::DisplayManager::get().default_display().expect("").monitors();
  // let monitors_vec: Vec<gdk4_win32::Win32Monitor> = monitors
  //   .iter()
  //   .filter_map(|result| {
  //     result
  //       .ok()
  //       .and_then(|item: glib::Object| item.downcast::<gdk4_win32::Win32Monitor>().ok())
  //   })
  //   .collect();
  // println!("{:?}", monitors_vec[0].model());
  // window.set_display();

  Ok(())
}

pub struct Bar {
  position: Option<BarPosition>,
  geometry: MonitorGeometry,
  current_time: String,
  widgets: Vec<crate::widgets::Widgets>,
}

#[relm4::component(pub)]
impl SimpleComponent for Bar {
  type Input = BarMsg;
  type Output = ();
  type Init = BarProps;
  type Widgets = AppWidgets;

  view! {
    gtk::ApplicationWindow {
      set_default_size: (1920, crate::common::HITOKAGE_STATUSBAR_HEIGHT),
      set_resizable: false,
      set_display: &gdk4::Display::default().expect("Failed to get default display"),
      set_decorated: false,
      set_visible: false, // We can't instantiate before we have hooked our connect_* on, so this should always be false

      gtk::Box {
        set_orientation: gtk::Orientation::Horizontal,
        set_hexpand: true,
        set_vexpand: true,
        set_homogeneous: true,

        gtk::Label {
          set_hexpand: true,
          set_label: "Hello, World!",
        },

        gtk::Label {
          set_hexpand: true,
          #[watch]
          set_label: &format!("{}", model.current_time),
        },

        // placeholder label
        gtk::Label {}
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
    let model = Bar {
      current_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      position: input.position,
      geometry: input.geometry.unwrap_or(MonitorGeometry {
        x: 0,
        y: 0,
        width: win_utils::get_primary_width(),
        height: win_utils::get_primary_height(),
      }),
      widgets: input.widgets,
    };

    // Timer
    let sender_clone = sender.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      sender_clone.input(BarMsg::Tick);
      glib::ControlFlow::Continue
    });

    let widgets = view_output!();

    // manually realize/show
    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      BarMsg::Tick => {
        self.current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      }
    }
  }
}
