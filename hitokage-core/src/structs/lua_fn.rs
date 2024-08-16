// a deserializable lua func into rust

use std::fmt;

use mlua::{Function, IntoLua, Lua, LuaSerdeExt, MultiValue, Table, Value};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Default, Serialize)]
pub struct LuaFn {
  fn_name: String, // some UUID
}

impl LuaFn {
  pub fn new(fn_name: String) -> Self {
    LuaFn { fn_name }
  }

  pub fn call<'lua>(self, lua: &'lua Lua, args: MultiValue<'lua>) -> mlua::Result<MultiValue<'lua>> {
    let lua_fn_name = self.fn_name.into_lua(lua).unwrap();
    let lua_fn = lua
      .globals()
      .get::<_, Table>("_fn")
      .unwrap()
      .get::<_, Function>(lua_fn_name)
      .unwrap();

    let res = lua_fn.call::<MultiValue<'lua>, mlua::MultiValue>(args);
    res
  }
}

struct LuaFnVisitor;

impl<'de> serde::de::Visitor<'de> for LuaFnVisitor {
  type Value = LuaFn;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a lua function")
  }

  fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
      where
          E: serde::de::Error, {
      let str = String::from_utf8(v.to_vec()).expect("Failed to deserialize lua function into rust");

      Ok(LuaFn::new(str))
  }
}

impl<'de> Deserialize<'de> for LuaFn {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(LuaFnVisitor)
  }
}