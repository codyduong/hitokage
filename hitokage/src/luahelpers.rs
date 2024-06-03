use mlua::{Function, IntoLua, Lua, LuaSerdeExt};
use relm4::ComponentSender;
use std::sync::{Arc, Mutex};

struct LuaFunctionHolder {
  func: Arc<Mutex<Function<'static>>>,
}

impl LuaFunctionHolder {
  fn new(func: Function<'static>) -> Self {
    LuaFunctionHolder {
      func: Arc::new(Mutex::new(func)),
    }
  }

  fn call(&self, args: Vec<mlua::Value>) -> anyhow::Result<()> {
    let func = self.func.lock().unwrap();
    func.call::<_, ()>(args)?;
    Ok(())
  }
}

// We need to further augment the lua modules here, since otherwise we have a cylic dependency
pub fn augment(lua: Lua, sender: ComponentSender<crate::AppState>) -> anyhow::Result<Lua> {
  let sender1 = sender.clone();
  let sender2 = sender.clone();

  {
    lua
      .globals()
      .set(
        "create_widget",
        lua
          .create_function(move |_, ()| {
            sender1.input(crate::Msg::LuaHook(crate::LuaHook {
              t: (crate::LuaHookType::CreateWidget),
              callback: Box::new(|_| Ok(())),
            }));
            Ok(())
          })
          .expect("Failed to set function"),
      )
      .expect("Failed to set function");

    lua
      .globals()
      .set(
        "read_state",
        lua
          .create_function(move |lua2, f: Function| {
            sender2.input(crate::Msg::LuaHook(crate::LuaHook {
              t: (crate::LuaHookType::ReadState),
              callback: Box::new(|_| Ok(())),
            }));
            let args = crate::STATE.read();
            if args.is_some() {
              f.call::<_, ()>(lua2.to_value(args.as_ref().unwrap()))?;
            } else {
              f.call::<_, ()>(mlua::Nil)?;
            }
            
            Ok(())
          })
          .expect("Failed to set function"),
      )
      .expect("Failed to set function");
  }

  Ok(lua)
}
