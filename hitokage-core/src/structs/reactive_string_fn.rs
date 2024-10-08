use super::lua_fn::LuaFn;
use super::reactive::{AsReactive, Reactive};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum ReactiveStringFn {
  Str(String),
  Reactive(Reactive<String>),
  Function(LuaFn),
}

struct ReactiveStringVisitor;

impl<'de> serde::de::Visitor<'de> for ReactiveStringVisitor {
  type Value = ReactiveStringFn;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a byte buffer representing a raw pointer")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(ReactiveStringFn::Str(v.to_owned()))
  }

  fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    match value[0] {
      0x00 => Ok(ReactiveStringFn::Reactive(super::reactive_string::parse_bytes(value))),
      0x01 => Ok(ReactiveStringFn::Function(super::lua_fn::parse_bytes(value))),
      _ => {
        panic!("Failed to deserialize unknown u8 byte array")
      }
    }
  }
}

impl<'de> Deserialize<'de> for ReactiveStringFn {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(ReactiveStringVisitor)
  }
}

impl ReactiveStringFn {
  pub fn as_fn(&self) -> Option<LuaFn> {
    match self {
      ReactiveStringFn::Function(f) => Some(f.clone()),
      _ => None
    }
  }
}

impl AsReactive<String> for ReactiveStringFn {
  fn as_reactive(self, sender: impl Into<Option<Sender<()>>>) -> Reactive<String> {
    match self {
      ReactiveStringFn::Str(str) => Reactive::<String> {
        value: Arc::new(Mutex::new(str.to_string())),
        sender: Arc::new(Mutex::new(sender.into())),
      },
      ReactiveStringFn::Reactive(reactive) => {
        let mut sender_guard = reactive.sender.lock().unwrap();
        *sender_guard = sender.into();

        Reactive::<String> {
          value: Arc::clone(&reactive.value),
          sender: Arc::clone(&reactive.sender),
        }
      }
      _ => Reactive::<String> {
        value: Arc::new(Mutex::new("".to_string())),
        sender: Arc::new(Mutex::new(sender.into())),
      }
    }
  }
}
