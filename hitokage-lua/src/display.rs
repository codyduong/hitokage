use gdk4::{
  glib::{self, object::Cast},
  prelude::*,
};
use mlua::{
  Lua, LuaSerdeExt,
  Value::{self, Nil},
  Variadic,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MonitorGeometry {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl From<gdk4::Rectangle> for MonitorGeometry {
  fn from(item: gdk4::Rectangle) -> Self {
    MonitorGeometry {
      x: item.x(),
      y: item.y(),
      width: item.width(),
      height: item.height(),
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct Monitor {
  connecter: Option<String>,
  description: Option<String>,
  geometry: MonitorGeometry,
  manufacturer: Option<String>,
  model: Option<String>,
  refresh_rate: i32,
  is_primary: bool,
}

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

fn current() {}

fn primary<'lua>(lua: &'lua Lua, _: Value) -> mlua::Result<Value<'lua>> {
  let monitors_vec: Option<Monitor> = get_monitors().find(|m| m.geometry.x == 0 && m.geometry.y == 0);

  let res = lua.to_value(&monitors_vec)?;

  Ok(res)
}

pub fn make_display<'lua>(lua: &'lua Lua) -> anyhow::Result<mlua::Table<'lua>> {
  let hitokage_display = lua.create_table()?;

  hitokage_display.set("all", lua.create_function(all)?)?;

  // hitokage_display.set("current", lua.create_function(|_, _args: Variadic<Value>| Ok(()))?)?;

  hitokage_display.set("primary", lua.create_function(primary)?)?;

  Ok(hitokage_display)
}

// tests
#[cfg(test)]
mod tests {
  use mlua::{Function, Integer, Lua, Result, String, Table, Value};

  trait FromLuaValue<'lua>: Sized {
    fn from_lua_value(value: Value<'lua>) -> Result<Self>;
  }

  impl<'lua> FromLuaValue<'lua> for Table<'lua> {
    fn from_lua_value(value: Value<'lua>) -> Result<Self> {
      match value {
        Value::Table(table) => Ok(table),
        _ => Err(mlua::Error::FromLuaConversionError {
          from: value.type_name(),
          to: "Table",
          message: None,
        }),
      }
    }
  }

  impl<'lua> FromLuaValue<'lua> for Function<'lua> {
    fn from_lua_value(value: Value<'lua>) -> Result<Self> {
      match value {
        Value::Function(function) => Ok(function),
        _ => Err(mlua::Error::FromLuaConversionError {
          from: value.type_name(),
          to: "Function",
          message: None,
        }),
      }
    }
  }

  impl<'lua> FromLuaValue<'lua> for String<'lua> {
    fn from_lua_value(value: Value<'lua>) -> Result<Self> {
      match value {
        Value::String(string) => Ok(string),
        _ => Err(mlua::Error::FromLuaConversionError {
          from: value.type_name(),
          to: "String",
          message: None,
        }),
      }
    }
  }

  impl<'lua> FromLuaValue<'lua> for Integer {
    fn from_lua_value(value: Value<'lua>) -> Result<Self> {
      match value {
        Value::Integer(integer) => Ok(integer),
        _ => Err(mlua::Error::FromLuaConversionError {
          from: value.type_name(),
          to: "Integer",
          message: None,
        }),
      }
    }
  }

  impl<'lua> FromLuaValue<'lua> for bool {
    fn from_lua_value(value: Value<'lua>) -> Result<Self> {
      match value {
        Value::Boolean(boolean) => Ok(boolean),
        _ => Err(mlua::Error::FromLuaConversionError {
          from: value.type_name(),
          to: "Boolean",
          message: None,
        }),
      }
    }
  }

  // Macro to assert the type of a Lua value and return the inner value if it matches
  macro_rules! assert_lua_type {
    ($value:expr, $type:ty) => {
      <$type as FromLuaValue>::from_lua_value($value)
        .expect(&format!("Expected Lua value to be of type {}", stringify!($type)))
    };
  }

  fn load_display(lua: &Lua) -> anyhow::Result<Table> {
    return super::make_display(lua);
  }

  #[test]
  fn test_all() -> anyhow::Result<()> {
    {
      let lua = Lua::new();
      let table = load_display(&lua)?;
      lua.globals().set("display", table)?;
      let value: Value = lua.globals().get("display")?;

      let table: Table = assert_lua_type!(value, Table);
      assert_lua_type!(table.get::<&str, Value>("all")?, mlua::Function);
      assert_lua_type!(table.get::<&str, Value>("primary")?, mlua::Function);
      assert_eq!(table.len()?, 0);
      assert_eq!(table.pairs::<String, Value>().count(), 2);
    }

    Ok(())
  }
}
