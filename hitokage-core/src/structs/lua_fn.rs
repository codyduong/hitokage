use mlua::RegistryKey;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LuaFn {
  pub r: Arc<RegistryKey>,
  // pub f: mlua::Function
}

impl<'de> Deserialize<'de> for LuaFn {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(LuaFnVisitor)
  }
}

pub struct LuaFnVisitor;

impl serde::de::Visitor<'_> for LuaFnVisitor {
  type Value = LuaFn;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a byte buffer representing a raw pointer")
  }

  fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(parse_bytes(value))
  }
}

pub(crate) fn parse_bytes(value: &[u8]) -> LuaFn {
  assert_eq!(
    value.len(),
    std::mem::size_of::<u8>() + std::mem::size_of::<usize>(),
    "Byte slice length is incorrect"
  );

  let identifier = value[0];
  assert_eq!(identifier, 0x01, "Identifier byte does not match expected value");

  let (_, id) = value.split_at(1);
  let id = usize::from_ne_bytes(id.try_into().unwrap());

  let r = crate::deserializer::FUNCTION_REGISTRY
    .lock()
    .unwrap()
    .remove(&id)
    .unwrap();

  LuaFn { r: Arc::new(r) }
}
