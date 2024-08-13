use config::reload_css_provider;
use gtk4::prelude::*;
use hitokage_core::event::{CONFIG_UPDATE, STATE};
use hitokage_core::event::{EVENT, NEW_EVENT};
use hitokage_core::{get_hitokage_asset, widgets};
use hitokage_core::widgets::bar;
use hitokage_lua::AppMsg;
use hitokage_lua::LuaHookType;
use log::LevelFilter;
use notify::Watcher;
use notify_debouncer_full::new_debouncer;
use relm4::component::Connector;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs::{self};
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TOOLWINDOW};

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
  _css_debouncer:
    notify_debouncer_full::Debouncer<notify::ReadDirectoryChangesWatcher, notify_debouncer_full::FileIdMap>,
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
        window.set_visible(false);

        let native = window.native().expect("Failed to get native");
        let surface = native.surface().expect("Failed to get surface");

        let handle = surface
          .downcast::<gdk4_win32::Win32Surface>()
          .expect("Failed to get Win32Surface")
          .handle();
        let win_handle = windows::Win32::Foundation::HWND(handle.0);

        unsafe {
          let ex_style = GetWindowLongPtrW(win_handle, GWL_EXSTYLE) as u32;
          let new_ex_style = ex_style | WS_EX_TOOLWINDOW.0;
          SetWindowLongPtrW(win_handle, GWL_EXSTYLE, new_ex_style as _);
        }
      }
    },
  }

  fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    // start the lua hook

    let lua_file_path = get_hitokage_asset("init.lua");
    let css_file_path = get_hitokage_asset("styles.css");

    log::info!("attempting to load lua init.lua at: {}", lua_file_path.display());
    log::info!("attempting to load lua styles.css at: {}", css_file_path.display());

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

    let (css_watcher_tx, css_watcher_rx) = channel();

    let mut css_debouncer = new_debouncer(Duration::from_secs(1), None, css_watcher_tx).unwrap();
    css_debouncer
      .watcher()
      .watch(&css_file_path, notify::RecursiveMode::NonRecursive)
      .unwrap();

    let old_provider = Rc::new(RefCell::new(gtk4::CssProvider::new()));

    {
      let root = root.clone();
      let css_file_path = css_file_path.clone();
      let old_provider = Rc::clone(&old_provider);

      glib::source::timeout_add_local_full(Duration::from_millis(50), glib::Priority::DEFAULT_IDLE, move || {
        match css_watcher_rx.try_recv() {
          Ok(result) => match result {
            Ok(_) => {
              let mut old_provider = old_provider.borrow_mut();
              log::info!("Reloading styles.css");
              *old_provider = reload_css_provider(&root, &css_file_path, &old_provider);
              glib::ControlFlow::Continue
            }
            Err(error) => {
              log::error!("Failed to reload css provider, errors: {:?}", error);
              glib::ControlFlow::Break
            }
          },
          Err(error) => match error {
            std::sync::mpsc::TryRecvError::Empty => glib::ControlFlow::Continue,
            std::sync::mpsc::TryRecvError::Disconnected => {
              log::error!("Css watcher dropped");
              glib::ControlFlow::Break
            }
          },
        }
      });
    }

    // load initial css
    {
      let mut old_provider = old_provider.borrow_mut();
      log::info!("Loading styles.css");
      *old_provider = reload_css_provider(&root, &css_file_path, &old_provider);
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
      _css_debouncer: css_debouncer,
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
        // Breaks on version mismatch between built and available version...
        // let notif: Option<komorebi_client::Notification> = serde_json::from_str(&notif).unwrap_or_else(|err| {
        //   log::warn!("Failed to read notifcations from komorebic: {:?}", err);

        //   None
        // });

        // let mut sswg = EVENT.write();
        // sswg.push(notif.clone());
        // drop(sswg);
        *STATE.write() = notif.state;
        *NEW_EVENT.write() = true;
      }
      AppMsg::KomorebiErr(line) => {
        println!("{:?}", &line);
      }

      AppMsg::LuaHook(info) => match info.t {
        LuaHookType::CreateBar(monitor, props, callback) => {
          let builder = bar::Bar::builder();

          let bar = builder.launch((*monitor, props, callback, root.clone()));
          // .forward(sender.input_sender(), std::convert::identity);

          self.bars.push(bar);
        }
        LuaHookType::ReadEvent => {
          *EVENT.write() = Vec::new();
          *NEW_EVENT.write() = false;
        }
        LuaHookType::CheckConfigUpdate => {
          *self.file_last_checked_at.lock().unwrap() = Instant::now();
        }
        LuaHookType::NoAction => (),
        _ => {
          // @codyduong TODO
          log::warn!("todo");
        }
      },
      AppMsg::DestroyActual => {
        // we can't prematurely drop our controllers our we panic!, so wait until we have cleaned up everything we need to
        for mut bar in self.bars.drain(..) {
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
  simple_logger::SimpleLogger::new()
    .with_module_level("handlebars", LevelFilter::Warn)
    .init()
    .unwrap();

  unsafe {
    SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).expect("Failed to set process DPI awareness");
  }

  // let style_file_path = "./example/styles.css";

  gtk::init().unwrap();
  if let Some(settings) = gtk::Settings::default() {
    // TODO @codyduong we need a primer/FAQ on blurry text
    settings.set_property("gtk-xft-antialias", 0);
    settings.set_property("gtk-xft-hinting", 1);
    settings.set_property("gtk-xft-hintstyle", "hintfull");
    settings.set_property("gtk-xft-rgba", "rgb");
    settings.set_property("gtk-xft-dpi", 300);
  }

  let app = RelmApp::new("com.example.hitokage");
  // let _ = app.set_global_css_from_file(style_file_path);
  app.run::<App>(());
}
