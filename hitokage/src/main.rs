use core::fmt;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

use bar::Bar;
use chrono::Local;
use gtk4::gdk;
use gtk4::prelude::*;
use log::trace;
use relm4::prelude::*;
use relm4::SharedState;
use serde::{Deserialize, Serialize};
use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

mod bar;
mod luahelpers;
mod socket;
mod win_utils;

#[derive(Debug, Deserialize, Serialize)]
#[derive(Clone)]
pub struct PartialNotif {
  pub state: serde_json::Value,
  pub event: serde_json::Value,
}

// pub static STATE: SharedState<Option<komorebi_client::State>> = SharedState::new();
pub static STATE: SharedState<Vec<PartialNotif>> = SharedState::new();
pub static NEW_STATE: SharedState<bool> = SharedState::new(); // if the state has changed since we last read the state

#[derive(Debug)]
pub enum LuaHookType {
  SubscribeState, // subscribe to a value in global state
  WriteState,     //
  ReadState,      // This should probably exclusively be used for initializing configurations, it does not subscribe!
  CreateBar,
  NoAction, // These hooks are used for Relm4 hooking into, so it is very possible we don't need to handle anything
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
  pub callback: Box<dyn Fn(mlua::Value) -> Result<()> + Send + Sync>,
}

#[derive(Debug)]
pub enum Msg {
  Komorebi(String),
  KomorebiErr(String),
  LuaHook(LuaHook),
  Tick, // system clock
}

const HITOKAGE_STATUSBAR_HEIGHT: i32 = 64;

struct App {
  current_time: String,
  // state: Option<komorebi_client::State>,
  // bar: Controller<Bar>,
}

// struct AppInit {
//   lua: mlua::Lua,
// }

#[relm4::component]
impl SimpleComponent for App {
  type Input = Msg;
  type Output = ();
  type Init = ();
  // type Widgets = AppWidgets;

  // view! {
  //   gtk::ApplicationWindow {
  //     set_default_size: (0, 0),
  //     set_resizable: false,
  //     set_display: &gdk::Display::default().expect("Failed to get default display"),
  //     set_decorated: false,
  //     set_visible: false,

  //     // gtk::Box {
  //     //     set_orientation: gtk::Orientation::Vertical,
  //     //     gtk::Label {
  //     //         set_label: "Hello, World!",
  //     //     },

  //     //     gtk::Label {
  //     //         #[watch]
  //     //         set_label: &format!("{}", model.current_time),
  //     //     },

  //     //     gtk::Button {
  //     //         set_label: "Test",
  //     //         connect_clicked => move |window| {
  //     //             println!("foobar");
  //     //         }
  //     //     },
  //     // },
  //   }
  // }

  view! {
    gtk::ApplicationWindow {
      set_visible: false,

      // LOL! Is there a better way to prevent this from showing?
      connect_show => move |window| {
        window.set_visible(false)
      }
    },
  }

  fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    // start the lua hook
    let sender_clone = sender.clone();

    let lua_file_path = "./hitokage.lua";
    let mut file = File::open(lua_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lua_script = contents;

    let _lua_thread = std::thread::spawn(move || {
      let lua = hitokage_lua::make_lua().unwrap();
      let lua = luahelpers::augment(lua, sender_clone).unwrap();

      let co = lua
        .create_thread(lua.load(lua_script).into_function().unwrap())
        .unwrap();

      loop {
        match co.resume::<_, ()>(()) {
          Ok(_) => (),
          Err(err) => {
            println!("Lua error: {:?}", err);
            break;
          }
        }
        // std::thread::sleep(std::time::Duration::from_millis(100)); // Small sleep to prevent tight loop
      }
    });

    // komorebi pipe
    let sender_clone = sender.clone();
    socket::start_async_reader_new(sender_clone);

    // system clock
    let sender_clone = sender.clone();
    // "Precise timing is not guaranteed, the timeout may be delayed by other events."
    // so yeah, use 500ms increment, if we skip a second we have bigger problems performance wise...
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      sender_clone.input(Msg::Tick);
      glib::ControlFlow::Continue
    });

    // bar.widget().realize();
    // gtk4::prelude::WidgetExt::realize(bar.widget());

    let model = App {
      current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      // bar,
    };

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Msg, sender: ComponentSender<Self>) {
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

        if let Some(notif_value) = notif {
          // let mut result = {
          //   let rg = STATE.read();
          //   let result = rg.to_vec();
          //   drop(rg);
          //   result
          // };
          // result.push(notif_value);

          // let mut r = Vec::new();
          // r.push(notif_value);

          // *STATE.write() = r;
          let mut stwg = STATE.write();
          stwg.push(notif_value);
          drop(stwg);
          *NEW_STATE.write() = true;
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
        LuaHookType::CreateBar => {
          let app = relm4::main_application();
          let builder = Bar::builder();

          println!("bar created 2");

          // app.add_window(&builder.root);

          app.add_window(&builder.root);

          let bar = builder
            .launch(())
            .forward(sender.input_sender(), std::convert::identity);

          ()
        }
        LuaHookType::ReadState => {
          *crate::STATE.write() = Vec::new();
          *crate::NEW_STATE.write() = false;

          ()
        }
        LuaHookType::NoAction => (),
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

#[tokio::main]
async fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();

  let app = RelmApp::new("com.example.hitokage");
  app.run::<App>(());
}
