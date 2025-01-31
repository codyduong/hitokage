use config::{reload_css_provider, LuaRuntime};
use gtk4::prelude::*;
use hitokage_core::components;
use hitokage_core::components::app::{AppMsg, LuaHookType};
use hitokage_core::components::bar;
use hitokage_core::components::weather::WeatherStation;
use hitokage_core::event::{CONFIG_UPDATE, EVENT, LUA_ACTION_REQUESTS, NEW_EVENT, STATE};
use hitokage_core::get_hitokage_asset;
use hitokage_core::structs::lua_action::LuaActionRequest;
use hitokage_core::structs::system::SystemWrapper;
use log::LevelFilter;
use notify::Watcher;
use notify_debouncer_full::new_debouncer;
use relm4::component::Controller;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Condvar};
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
  // lua: mlua::Lua, // TODO @codyduong, reevaluate after mlua0.10.0 is stable
  bars: Vec<Controller<components::bar::Bar>>,
  file_last_checked_at: Arc<Mutex<Instant>>,
  // so we only keep one weather station to request forecasts (todo @codyduong support multiple weather stations)
  weather_station: Arc<Mutex<Option<WeatherStation>>>,
  weather_station_count: Arc<AtomicUsize>,
  system: SystemWrapper,
  is_destroyed_condvar: Arc<(Mutex<bool>, Condvar)>,
  bars_destroyed_condvar: Arc<(Mutex<usize>, Condvar)>, // the number of bars that have destroyed

  // keep alive for lifetime of app
  _debouncer: notify_debouncer_full::Debouncer<notify::ReadDirectoryChangesWatcher, notify_debouncer_full::FileIdMap>,
  _css_debouncer:
    notify_debouncer_full::Debouncer<notify::ReadDirectoryChangesWatcher, notify_debouncer_full::FileIdMap>,
  _tx_lua: Sender<bool>,
}

struct AppInit {
  lua: mlua::Lua,
  is_stopped: Arc<AtomicBool>,
}

#[relm4::component(pub)]
impl Component for App {
  type Input = AppMsg;
  type Output = ();
  type Init = AppInit;
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

