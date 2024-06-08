use gdk4::{
  glib::{self, object::Cast},
  prelude::*,
};
use hitokage_core::common::{Monitor, MonitorGeometry};
use mlua::{AnyUserData, Lua, LuaSerdeExt, MetaMethod, UserData, UserDataMethods, Value};

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

struct MonitorUserData {}

impl MonitorUserData {
  fn new() -> Self {
    MonitorUserData {}
  }
}

impl UserData for MonitorUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_function("all", all);
    methods.add_meta_function(MetaMethod::Call, all);

    methods.add_function("primary", primary);
  }
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

pub fn make<'lua>(lua: &'lua Lua) -> anyhow::Result<AnyUserData<'lua>> {
  let userdata = lua
    .create_userdata(MonitorUserData::new())
    .unwrap();

  Ok(userdata)
}

// tests
#[cfg(test)]
mod tests {
  use mlua::{AnyUserData, AnyUserDataExt, Lua, Table, UserData, Value};

  use crate::assert_lua_type;

  fn create_userdata(lua: &Lua) -> anyhow::Result<AnyUserData> {
    return super::make(lua);
  }

  #[test]
  fn test_all() -> anyhow::Result<()> {
    {
      let lua = Lua::new();
      let userdata = create_userdata(&lua)?;
      lua.globals().set("userdata", userdata)?;
      let value: Value = lua.globals().get("userdata")?;

      let userdata: AnyUserData = assert_lua_type!(value, AnyUserData);
      let metatable = userdata.get_metatable()?;
      assert_lua_type!(userdata.get::<&str, Value>("all")?, mlua::Function);
      assert_lua_type!(userdata.get::<&str, Value>("primary")?, mlua::Function);
      assert_lua_type!(metatable.get("__call")?, mlua::Function);
      // assert_eq!(table.len()?, 0);
      // assert_eq!(table.pairs::<String, Value>().count(), 2);
    }

    Ok(())
  }
}
