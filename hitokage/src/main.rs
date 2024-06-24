use glib::translate::Borrowed;
use gtk4::prelude::*;
use hitokage_core::lua::event::{EventNotif, CONFIG_UPDATE, STATE};
use hitokage_core::lua::event::{EVENT, NEW_EVENT};
use hitokage_core::widgets::bar::BarMsg;
use hitokage_core::widgets::clock::ClockMsg;
use hitokage_core::widgets::{bar, WidgetController};
use hitokage_core::{widgets, win_utils};
use hitokage_lua::AppMsg;
use hitokage_lua::LuaHookType;
use mlua::LuaSerdeExt;
use notify::{RecommendedWatcher, Watcher};
use relm4::component::Connector;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};
// use windows::Win32::Foundation::CloseHandle;
// use windows::Win32::Foundation::HANDLE;
// use windows::Win32::System::Threading::{OpenThread, TerminateThread, THREAD_TERMINATE};
mod config;
mod socket;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum LuaCoroutineMessage {
  Suspend,
  Reload,
}

struct App {
  bars: Vec<Connector<widgets::bar::Bar>>,
  // keep alive for lifetime of app
  _filewatcher: notify::ReadDirectoryChangesWatcher,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
  type Input = hitokage_lua::AppMsg;
  type Output = ();
  type Init = ();

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

    let lua_file_path = if cfg!(feature = "development") {
      let mut path = Path::new(file!()).to_path_buf();
      path.push("../../../example/init.lua");
      fs::canonicalize(path).expect("Failed to canonicalize path")
    } else {
      let mut path = dirs::home_dir().expect("Could not find home directory");
      path.push(".config/hitokage/init.lua");
      path
    };

    let preventer_called = Arc::new(Mutex::new(false));
    let is_stopped = Arc::new(Mutex::new(false));
    let lua_thread_id = Arc::new(Mutex::new(0));
    let last_processed = Arc::new(Mutex::new(None));

    let mut lua_handle = config::create_lua_handle(
      sender.clone(),
      lua_file_path.clone(),
      lua_thread_id.clone(),
      preventer_called.clone(),
      is_stopped.clone(),
    );

    let _monitor_handle =
      config::create_watcher_handle(lua_thread_id.clone(), preventer_called.clone(), is_stopped.clone());

    let (tx, rx) = channel();

    let config = notify::Config::default()
      .with_poll_interval(Duration::from_millis(500))
      .with_compare_contents(true);

    let mut filewatcher: RecommendedWatcher = Watcher::new(tx.clone(), config).expect("Failed to start file watcher");
    filewatcher
      .watch(&lua_file_path, notify::RecursiveMode::NonRecursive)
      .expect("Failed to start file watcher 2");

    let sender_clone = sender.clone();
    let _filewatcher_handle = thread::spawn(move || loop {
      match rx.recv() {
        Ok(event) => match event {
          Ok(e) => {
            let mut last = last_processed.lock().unwrap();
            let now = Instant::now();
            // there are multiple file save events... so ignore the next few if we already started one in the last 250ms
            if let Some(last_time) = *last {
              if now.duration_since(last_time) < Duration::from_millis(250) {
                log::warn!("File update skipped: {:?}", e);
                continue;
              }
            }
            log::info!("File update: {:?}", e);
            *last = Some(now);

            let (tx, rx) = channel::<()>();
            // destroy all existing parts of the relm/gtk4 app
            sender_clone.input(hitokage_lua::AppMsg::Destroy(tx));
            // wait for it to complete
            rx.recv().unwrap();
            log::debug!("We have cleaned up our messes, we can actually destroy the bar!");
            sender_clone.input(hitokage_lua::AppMsg::DestroyActual);

            let is_stopped_value = *is_stopped.lock().unwrap();
            if !is_stopped_value {
              // if we are stuck in lua execution loop we need to dispatch a response to it for it to implode itself
              *CONFIG_UPDATE.write() = true;
            } else {
              // otherwise we have finished execution, so dispatch a new thread
              lua_handle = config::create_lua_handle(
                sender_clone.clone(),
                lua_file_path.clone(),
                lua_thread_id.clone(),
                preventer_called.clone(),
                is_stopped.clone(),
              );
            }
          }
          Err(e) => {
            log::error!("Watch error: {:?}", e);
          }
        },
        Err(e) => {
          log::error!("Receive error: {:?}", e);
        }
      }
    });

