use gtk4::prelude::*;
use hitokage_core::lua::event::{EventNotif, CONFIG_UPDATE, STATE};
use hitokage_core::lua::event::{EVENT, NEW_EVENT};
use hitokage_core::widgets;
use hitokage_core::widgets::bar;
use hitokage_core::widgets::bar::BarMsg;
use hitokage_lua::AppMsg;
use hitokage_lua::LuaHookType;
use notify::Watcher;
use notify_debouncer_full::new_debouncer;
use relm4::component::Connector;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

mod config;
mod socket;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum LuaCoroutineMessage {
  Suspend,
  Reload,
}

struct App {
  bars: Vec<Connector<widgets::bar::Bar>>,
  file_last_checked_at: Arc<Mutex<Instant>>,
  // keep alive for lifetime of app
  _debouncer: notify_debouncer_full::Debouncer<notify::ReadDirectoryChangesWatcher, notify_debouncer_full::FileIdMap>,
  _tx_lua: Sender<bool>,
}

#[relm4::component(pub)]
impl Component for App {
  type Input = hitokage_lua::AppMsg;
  type Output = ();
  type Init = ();
  type CommandOutput = ();

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
    let file_last_checked_at = Arc::new(Mutex::new(Instant::now())); // when did we last check for a config update through any means?

    // these names suck, these are for sending to create a new luahandle for forceful termination
    let (tx_lua, rx_lua) = channel::<bool>();

    // if we need this for some reason, uhh good luck managing the arc mutex,
    // plus untangle all the other ones then...
    let _ = config::create_lua_handle(
      sender.clone(),
      lua_file_path.clone(),
      lua_thread_id.clone(),
      preventer_called.clone(),
      is_stopped.clone(),
      file_last_checked_at.clone(),
      tx_lua.clone(),
    );

    let _monitor_handle = config::create_watcher_handle(preventer_called.clone(), is_stopped.clone());

    let (tx, rx) = channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();
    debouncer
      .watcher()
      .watch(&lua_file_path, notify::RecursiveMode::NonRecursive)
      .unwrap();

    {
      let sender = sender.clone();
      let lua_file_path = lua_file_path.clone();
      let lua_thread_id = lua_thread_id.clone();
      let preventer_called = preventer_called.clone();
      let is_stopped = is_stopped.clone();
      let file_last_checked_at = file_last_checked_at.clone();
      let tx_lua = tx_lua.clone();

      let _filewatcher_handle = thread::spawn(move || loop {
        match rx.recv() {
          Ok(event) => match event {
            Ok(e) => {
              log::info!("File update: {:?}", e);

              let (tx, rx) = channel::<()>();
              // destroy all existing parts of the relm/gtk4 app
              sender.input(hitokage_lua::AppMsg::Destroy(tx));
              // wait for it to complete
              rx.recv().unwrap();
              log::debug!("We have cleaned up our messes, we can actually destroy the bar!");
              sender.input(hitokage_lua::AppMsg::DestroyActual);

              let mut stopped_guard = is_stopped.lock().unwrap();
              if !*stopped_guard {
                // if we are stuck in lua execution loop we need to dispatch a response to it for it to implode itself
                let mut wg = CONFIG_UPDATE.write();
                *wg = true;
                drop(wg);

                match rx_lua.recv() {
                  Ok(true) => {
                    log::info!("Lua thread reset itself within lua environment");

                    *stopped_guard = false;
                  }
                  Ok(false) => {
                    log::info!("Lua thread coroutine deadlocked, starting new lua thread");
                    let _ = config::create_lua_handle(
                      sender.clone(),
                      lua_file_path.clone(),
                      lua_thread_id.clone(),
                      preventer_called.clone(),
                      is_stopped.clone(),
                      file_last_checked_at.clone(),
                      tx_lua.clone(),
                    );

                    *stopped_guard = false;
                  }
                  Err(_) => {
                    // @codyduong LOL, todo
                    log::error!("Lua channel closed before receiving confirmation of launch of lua thread")
                  }
                }
              } else {
                log::info!("Lua thread finished executing, starting new lua thread");
                // otherwise we have finished execution, so dispatch a new thread
                let _ = config::create_lua_handle(
                  sender.clone(),
                  lua_file_path.clone(),
                  lua_thread_id.clone(),
                  preventer_called.clone(),
                  is_stopped.clone(),
                  file_last_checked_at.clone(),
                  tx_lua.clone(),
                );

                *stopped_guard = false;
              }
            }
            Err(e) => {
              log::error!("Watch error: {:?}", e);
            }
          },
          Err(e) => {
            log::error!("Receive error: {:?}", e);
          }
        };
        thread::sleep(Duration::from_millis(100));
      });
    }

    // komorebi pipe
    let sender_clone = sender.clone();
    socket::start(sender_clone);

    // bar.widget().realize();
    // gtk4::prelude::WidgetExt::realize(bar.widget());

    let model = App {
      bars: Vec::new(),
      file_last_checked_at,
      _debouncer: debouncer,
      _tx_lua: tx_lua,
      // bar,
    };

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>, root: &Self::Root) {
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
          let builder = bar::Bar::builder();

          let bar = builder.launch((monitor, props, id, callback, root.clone()));
          // .forward(sender.input_sender(), std::convert::identity);

          self.bars.push(bar);

          ()
        }
        LuaHookType::ReadEvent => {
          *EVENT.write() = Vec::new();
          *NEW_EVENT.write() = false;

          ()
        }
        LuaHookType::CheckConfigUpdate => {
          *self.file_last_checked_at.lock().unwrap() = Instant::now();
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
      }
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

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
    socket::shutdown().expect("Failed to shutdown komorebi socket");
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
