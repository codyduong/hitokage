use hitokage_core::structs::reactive::Reactive;
use mlua::{Lua, Value};

pub fn make(lua: &Lua) -> anyhow::Result<mlua::Table> {
  let table = lua.create_table()?;

  table.set(
    "create",
    lua.create_function({
      move |lua_inner, f: Value| {
        match f {
          Value::String(lua_str) => {
            let str = lua_str.to_str().unwrap().to_string();
            let reactive_string = Reactive::new(str);
            log::info!("lua instantiated reactive at: {:?}", &reactive_string);
            log::info!(
              "lua instantiated arc at: {:?}",
              std::sync::Arc::<std::sync::Mutex<std::string::String>>::as_ptr(&reactive_string.value)
            );
            lua_inner.pack(reactive_string)
          }
          // Value::Nil => (),
          _ => Err(mlua::Error::FromLuaConversionError {
            from: f.type_name(),
            to: "Reactive UserData".to_string(),
            message: Some("Failed to create a Reactive of this type. Is this a supported type?".to_string()),
          }),
        }
      }
    })?,
  )?;

  Ok(table)
}
