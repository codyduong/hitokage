use gdk4::{
  glib::{self, object::Cast},
  prelude::*,
};
use hitokage_core::common::{Monitor, MonitorGeometry};
use mlua::{
  Lua, LuaSerdeExt,
  Value::{self},
};

fn get_monitors() -> impl Iterator<Item = Monitor> {
  let display = gdk4::Display::default().expect("Failed to get default display");
  let monitors: Vec<_> = display.monitors().iter().collect();
  let iter = monitors.into_iter().filter_map(|result| {
    result.ok().and_then(
      |item: glib::Object| match item.downcast::<gdk4_win32::Win32Monitor>().ok() {
        Some(monitor) => {
          let geometry: MonitorGeometry = monitor.geometry().into();
          Some(Monitor {
            connecter: monitor.connector().map(|s| s.to_string()),
            description: monitor.description().map(|s| s.to_string()),
            geometry,
            manufacturer: monitor.manufacturer().map(|s| s.to_string()),
            model: monitor.model().map(|s| s.to_string()),
            refresh_rate: monitor.refresh_rate(),
            is_primary: geometry.x == 0 && geometry.y == 0,
          })
        }
        _ => None,
      },
    )
  });
  iter
}

fn all<'lua>(lua: &'lua Lua, _: Value) -> mlua::Result<Value<'lua>> {
  let monitors_vec: Vec<Monitor> = get_monitors().collect();

  let res = lua.to_value(&monitors_vec)?;

  Ok(res)
}

// @todo @codyduong
fn current() {}

fn primary<'lua>(lua: &'lua Lua, _: Value) -> mlua::Result<Value<'lua>> {
  let monitors_vec: Option<Monitor> = get_monitors().find(|m| m.geometry.x == 0 && m.geometry.y == 0);

  let res = lua.to_value(&monitors_vec)?;

  Ok(res)
}

pub fn make<'lua>(lua: &'lua Lua) -> anyhow::Result<mlua::Table<'lua>> {
  let table = lua.create_table()?;

  table.set("all", lua.create_function(all)?)?;

  // hitokage_display.set("current", lua.create_function(|_, _args: Variadic<Value>| Ok(()))?)?;

  table.set("primary", lua.create_function(primary)?)?;

  Ok(table)
}

// tests
#[cfg(test)]
mod tests {
  use mlua::{Lua, Table, Value};

  use crate::assert_lua_type;

  fn create_table(lua: &Lua) -> anyhow::Result<Table> {
    return super::make(lua);
  }

  #[test]
  fn test_all() -> anyhow::Result<()> {
    {
      let lua = Lua::new();
      let table = create_table(&lua)?;
      lua.globals().set("table", table)?;
      let value: Value = lua.globals().get("table")?;

      let table: Table = assert_lua_type!(value, mlua::Table);
      assert_lua_type!(table.get::<&str, Value>("all")?, mlua::Function);
      assert_lua_type!(table.get::<&str, Value>("primary")?, mlua::Function);
      assert_eq!(table.len()?, 0);
      assert_eq!(table.pairs::<String, Value>().count(), 2);
    }

    Ok(())
  }
}
