use gtk4::prelude::*;
use mlua::{
  Function, IntoLua, Lua, LuaSerdeExt,
  Value::{self, Nil},
};
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::bar::BarProps;

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
  let lua = std::rc::Rc::new(lua);

  let binding = lua.clone();
  let hitokage_mod = hitokage_lua::get_or_create_module(&binding, "hitokage")?;
  let bar_mod = hitokage_lua::get_or_create_sub_module(&binding, "bar")?;

  {
    // hitokage.bar.*
    bar_mod.set(
      "create",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, value: Value| {
          let props: BarProps = lua_inner.from_value(value)?;
          sender.input(crate::Msg::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::CreateBar(props),
            callback: Box::new(|_| Ok(())),
          }));
          Ok(())
        }
      })?,
    )?;

    // hitokage.*
    hitokage_mod.set(
      "read_state",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, f: Value| {
          sender.input(crate::Msg::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::ReadState,
            callback: Box::new(|_| Ok(())),
          }));
          let args = crate::STATE.read();

          let lua_args = lua_inner.to_value(std::ops::Deref::deref(&args));

          match f {
            Value::Function(func) => func.call::<_, ()>(lua_args.clone())?,
            Value::Nil => (),
            _ => {
              return Err(mlua::Error::FromLuaConversionError {
                from: f.type_name(),
                to: "Function or Nil",
                message: Some("Expected a function or nil".to_string()),
              })
            }
          }

          Ok(lua_args)
        }
      })?,
    )?;

    hitokage_mod.set(
      "new_state",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, f: Value| {
          sender.input(crate::Msg::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::NoAction,
            callback: Box::new(|_| Ok(())),
          }));
          let args = crate::NEW_STATE.read();

          let lua_args = lua_inner.to_value(std::ops::Deref::deref(&args));

          match f {
            Value::Function(func) => {
              func.call::<_, ()>(lua_args.clone())?;
            }
            Value::Nil => (),
            _ => {
              return Err(mlua::Error::FromLuaConversionError {
                from: f.type_name(),
                to: "Function or Nil",
                message: Some("Expected a function or nil".to_string()),
              })
            }
          }

          Ok(lua_args)
        }
      })?,
    )?;

    // hitokage_mod.set(
    //   "subscribe_state",
    //   lua.create_function({
    //     let sender = sender.clone();
    //     move |lua_inner, value: Value| {
          
    //       let args = crate::NEW_STATE.read();

    //       let lua_args = lua_inner.to_value(std::ops::Deref::deref(&args));

    //       match value {
    //         Value::Function(func) => {
    //           sender.input(crate::Msg::LuaHook(crate::LuaHook {
    //             t: crate::LuaHookType::NoAction,
    //             callback: Box::new(|arg| {
    //               func.call::<_, ()>(arg);
    //               Ok(())
    //             }),
    //           }));
    //         }
    //         _ => {
    //           return Err(mlua::Error::FromLuaConversionError {
    //             from: value.type_name(),
    //             to: "Function",
    //             message: Some("Expected a function".to_string()),
    //           })
    //         }
    //       }

    //       Ok(lua_args)
    //     }
    //   })?,
    // )?;

    // TOOD @codyduong
    // subscriptions should be a sugar syntax around the subscription loop
    // it is entirely handled in lua since it is the only safe way
  }

  Ok(lua)
}
