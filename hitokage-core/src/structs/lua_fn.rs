use mlua::RegistryKey;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct LuaFn {
  // pub f: mlua::OwnedFunction,
  pub r: RegistryKey
}

// impl From<LuaFn> for mlua::OwnedFunction {
//   fn from(value: LuaFn) -> Self {
//     value.f
//   }
// }

impl<'de> Deserialize<'de> for LuaFn {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(LuaFnVisitor::default())
  }
}

pub(crate) struct LuaFnVisitor {
  pub(crate) f: Option<mlua::OwnedFunction>,
}

impl Default for LuaFnVisitor {
  fn default() -> Self {
    Self { f: None }
  }
}

impl<'de> serde::de::Visitor<'de> for LuaFnVisitor {
  type Value = LuaFn;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a byte buffer representing a raw pointer")
  }

  fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
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

    Ok(Self::Value {
      // f: self.f.expect("Failed to deserialize lua function"),
      r,
    })
  }
}