  fn init(init: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    // start the lua hook
    let lua = init.lua;
    let is_stopped = init.is_stopped;

    let lua_file_path = get_hitokage_asset("init.lua");
    let css_file_path = get_hitokage_asset("styles.css");

    log::info!("attempting to load lua init.lua at: {}", lua_file_path.display());
    log::info!("attempting to load lua styles.css at: {}", css_file_path.display());

    let preventer_called = Arc::new(AtomicBool::new(false));
    let lua_thread_id = Arc::new(AtomicU32::new(0));
    let file_last_checked_at = Arc::new(Mutex::new(Instant::now())); // when did we last check for a config update through any means?

    // these names suck, these are for sending to create a new luahandle for forceful termination
    let (tx_lua, rx_lua) = channel::<bool>();

    // if we need this for some reason, uhh good luck managing the arc mutex,
    // plus untangle all the other ones then...
    let runtime = LuaRuntime::new(
      lua.clone(),
      sender.clone(),
      lua_file_path.clone(),
      lua_thread_id.clone(),
      preventer_called.clone(),
      is_stopped.clone(),
      file_last_checked_at.clone(),
      tx_lua.clone(),
    );
    runtime.clone().start_runtime();

    let _monitor_handle = config::create_watcher_handle(
      preventer_called.clone(),
      is_stopped.clone(),
      sender.input_sender().clone(),
    );

    let (tx, rx) = channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), Some(Duration::from_millis(500)), tx).unwrap();
    debouncer
      .watcher()
      .watch(&lua_file_path, notify::RecursiveMode::NonRecursive)
      .unwrap();

    let is_destroyed_condvar = Arc::new((Mutex::new(false), Condvar::new()));
    let is_destroyed_condvar_2 = Arc::clone(&is_destroyed_condvar);

    {
      let sender = sender.clone();

      let _filewatcher_handle = thread::spawn(move || {
        loop {
          match rx.recv() {
            Ok(event) => match event {
              Ok(e) => {
                log::info!("File update: {:?}", e);

                sender.input(AppMsg::DestroyActual);

                let (lock, cvar) = &*is_destroyed_condvar_2;
                let mut destroyed = lock.lock().unwrap();
                while !*destroyed {
                  destroyed = cvar.wait(destroyed).unwrap();
                  thread::sleep(Duration::from_millis(100));
                }
                // SHAppBarMessage has no synchronization(? factcheck TODO) i could find, so just wait 500ms after we
                // call it and hope the system was fast enough
                std::thread::sleep(Duration::from_millis(2000));
                *destroyed = false;

                if !is_stopped.load(Ordering::SeqCst) {
                  // if we are stuck in lua execution loop we need to dispatch a response to it for it to implode itself
                  match CONFIG_UPDATE.try_write() {
                      Ok(mut wg) => {  
                        *wg = true;
                        drop(wg); 
                      },
                      Err(e) => {
                        log::warn!("{}", e);
                        runtime.clone().start_runtime();
                      },
                  };

                  match rx_lua.recv() {
                    Ok(true) => {
                      log::info!("Lua thread reset itself within lua environment");
                    }
                    Ok(false) => {
                      log::info!("Lua thread coroutine deadlocked, starting new lua thread");
                      runtime.clone().start_runtime();
                    }
                    Err(_) => {
                      // @codyduong LOL, todo
                      log::error!("Lua channel closed before receiving confirmation of launch of lua thread");
                      is_stopped.store(true, Ordering::SeqCst);
                    }
                  }
                } else {
                  log::info!("Lua thread finished executing, starting new lua thread");
                  // otherwise we have finished execution, so dispatch a new thread
                  runtime.clone().start_runtime();
                  is_stopped.store(false, Ordering::SeqCst);
                }
              }
              Err(e) => {
                log::error!("Watch error: {:?}", e);
              }
            },
            Err(e) => {
              log::error!("Receive error: {:?}", e);
              break;
            }
          };
          thread::sleep(Duration::from_millis(100));
        }
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
    let komorebi_socket = socket::start(sender_clone);
    if komorebi_socket.is_err() {
      log::debug!("No komorebi connection created");
    }

    let model = App {
      // lua,
      bars: Vec::new(),
      file_last_checked_at,
      weather_station: Arc::new(Mutex::new(None)),
      weather_station_count: Arc::new(AtomicUsize::new(0)),
      system: SystemWrapper::default(),
      is_destroyed_condvar,
      bars_destroyed_condvar: Arc::new((Mutex::new(0), Condvar::new())),
      _debouncer: debouncer,
      _css_debouncer: css_debouncer,
      _tx_lua: tx_lua,
    };

    let widgets = view_output!();

    *EVENT.write() = VecDeque::with_capacity(50);
    *LUA_ACTION_REQUESTS.write() = VecDeque::with_capacity(50);

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      // Komorebi States
      AppMsg::Komorebi(notif) => {
        // Breaks on version mismatch between built and available version...
        // let notif: Option<komorebi_client::Notification> = serde_json::from_str(&notif).unwrap_or_else(|err| {
        //   log::warn!("Failed to read notifcations from komorebic: {:?}", err);

        //   None
        // });

        let mut deque = EVENT.write();
        if deque.len() == deque.capacity() {
          deque.pop_front();
        }
        deque.push_back(notif.clone());
        *STATE.write() = notif.state;
        *NEW_EVENT.write() = true;
      }
      AppMsg::KomorebiErr(line) => {
        println!("{:?}", &line);
      }

      AppMsg::LuaHook(info) => match info.t {
        LuaHookType::CreateBar(monitor, props, callback) => {
          let builder = bar::Bar::builder();

          let bar = builder
            .launch((*monitor, props, callback, root.clone(), Arc::clone(&self.bars_destroyed_condvar)))
            .forward(sender.input_sender(), |m| m);

          self.bars.push(bar);
        }
        LuaHookType::ReadEvent => {
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
      AppMsg::RequestWeatherStation(tx, config) => {
        log::debug!("Requesting weather station");
        let mut weather_station_lock = self.weather_station.lock().unwrap();
        if let Some(weather_station) = &*weather_station_lock {
          log::debug!("Sending existing weather station");
          tx.send(weather_station.clone()).unwrap();
        } else {
          log::debug!("Creating new weather station");
          let weather_station = WeatherStation::new(
            config.expect("There was no existing weather station and we did not provide props for a new one"),
          );
          *weather_station_lock = Some(weather_station.clone());
          tx.send(weather_station).unwrap();
        }
        self
          .weather_station_count
          .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
      }
      AppMsg::DropWeatherStation => {
        log::debug!("Dropping ref to weather station");
        let weather_station_count = &self.weather_station_count;
        let curr = self.weather_station_count.load(std::sync::atomic::Ordering::SeqCst);
        let min = curr.saturating_sub(1);
        weather_station_count.fetch_min(min, std::sync::atomic::Ordering::SeqCst);
        let mut weather_station_lock = self.weather_station.lock().unwrap();
        if weather_station_count.load(std::sync::atomic::Ordering::SeqCst) == 0 {
          *weather_station_lock = None;
          log::debug!("Weather Station dropped")
        }
      }
      AppMsg::RequestSystem(tx) => {
        log::debug!("Requesting system");
        tx.send(self.system.clone()).unwrap();
      }
      AppMsg::RequestLuaAction(id, args, f) => {
        log::debug!("Requested lua in component");
        let mut deque = LUA_ACTION_REQUESTS.write();
        if deque.len() == deque.capacity() {
          panic!("Our requests filled!")
        }
        deque.push_back(LuaActionRequest { id, args, f: Some(f) });
      }
      AppMsg::DestroyActual => {
        self.weather_station_count.fetch_min(0, std::sync::atomic::Ordering::SeqCst);
        let mut weather_station_lock = self.weather_station.lock().unwrap();
        *weather_station_lock = None;

        let bars_count = self.bars.len();
        for bar in self.bars.drain(..) {
          bar.widget().destroy();
          drop(bar);
        }

        // im not sure this does anything meaningful without sync signals from SHAppBarMessage
        let (lock, cvar) = &*self.bars_destroyed_condvar;
        let mut bars_destroyed = lock.lock().unwrap();
        while *bars_destroyed != bars_count {
          bars_destroyed = cvar.wait(bars_destroyed).unwrap();
          thread::sleep(Duration::from_millis(50));
        }
        let (lock, cvar) = &*self.is_destroyed_condvar;
        let mut destroyed = lock.lock().unwrap();

        *bars_destroyed = 0;
        *destroyed = true;

        cvar.notify_one();
      }
      AppMsg::NoOp => {}
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
    cleanup();
  }
}

#[tokio::main]
async fn main() {
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
    settings.set_property("gtk-xft-antialias", -1);
    settings.set_property("gtk-xft-hinting", -1);
    settings.set_property("gtk-xft-hintstyle", "hintfull");
    settings.set_property("gtk-xft-rgba", "rgb");
    settings.set_property("gtk-xft-dpi", 300);
  }

  let app = RelmApp::new("com.example.hitokage");
  let is_stopped = Arc::new(AtomicBool::new(false));
  let lua = mlua::Lua::new();

  {
    let is_stopped = is_stopped.clone();
    tokio::spawn(async move {
      tokio::signal::ctrl_c().await.unwrap();
      is_stopped.store(true, Ordering::SeqCst);
      use windows::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_C_EVENT};
      cleanup();

      // todo this should be signalled rather than timed for completion of cleanup
      tokio::time::sleep(Duration::from_millis(500)).await;

      unsafe {
        GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0).expect("Failed to re-send CTRL+C signal");
      }
    });
  }

  // let _ = app.set_global_css_from_file(style_file_path);
  app.run::<App>(AppInit { lua, is_stopped });
}

fn cleanup() {
  socket::shutdown().expect("Failed to shutdown komorebi socket");
}
