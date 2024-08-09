use crate::{App, LuaCoroutineMessage};
use gtk4::{style_context_add_provider_for_display, style_context_remove_provider_for_display, ApplicationWindow};
use hitokage_core::{event::CONFIG_UPDATE, win_utils};
use mlua::LuaSerdeExt;
use notify_debouncer_full::DebouncedEvent;
use relm4::ComponentSender;
use std::{
  fs::File,
  io::Read,
  path::PathBuf,
  sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
  },
  thread::{self},
  time::{Duration, Instant},
};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{OpenThread, TerminateThread, THREAD_TERMINATE};

pub fn load_content(path: Option<PathBuf>) -> String {
  let mut contents = String::new();

  if let Some(path) = path {
    let mut file = File::open(path.clone()).unwrap();
    file.read_to_string(&mut contents).unwrap();
  }

  let prepend = include_str!("./lua/prepend.lua");
  let append = include_str!("./lua/append.lua");

  prepend.to_owned() + "\n" + &contents + "\n" + append
}

pub fn create_lua_handle(
  sender: ComponentSender<App>,
  file_path: PathBuf,
  lua_thread_id: Arc<Mutex<u32>>,
  preventer_called: Arc<Mutex<bool>>,
  is_stopped: Arc<Mutex<bool>>,
  file_last_checked_at: Arc<Mutex<Instant>>,
  tx: Sender<bool>,
) -> std::thread::JoinHandle<Result<(), mlua::Error>> {
  return thread::spawn(move || -> anyhow::Result<(), mlua::Error> {
    let lua = hitokage_lua::make(sender).unwrap();

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

    let func_or_err = lua.load(load_content(Some(file_path.clone()))).into_function();

    // if we failed to create a coroutine default to an empty script
    let coroutine = lua
      .create_thread({
        match func_or_err {
          Ok(func) => func,
          Err(err) => {
            log::error!("There was an error loading your user script: {:?}", err);
            log::info!("Falling back to an empty script. Waiting for user script fixes");
            lua
              .load(load_content(None))
              .into_function()
              .expect("Internal script error when falling back to empty user script")
          }
        }
      })
      .expect("Failed to create a lua thread");

    let tx_clone = tx.clone();

    // file-watcher is injected through a hook to ensure we are polling it through other means as well
    coroutine.set_hook(mlua::HookTriggers::new().every_nth_instruction(500), {
      let file_last_checked_at = Arc::clone(&file_last_checked_at);
      move |lua, _| {
        let valid_status = match lua.current_thread().status() {
          mlua::ThreadStatus::Resumable => true,
          mlua::ThreadStatus::Unresumable => false,
          mlua::ThreadStatus::Error => false,
        };
        let mut guard = file_last_checked_at.lock().unwrap();
        // skip if we havent passed enough time, or we aren't invalid status
        // @codyduong this is super fragile, we can double run this on accident, instead use a different method to check
        if ((*guard).elapsed() <= Duration::from_millis(250)) || valid_status {
          drop(guard);
          return Ok(());
        }

        let mut update_guard = CONFIG_UPDATE.write();
        if *update_guard {
          *update_guard = false;
          drop(update_guard);

          lua.remove_hook();

          {
            let lua_thread_id = lua_thread_id.clone();
            thread::spawn(move || {
              terminate_thread(lua_thread_id.clone());
            });
          }

          // safe reload not possible!
          tx_clone.send(false).unwrap();
        }

        *guard = Instant::now();
        drop(guard);

        Ok(())
      }
    });

    loop {
      let time = Instant::now();
      match coroutine.resume::<_, mlua::Value>(()) {
        Ok(value) => match value {
          mlua::Value::Nil => (),
          mlua::Value::Boolean(_) => (),
          mlua::Value::UserData(_) => (),
          mlua::Value::LightUserData(_) => (),
          _ => {
            let props: mlua::Result<LuaCoroutineMessage> = lua.from_value(value.clone());

            match props {
              Ok(msg) => match msg {
                LuaCoroutineMessage::Suspend => {
                  log::info!("Received suspend from lua coroutine");
                  let mut is_stopped = is_stopped.lock().unwrap();
                  *is_stopped = true;
                  break Ok(());
                }
                LuaCoroutineMessage::Reload => {
                  log::info!("Received reload from lua coroutine");
                  let mut sswg = CONFIG_UPDATE.write();
                  *sswg = false;
                  let user_script = load_content(Some(file_path.clone()));
                  let result = lua.load(user_script).into_function();
                  drop(sswg);
                  match result {
                    Ok(func) => {
                      let _ = coroutine.reset(func);
                      tx.send(true).unwrap(); // safe reload success
                    }
                    Err(_) => {
                      // no need to handle the error since we will attempt to start again
                      // which will indicate the error
                      tx.send(false).unwrap();
                    }
                  }
                }
              },
              Err(err) => {
                log::error!("Received unknown coroutine return, {:?}, {:?}", err, value);
                let mut is_stopped = is_stopped.lock().unwrap();
                *is_stopped = true;
                break Err(err);
              }
            }
          }
        },
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
}

pub fn create_watcher_handle(
  preventer_called: Arc<Mutex<bool>>,
  is_stopped: Arc<Mutex<bool>>,
) -> std::thread::JoinHandle<()> {
  return thread::spawn(move || {
    let mut start_time = Instant::now();

    loop {
      {
        let is_stopped = is_stopped.lock().unwrap();

        if *is_stopped {
          log::debug!("Lua execution finished, stopping lua watcher");
          break;
        }
      }

      let overrun = start_time.elapsed() >= Duration::from_secs(5);

      if overrun {
        log::error!(
          "There was a possible infinite loop or deadlock detected in your hitokage.lua! Did you mean to use hitokage.dispatch()? "
        ); //@codyduong add a link to user-end docs
        start_time = Instant::now();
      }

      {
        let mut called = preventer_called.lock().unwrap();

        if *called {
          *called = false;
          start_time = Instant::now();
        };
      }

      thread::sleep(Duration::from_millis(100));
    }
  });
}

pub fn terminate_thread(id_arc: Arc<Mutex<u32>>) {
  let mut id_guard = id_arc.lock().unwrap();
  log::debug!("Attempting to terminate lua thread with id: {:?}", id_guard);
  if *id_guard != 0 {
    unsafe {
      let handle = OpenThread(THREAD_TERMINATE, false, *id_guard).unwrap();

      if handle != HANDLE(0) {
        let result = TerminateThread(handle, 1);

        if let Err(result) = result {
          // let error_code = windows::Win32::Foundation::GetLastError();
          log::error!("Failed to terminate thread: {:?}", result);
        } else {
          log::debug!("Successfully terminated thread");
        }

        let _ = CloseHandle(handle);
      } else {
        let error_code = windows::Win32::Foundation::GetLastError();
        log::error!("Failed to open thread handle: {:?}", error_code);
      }
    }
  }
  *id_guard = 0;
}

pub fn reload_css_provider(
  root: &ApplicationWindow,
  css_file_path: &PathBuf,
  old_provider: &gtk4::CssProvider,
) -> gtk4::CssProvider {
  let provider = gtk4::CssProvider::new();
  let css_file = gdk4::gio::File::for_path(&css_file_path);
  provider.load_from_file(&css_file);

  let display = gtk4::prelude::WidgetExt::display(root);

  style_context_remove_provider_for_display(&display, old_provider);

  style_context_add_provider_for_display(&display, &provider, 500);
  provider
}

// pub fn reload_css(
//   root: &ApplicationWindow,
//   css_file_path: &PathBuf,
//   rx: Receiver<Result<Vec<DebouncedEvent>, Vec<notify::Error>>>,
// ) -> Result<Vec<DebouncedEvent>, std::sync::mpsc::TryRecvError> {
//   let mut old_providers: Vec<&gtk4::CssProvider> = vec![];

//   match rx.try_recv() {
//     Ok(result) => {
//       match result {
//         Ok(result) => {
          
//           result
//         },
//         Err(error) => {

//         }
//       }
//     }
//     Err(error) => {
//       log::error!("There was an error reloading css: {:?}", error);
//       Err(error)
//     }
//   }
//   // glib::source::idle_add_local(move ||);
// }