    // komorebi pipe
    let sender_clone = sender.clone();
    socket::start_async_reader_new(sender_clone);

    // bar.widget().realize();
    // gtk4::prelude::WidgetExt::realize(bar.widget());

    let model = App {
      bars: Vec::new(),
      _filewatcher: filewatcher,
      // bar,
    };

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
    match msg {
      // Komorebi States
      AppMsg::Komorebi(notif) => {
        // println!("{:?}", &notif);
        let notif: Option<EventNotif> = serde_json::from_str(&notif).unwrap_or_else(|err| {
          log::warn!("Failed to read notification from komorebic: {:?}", err);

          None
        });

        // Breaks on version mismatch between built and available version...
        // let notif: Option<komorebi_client::Notification> = serde_json::from_str(&notif).unwrap_or_else(|err| {
        //   log::warn!("Failed to read notifcations from komorebic: {:?}", err);

        //   None
        // });

        if let Some(notif_value) = notif {
          let mut sswg = EVENT.write();
          sswg.push(notif_value.clone());
          drop(sswg);
          *STATE.write() = notif_value.state;
          *NEW_EVENT.write() = true;
        }
      }
      AppMsg::KomorebiErr(line) => {
        println!("{:?}", &line);
      }

      AppMsg::LuaHook(info) => match info.t {
        LuaHookType::CreateBar(monitor, props, id, callback) => {
          let app = relm4::main_application();
          let builder = bar::Bar::builder();

          app.add_window(&builder.root);

          let bar = builder.launch((monitor, props, id, callback));
          // .forward(sender.input_sender(), std::convert::identity);

          self.bars.push(bar);

          ()
        }
        LuaHookType::ReadEvent => {
          *EVENT.write() = Vec::new();
          *NEW_EVENT.write() = false;

          ()
        }
        LuaHookType::NoAction => (),
        _ => {
          // @codyduong TODO
          log::warn!("todo");

          ()
        }
      },
      AppMsg::Destroy(tx) => {
        // this has to be refactored because there is no way this is the right way to do it
        let mut rxv: Vec<std::sync::mpsc::Receiver<()>> = vec![];

        for bar in self.bars.iter() {
          let (inner_tx, inner_rx) = channel::<()>();
          let _ = bar.sender().send(BarMsg::Destroy(inner_tx));
          rxv.push(inner_rx);
        }

        thread::spawn(move || {
          for rx in &rxv {
            let _ = rx.recv().unwrap();
          }
          let _ = tx.send(());
        });
      },
      AppMsg::DestroyActual => {
        // we can't prematurely drop our controllers our we panic!, so wait until we have cleaned up everything we need to
        for mut bar in self.bars.drain(..) {
          let _ = bar.sender().send(BarMsg::DestroyActual);
          bar.widget().destroy();
          // explode!
          bar.detach_runtime();
          drop(bar);
        }
      }
    }
  }
}

fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();

  unsafe {
    SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).expect("Failed to set process DPI awareness");
  }

  let style_file_path = "./example/styles.css";

  gtk::init().unwrap();
  if let Some(settings) = gtk::Settings::default() {
    // TODO @codyduong we need a primer/FAQ on blurry text
    settings.set_property("gtk-xft-antialias", &0);
    settings.set_property("gtk-xft-hinting", &1);
    settings.set_property("gtk-xft-hintstyle", &"hintfull");
    settings.set_property("gtk-xft-rgba", &"rgb");
  }

  let app = RelmApp::new("com.example.hitokage");
  let _ = app.set_global_css_from_file(style_file_path);
  app.run::<App>(());
}
