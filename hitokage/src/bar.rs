use crate::win_utils;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;

fn setup_window_size(window: ApplicationWindow) -> anyhow::Result<()> {
  let x = win_utils::get_primary_width();
  window.set_size_request(x, crate::HITOKAGE_STATUSBAR_HEIGHT);

  Ok(())
}

fn setup_window_pos(window: ApplicationWindow) -> anyhow::Result<()> {
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
      0,
      0,
      0,
      0,
      windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE,
    )
    .ok();
  }

  Ok(())
}

pub struct Bar {}

#[relm4::component(pub)]
impl SimpleComponent for Bar {
  type Input = ();
  type Output = crate::Msg;
  type Init = ();
  type Widgets = AppWidgets;

  view! {
    gtk::ApplicationWindow {
      set_default_size: (900, 64),
      set_resizable: false,
      set_display: &gdk4::Display::default().expect("Failed to get default display"),
      set_decorated: false,
      set_visible: false, // We can't instantiate before we have hooked our connect_* on, so this should always be false

      gtk::Box {
        set_orientation: gtk::Orientation::Vertical,
        gtk::Label {
          set_label: "Hello, World!",
        },

        // gtk::Label {
        //   #[watch]
        //   set_label: &format!("{}", model.current_time),
        // },

        gtk::Button {
          set_label: "Test",
          connect_clicked => move |window| {
            println!("foobar");
          }
        },
      },

      connect_realize => move |window| {
        let _ = setup_window_size(window.clone());
      },

      connect_show => move |window| {
        // Surfaces aren't built in realize, but they are ready for consumption here
        let _ = setup_window_pos(window.clone());
      }
    }
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model = Bar {};

    let widgets = view_output!();

    // manually realize/show
    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {}
}
