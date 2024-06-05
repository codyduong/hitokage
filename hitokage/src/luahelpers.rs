use mlua::{Function, IntoLua, Lua, LuaSerdeExt, Value::{self, Nil}};
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
pub fn augment(lua: Lua, sender: ComponentSender<crate::App>) -> anyhow::Result<std::rc::Rc<Lua>> {
  let sender1 = sender.clone();
  let sender2 = sender.clone();
  let sender3 = sender.clone();

  let lua = std::rc::Rc::new(lua);

  let binding = lua.clone();
  let hitokage_mod = hitokage_lua::get_or_create_module(&binding, "hitokage")?;
  let bar_mod = hitokage_lua::get_or_create_sub_module(&binding, "bar")?;

  {
    // hitokage.bar.*
    bar_mod.set(
      "create",
      lua.create_function(move |_, ()| {
        sender1.input(crate::Msg::LuaHook(crate::LuaHook {
          t: (crate::LuaHookType::CreateBar),
          callback: Box::new(|_| Ok(())),
        }));
        Ok(())
      })?,
    )?;

    // hitokage.*
    hitokage_mod.set(
      "read_state",
      lua.create_function(move |lua2, f: Value| {
        sender2.input(crate::Msg::LuaHook(crate::LuaHook {
          t: crate::LuaHookType::ReadState,
          callback: Box::new(|_| Ok(())),
        }));
        let args = crate::STATE.read();

        let lua_args = lua2.to_value(std::ops::Deref::deref(&args));
    
        match f {
          Value::Function(func) => {
            func.call::<_, ()>(lua_args.clone())?
          }
          Value::Nil => (),
          _ => return Err(mlua::Error::FromLuaConversionError {
            from: f.type_name(),
            to: "Function or Nil",
            message: Some("Expected a function or nil".to_string()),
          }),
        }
    
        Ok(lua_args)
      })?,
    )?;

    hitokage_mod.set(
      "new_state",
      lua.create_function(move |lua2, f: Value| {
        sender3.input(crate::Msg::LuaHook(crate::LuaHook {
          t: crate::LuaHookType::NoAction,
          callback: Box::new(|_| Ok(())),
        }));
        let args = crate::NEW_STATE.read();
    
        let lua_args = lua2.to_value(std::ops::Deref::deref(&args));
    
        match f {
          Value::Function(func) => {
            func.call::<_, ()>(lua_args.clone())?;
          }
          Value::Nil => (),
          _ => return Err(mlua::Error::FromLuaConversionError {
            from: f.type_name(),
            to: "Function or Nil",
            message: Some("Expected a function or nil".to_string()),
          }),
        }
    
        Ok(lua_args)
      })?,
    )?;

    // TOOD @codyduong
    // subscriptions should be a sugar syntax around the subscription loop
    // it is entirely handled in lua since it is the only safe way
  }

  Ok(lua)
}
