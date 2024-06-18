use gtk4::prelude::*;
use hitokage_core::lua::event::{EventNotif, STATE};
use hitokage_core::lua::event::{EVENT, NEW_EVENT};
use hitokage_core::widgets::bar;
use hitokage_core::{widgets, win_utils};
use hitokage_lua::AppMsg;
use hitokage_lua::LuaHookType;
use relm4::component::Connector;
use relm4::prelude::*;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;
// use windows::Win32::Foundation::CloseHandle;
// use windows::Win32::Foundation::HANDLE;
// use windows::Win32::System::Threading::{OpenThread, TerminateThread, THREAD_TERMINATE};
mod socket;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

struct App {
  bars: Vec<Connector<widgets::bar::Bar>>,
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

    let lua_file_path = "./hitokage.lua";
    let mut file = File::open(lua_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let user_script = contents;

    let sender_clone = sender.clone();

    let preventer_called = Arc::new(Mutex::new(false));
    let preventer_called_clone = Arc::clone(&preventer_called);

    let is_stopped = Arc::new(Mutex::new(false));
    let is_stopped_clone = Arc::clone(&is_stopped);

    let lua_thread_id = Arc::new(Mutex::new(0));
    let lua_thread_id_clone = Arc::clone(&lua_thread_id);

    let _lua_handle = std::thread::spawn(move || -> anyhow::Result<(), mlua::Error> {
      let lua = hitokage_lua::make(sender_clone).unwrap();

      {
        let mut thread_id = lua_thread_id.lock().unwrap();
        *thread_id = win_utils::get_current_thread_id();
      }

      {
        let globals = lua.globals();

        let preventer_fn = lua.create_function(move |_, ()| {
          let mut called = preventer_called.lock().unwrap();
          *called = true;
          Ok(())
        })?;

        globals.set("_not_deadlocked", preventer_fn)?;

        Ok::<(), mlua::Error>(())
      }?;

      let coroutine = lua
        .create_thread(lua.load(user_script).into_function().unwrap())
        .unwrap();

      loop {
        let time = Instant::now();
        match coroutine.resume::<_, ()>(()) {
          Ok(_) => (),
          Err(mlua::Error::CoroutineInactive) => {
            let mut is_stopped = is_stopped.lock().unwrap();
            *is_stopped = true;
            break Ok(());
          }
          Err(err) => {
            log::error!("Lua error: {:?}", err);
            let mut is_stopped = is_stopped.lock().unwrap();
            *is_stopped = true;
            break Err(err);
          }
        }
        if time.elapsed() <= Duration::from_millis(100) {
          std::thread::sleep(Duration::from_millis(100) - time.elapsed())
        }
      }
    });

    let _monitor_handle = thread::spawn(move || {
      let mut start_time = Instant::now();

      loop {
        {
          let is_stopped = is_stopped_clone.lock().unwrap();

          if *is_stopped {
            log::debug!("Lua execution finished, stopping lua watcher");
            break;
          }
        }

        if start_time.elapsed() >= Duration::from_secs(5) {
          log::error!(
            "There was a possible infinite loop or deadlock detected in your hitokage.lua! Did you mean to use hitokage.loop(): "
          ); //@codyduong add a link to user-end docs

          let _thread_id = *lua_thread_id_clone.lock().unwrap();

          // I'm sure there are no leaks or problems here LOL /s - @codyduong
          // log::debug!("Attempting to terminate lua thread with id: {:?}", thread_id);
          // if thread_id != 0 {
          //   unsafe {
          //     let handle = OpenThread(THREAD_TERMINATE, false, thread_id).unwrap();

          //     if handle != HANDLE(0) {
          //       let result = TerminateThread(handle, 1);

          //       if let Err(result) = result {
          //         // let error_code = windows::Win32::Foundation::GetLastError();
          //         log::error!("Failed to terminate thread: {:?}", result);
          //       } else {
          //         log::debug!("Successfully terminated thread");
          //       }

          //       let _ = CloseHandle(handle);
          //     } else {
          //       let error_code = windows::Win32::Foundation::GetLastError();
          //       log::error!("Failed to open thread handle: {:?}", error_code);
          //     }
          //   }
          // }

          break;
        }

        {
          let mut called = preventer_called_clone.lock().unwrap();

          if *called {
            *called = false;
            start_time = Instant::now();
          };
        }

        thread::sleep(Duration::from_millis(500));
      }
    });

    // komorebi pipe
    let sender_clone = sender.clone();
    socket::start_async_reader_new(sender_clone);

    // bar.widget().realize();
    // gtk4::prelude::WidgetExt::realize(bar.widget());

    let model = App {
      bars: Vec::new(),
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

      //
      AppMsg::LuaHook(info) => match info.t {
        LuaHookType::CreateBar(props, id, callback) => {
          let app = relm4::main_application();
          let builder = bar::Bar::builder();

          app.add_window(&builder.root);

          let bar = builder.launch((props, id, callback));
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
          println!("todo");

          ()
        }
      },
    }
  }
}

fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();

  unsafe {
    SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).expect("Failed to set process DPI awareness");
  }

  let style_file_path = "./styles.css";

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
