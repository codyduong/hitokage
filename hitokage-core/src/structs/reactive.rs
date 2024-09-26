use mlua::{FromLua, Lua, LuaSerdeExt, MetaMethod, UserData, UserDataMethods, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Reactive<T>
where
  T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
  pub value: Arc<Mutex<T>>,
  pub sender: Arc<Mutex<Option<Sender<()>>>>,
}

impl<T> Reactive<T>
where
  T: Clone + Debug + Serialize + for<'de> Deserialize<'de> + std::cmp::PartialEq,
{
  pub fn new(t: T) -> Self {
    Reactive {
      value: Arc::new(Mutex::new(t)),
      sender: Arc::new(Mutex::new(None)),
    }
  }

  pub fn get(&self) -> T {
    self.value.lock().unwrap().clone()
  }

  pub fn set(&self, value: impl Into<T>) {
    let v_set = value.into();
    let diff = v_set != *self.value.lock().unwrap();

    if diff {
      let mut v = self.value.lock().unwrap();
      if let Some(sender) = self.sender.lock().unwrap().clone() {
        // only send a message to update if we are actually different
        let _ = sender.send(());
      }

      *v = v_set;
    }
  }
}

impl<T> Serialize for Reactive<T>
where
  T: Clone + Debug + Serialize + for<'de> Deserialize<'de> + Send + 'static + std::cmp::PartialEq,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let lua = Lua::new();
    let packed = lua.pack(self.clone()).map_err(serde::ser::Error::custom)?;
    packed.serialize(serializer)
  }
}

impl<'lua, T> FromLua<'lua> for Reactive<T>
where
  T: Clone
    + Debug
    + Serialize
    + for<'de2> Deserialize<'de2>
    + std::marker::Send
    + for<'lua2> mlua::FromLua<'lua2>
    + 'static,
{
  fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
    match value {
      Value::UserData(ud) => {
        let foo = ud.borrow::<Reactive<T>>()?;
        Ok(foo.to_owned())
      }
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "Reactive",
        message: None,
      }),
    }
  }
}

impl<T> UserData for Reactive<T>
where
  T: Clone + Debug + Serialize + for<'de> Deserialize<'de> + Send + 'static + std::cmp::PartialEq,
{
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get", |lua, this, _: ()| {
      let lua_value: Value = lua.to_value(&this.get())?;
      Ok(lua_value)
    });

    methods.add_method_mut("set", |lua, this, new_value: Value| {
      let deserialized_value: T = lua.from_value(new_value.clone())?;
      this.set(deserialized_value);
      Ok(())
    });

    methods.add_meta_method(MetaMethod::Index, |lua, instance, key| match key {
      Value::String(k) => match k.to_str()? {
        "value" => {
          let value = instance.value.lock().unwrap();
          Ok(lua.to_value(&*value)?)
        }
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    });

    methods.add_meta_method(MetaMethod::NewIndex, |lua, instance, args: mlua::Variadic<Value>| {
      if args.len() > 2 {
        return Err(mlua::Error::BindError);
      }

      let key = args.get(0).unwrap();
      let val = args.get(1).unwrap();

      match key {
        Value::String(s) => match s.to_str()? {
          "value" => {
            let deserialized_value: T = match lua.from_value(val.clone()) {
              Ok(o) => o,
              Err(e) => {
                return Err(mlua::Error::WithContext {
                  context: "Failed to modify reactive value".into(),
                  cause: Arc::new(e),
                })
              }
            };
            let mut value = instance.value.lock().unwrap();
            *value = deserialized_value.clone();
            return Ok(());
          }
          _ => (),
        },
        _ => (),
      };
      Err(mlua::Error::RuntimeError("Attempt to modify readonly field".into()))
    });
  }
}

impl<T> PartialEq for Reactive<T>
where
  T: Clone + Debug + Serialize + for<'de> Deserialize<'de> + std::cmp::PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    *self.value.lock().unwrap() == *other.value.lock().unwrap()
  }

  fn ne(&self, other: &Self) -> bool {
    !self.eq(other)
  }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
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

    // @codyduong TODO this is cursed...
    Ok(ReactiveString::Reactive(Reactive {
      value: raw_arc1,
      sender: raw_arc2,
    }))
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

  pub fn as_reactive_string(&self, sender: impl Into<Option<Sender<()>>>) -> Reactive<String> {
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

impl From<Reactive<String>> for ReactiveString {
  fn from(value: Reactive<String>) -> Self {
    ReactiveString::Reactive(value)
  }
}

impl From<ReactiveString> for String {
  fn from(value: ReactiveString) -> Self {
    match value {
      ReactiveString::Str(str) => str,
      ReactiveString::Reactive(reactive) => reactive.get(),
    }
  }
}

pub fn create_react_sender<T: 'static + Clone + Debug>(relm_sender: &relm4::Sender<T>, msg: T) -> Sender<()> {
  let (tx, rx) = std::sync::mpsc::channel::<()>();

  let relm_sender = relm_sender.clone();
  glib::timeout_add_local(std::time::Duration::from_millis(100), move || match rx.try_recv() {
    Ok(_) => {
      match relm_sender.send(msg.clone()) {
        Ok(_) => {}
        Err(err) => {
          log::error!("Reactive watcher failed to send relm4 message: {:?}", err);
        }
      };
      glib::ControlFlow::Continue
    }
    Err(error) => match error {
      std::sync::mpsc::TryRecvError::Empty => glib::ControlFlow::Continue,
      std::sync::mpsc::TryRecvError::Disconnected => {
        // log::info!("Reactive watcher dropped");
        glib::ControlFlow::Break
      }
    },
  });

  tx
}
