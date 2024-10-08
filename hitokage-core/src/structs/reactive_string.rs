use super::reactive::{AsReactive, Reactive};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum ReactiveString {
  Str(String),
  Reactive(Reactive<String>),
}

struct ReactiveStringVisitor;

impl<'de> serde::de::Visitor<'de> for ReactiveStringVisitor {
  type Value = ReactiveString;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a byte buffer representing a raw pointer")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(ReactiveString::Str(v.to_owned()))
  }

  fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(ReactiveString::Reactive(parse_bytes(value)))
  }
}

pub(crate) fn parse_bytes(value: &[u8]) -> Reactive<String> {
  assert_eq!(
    value.len(),
    std::mem::size_of::<u8>() + std::mem::size_of::<usize>() * 2,
    "Byte slice length is incorrect"
  );

  let identifier = value[0];
  assert_eq!(identifier, 0x00, "Identifier byte does not match expected value");

  let (_, pointer_bytes) = value.split_at(1);
  let (bytes1, bytes2) = pointer_bytes.split_at(std::mem::size_of::<usize>());

  let ptr1_value = usize::from_ne_bytes(bytes1.try_into().unwrap());
  let ptr2_value = usize::from_ne_bytes(bytes2.try_into().unwrap());

  let ptr1 = ptr1_value as *const Mutex<String>;
  let ptr2 = ptr2_value as *const Mutex<Option<Sender<()>>>;

  let raw_arc1 = unsafe { Arc::from_raw(ptr1) };
  let raw_arc2 = unsafe { Arc::from_raw(ptr2) };

  if Arc::strong_count(&raw_arc1) == 0 || Arc::strong_count(&raw_arc2) == 0 {
    panic!("we dropped this reactive... @codyduong make this more meaningful")
  }

  Reactive {
    value: raw_arc1,
    sender: raw_arc2,
  }
}

impl<'de> Deserialize<'de> for ReactiveString {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(ReactiveStringVisitor)
  }
}

impl ReactiveString {
  pub fn as_str(&self) -> Cow<str> {
    match self {
      ReactiveString::Str(str) => Cow::Borrowed(str),
      ReactiveString::Reactive(reactive) => {
        let value = reactive.value.lock().unwrap();
        Cow::Owned(value.clone())
      }
    }
  }
}

impl AsReactive<String> for ReactiveString {
  fn as_reactive(self, sender: impl Into<Option<Sender<()>>>) -> Reactive<String> {
    match self {
      ReactiveString::Str(str) => Reactive::<String> {
        value: Arc::new(Mutex::new(str.to_string())),
        sender: Arc::new(Mutex::new(sender.into())),
      },
      ReactiveString::Reactive(reactive) => {
        let mut sender_guard = reactive.sender.lock().unwrap();
        *sender_guard = sender.into();

        Reactive::<String> {
          value: Arc::clone(&reactive.value),
          sender: Arc::clone(&reactive.sender),
        }
      }
    }
  }
}

// impl From<Reactive<String>> for ReactiveString {
//   fn from(value: Reactive<String>) -> Self {
//     ReactiveString::Reactive(value)
//   }
// }

impl From<ReactiveString> for String {
  fn from(value: ReactiveString) -> Self {
    match value {
      ReactiveString::Str(str) => str,
      ReactiveString::Reactive(reactive) => reactive.get(),
    }
  }
}
