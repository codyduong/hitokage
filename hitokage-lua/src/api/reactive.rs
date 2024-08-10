use hitokage_core::structs::reactive::Reactive;
use mlua::{Lua, LuaSerdeExt, Value};

pub fn make(lua: &Lua) -> anyhow::Result<mlua::Table> {
  let table = lua.create_table()?;

  table.set(
    "create",
    lua.create_function({
      move |lua_inner, f: Value| {
        match f {
          Value::String(lua_str) => {
            let str = lua_str.to_str().unwrap().to_string();
            let foo = Reactive::new(str);
            log::info!("lua instantiated reactive at: {:?}", &foo);
            log::info!(
              "lua instantiated arc at: {:?}",
              std::sync::Arc::<std::sync::Mutex<std::string::String>>::as_ptr(&foo.value)
            );
            lua_inner.pack(foo)
          }
          // Value::Nil => (),
          _ => Err(mlua::Error::FromLuaConversionError {
            from: f.type_name(),
            to: "lmao",
            message: Some("Expected better arguments TODO @codyduong".to_string()),
          }),
        }
      }
    })?,
  )?;

  Ok(table)
}
