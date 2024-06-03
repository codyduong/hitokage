use core::fmt;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

use chrono::Local;
use gtk4::gdk;
use gtk4::prelude::*;
use log::trace;
use relm4::prelude::*;
use relm4::SharedState;
use serde::{Serialize, Deserialize};
use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

mod luahelpers;
mod pipes;
mod win_utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialNotif {
  pub state: serde_json::Value,
  pub event: serde_json::Value,
}

// pub static STATE: SharedState<Option<komorebi_client::State>> = SharedState::new();
pub static STATE: SharedState<Option<PartialNotif>> = SharedState::new();

struct OtherModel {}

#[relm4::component]
impl SimpleComponent for OtherModel {
  type Input = ();
  type Output = ();
  type Init = ();

  view! {
      gtk::ApplicationWindow {
          set_decorated: true,
          set_visible: true,

          gtk::Box {
              gtk::Label {
                  set_label: "Foo, Bar!",
              },
          },

      },
  }

  fn init(input: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model = OtherModel {};

    // Insert the code generation of the view! macro here
    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {}
}

#[derive(Debug)]
pub enum LuaHookType {
  SubscribeState, // subscribe to a value in global state
  WriteState,     //
  ReadState,      // This should probably exclusively be used for initializing configurations, it does not subscribe!
  CreateWidget,
}

impl fmt::Debug for LuaHook {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("LuaHook")
      .field("t", &self.t)
      .field("callback", &"<function>")
      .finish()
  }
}

pub struct LuaHook {
  pub t: LuaHookType,
  pub callback: Box<dyn Fn(mlua::Value) -> Result<()> + Send>,
}

#[derive(Debug)]
enum Msg {
  Komorebi(String),
  KomorebiErr(String),
  LuaHook(LuaHook),
  Tick, // system clock
}

const HITOKAGE_STATUSBAR_HEIGHT: i32 = 64;

struct AppState {
  current_time: String,
  // state: Option<komorebi_client::State>,
}

struct AppInit {
  lua: mlua::Lua,
}

#[relm4::component]
impl SimpleComponent for AppState {
  type Input = Msg;
  type Output = ();
  type Init = AppInit;
  type Widgets = AppWidgets;

  view! {
      gtk::ApplicationWindow {
          set_default_size: (0, 0),
          set_resizable: false,
          set_display: &gdk::Display::default().expect("Failed to get default display"),
          set_decorated: false,
          // set_visible: true,

          gtk::Box {
              set_orientation: gtk::Orientation::Vertical,
              gtk::Label {
                  set_label: "Hello, World!",
              },

              gtk::Label {
                  #[watch]
                  set_label: &format!("{}", model.current_time),
              },

              gtk::Button {
                  set_label: "Test",
                  connect_clicked => move |window| {
                      println!("foobar");
                  }
              },
          },

          connect_realize => move |window| {
              let x = win_utils::get_primary_width();
              window.set_size_request(x, HITOKAGE_STATUSBAR_HEIGHT);
          },

          // this fails in realize, for what reason i have no clue LOL! @codyduong
          connect_show => move |window| {
              // Set Status bar to TOP or BOTTOM of screen

              // https://discourse.gnome.org/t/set-absolut-window-position-in-gtk4/8552/4
              let native = window.native().expect("Failed to get native");
              let surface = native.surface().expect("Failed to get surface");

              // specifically for windows -> https://discourse.gnome.org/t/how-to-center-gtkwindows-in-gtk4/3112/13
              let handle = surface.downcast::<gdk4_win32::Win32Surface>().expect("Failed to get Win32Surface").handle();
              let win_handle = HWND(handle.0);

              println!("{:?}", win_handle);

              unsafe {
                  SetWindowPos(
                      win_handle,
                      HWND_TOPMOST,
                      0,
                      0,
                      0,
                      0,
                      SWP_NOSIZE,
                  )
                  .ok();
              }
          }
      }
  }

  fn init(init_params: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let model = AppState {
      current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      // state: None,
    };

    // Insert the code generation of the view! macro here
    let widgets = view_output!();

    // start the lua hook
    let sender_clone = sender.clone();
    let lua = init_params.lua;

    let lua = luahelpers::augment(lua, sender_clone).unwrap();

    let lua_file_path = "./hitokage.lua";
    let mut file = File::open(lua_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lua_script = contents;

    let lua_result = lua.load(lua_script).exec();

    println!("{:?}", lua_result);

    // komorebi pipe
    let sender_clone = sender.clone();
    pipes::start_async_reader(sender_clone);

    // system clock
    let sender_clone = sender.clone();
    // "Precise timing is not guaranteed, the timeout may be delayed by other events."
    // so yeah, use 500ms increment, if we skip a second we have bigger problems performance wise...
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      sender_clone.input(Msg::Tick);
      glib::ControlFlow::Continue
    });

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>) {
    match msg {
      // Komorebi States
      Msg::Komorebi(notif) => {
        // println!("{:?}", &notif);
        let notif: Option<PartialNotif> = serde_json::from_str(&notif).unwrap_or_else(|err| {
          log::warn!("Failed to read notification from komorebic: {:?}", err);

          None
        });

        // Breaks on version mismatch between built and available version...
        // let notif: Option<komorebi_client::Notification> = serde_json::from_str(&notif).unwrap_or_else(|err| {
        //   log::warn!("Failed to read notifcations from komorebic: {:?}", err);

        //   None
        // });

        if notif.is_some() {
          *STATE.write() = Some(notif.unwrap());
        }

        // println!("{:?}", &notif);
        // self.output = String::new();
        // self.output.push_str(&notif);
        // self.output.push('\n');
      }
      Msg::KomorebiErr(line) => {
        println!("{:?}", &line);
        // self.output = String::new();
        // self.output.push_str("ERROR!");
        // self.output.push_str(&line);
        // self.output.push('\n');
      }

      //
      Msg::LuaHook(info) => match info.t {
        LuaHookType::CreateWidget => {
          let app = relm4::main_application();
          let builder = OtherModel::builder();

          app.add_window(&builder.root);

          builder.launch(()).detach_runtime();

          ()
        }
        LuaHookType::ReadState => {
          println!("jeff");

          ()
        }
        _ => {
          println!("todo");

          ()
        }
      },

      // Primary Program Loop
      Msg::Tick => {
        self.current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      }
    }
  }
}

fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();
  let lua = hitokage_lua::make_lua().unwrap();

  let app = RelmApp::new("com.example.hitokage");
  app.run::<AppState>(AppInit { lua: lua });
}
