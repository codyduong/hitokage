use hitokage_core::event::{CONFIG_UPDATE, EVENT, NEW_EVENT};
use mlua::{Lua, LuaSerdeExt, Value};
use relm4::{Component, ComponentSender};

pub fn make<'lua, C>(lua: &'lua Lua, sender: &ComponentSender<C>) -> anyhow::Result<mlua::Table<'lua>>
where
  C: Component<Input = crate::AppMsg>,
  <C as Component>::Output: std::marker::Send,
{
  let table = lua.create_table()?;

  {
    table.set(
      "get_unread",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, f: Value| {
          sender.input(crate::AppMsg::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::ReadEvent,
            // callback: Box::new(|_| Ok(())),
          }));
          let args = EVENT.read();

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

    table.set(
      "has_unread",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, f: Value| {
          sender.input(crate::AppMsg::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::NoAction,
            // callback: Box::new(|_| Ok(())),
          }));
          let args = NEW_EVENT.read();

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

    let configuration = lua.create_table()?;

    configuration.set(
      "changed",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, _value: Value| {
          sender.input(crate::AppMsg::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::CheckConfigUpdate,
            // callback: Box::new(|_| Ok(())),
          }));
          let args = CONFIG_UPDATE.read();
          let lua_args = lua_inner.to_value(std::ops::Deref::deref(&args));

          Ok(lua_args)
        }
      })?,
    )?;

    table.set("configuration", configuration)?;
  }

  Ok(table)
}
