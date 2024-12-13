use crate::{App, LuaCoroutineMessage};
use gtk4::{style_context_add_provider_for_display, style_context_remove_provider_for_display, ApplicationWindow};
use hitokage_core::{event::CONFIG_UPDATE, win_utils};
use mlua::LuaSerdeExt;
use relm4::ComponentSender;
use std::{
  fs::File,
  io::Read,
  path::PathBuf,
  sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    mpsc::Sender,
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
  lua: mlua::Lua,
  sender: ComponentSender<App>,
  file_path: PathBuf,
  lua_thread_id: Arc<AtomicU32>,
  preventer_called: Arc<AtomicBool>,
  is_stopped: Arc<AtomicBool>,
  file_last_checked_at: Arc<Mutex<Instant>>,
  tx: Sender<bool>,
) -> std::thread::JoinHandle<Result<(), mlua::Error>> {
  return thread::spawn(move || -> anyhow::Result<(), mlua::Error> {
    let lua = hitokage_lua::make(lua, sender.clone()).unwrap();

    lua_thread_id.store(win_utils::get_current_thread_id(), Ordering::SeqCst);

    let globals = lua.globals();

    let preventer_fn = lua.create_function(move |_, ()| {
      preventer_called.store(true, Ordering::SeqCst);
      Ok(())
    })?;

    globals.set("_not_deadlocked", preventer_fn)?;

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
          mlua::ThreadStatus::Finished => false,
          mlua::ThreadStatus::Running => false,
          mlua::ThreadStatus::Error => false,
        };
        let mut guard = file_last_checked_at.lock().unwrap();
        // skip if we havent passed enough time, or we aren't invalid status
        // @codyduong this is super fragile, we can double run this on accident, instead use a different method to check
        if ((*guard).elapsed() <= Duration::from_millis(250)) || valid_status {
          drop(guard);
          return Ok(mlua::VmState::Continue);
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

        Ok(mlua::VmState::Continue)
      }
    });

    loop {
      let time = Instant::now();
      match coroutine.resume::<mlua::Value>(()) {
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
                  is_stopped.store(true, Ordering::SeqCst);
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
                is_stopped.store(true, Ordering::SeqCst);
                break Err(err);
              }
            }
          }
        },
        Err(mlua::Error::CoroutineUnresumable) => {
          is_stopped.store(true, Ordering::SeqCst);
          break Ok(());
        }
        Err(err) => {
          log::error!("Lua error: {:?}", err);
          is_stopped.store(true, Ordering::SeqCst);
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
  preventer_called: Arc<AtomicBool>,
  is_stopped: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
  return thread::spawn(move || {
    let mut start_time = Instant::now();

    loop {
      let is_stopped = is_stopped.load(Ordering::SeqCst);

      if is_stopped {
        log::debug!("Lua execution finished, stopping lua watcher");
        break;
      }

      let overrun = start_time.elapsed() >= Duration::from_secs(5);

      if overrun {
        log::error!(
          "There was a possible infinite loop or deadlock detected in your hitokage.lua! Did you mean to use hitokage.dispatch()? "
        ); //@codyduong add a link to user-end docs
        start_time = Instant::now();
      }

      let called = preventer_called.load(Ordering::SeqCst);

      if called {
        preventer_called.store(true, Ordering::SeqCst);
        start_time = Instant::now();
      };

      thread::sleep(Duration::from_millis(100));
    }
  });
}

pub fn terminate_thread(id_arc: Arc<AtomicU32>) {
  let id_guard = id_arc.load(Ordering::SeqCst);
  log::debug!("Attempting to terminate lua thread with id: {:?}", id_guard);
  if id_guard != 0 {
    unsafe {
      let handle = OpenThread(THREAD_TERMINATE, false, id_guard).unwrap();

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
  id_arc.store(0, Ordering::SeqCst);
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
