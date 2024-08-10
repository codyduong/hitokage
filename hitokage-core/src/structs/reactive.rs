use mlua::{FromLua, Lua, LuaSerdeExt, UserData, UserDataMethods, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Reactive<T>
where
  T: Serialize + for<'de> Deserialize<'de> + Clone,
{
  // @codyduong todo remove pub
  pub value: Arc<Mutex<T>>,
}

impl<T> Reactive<T>
where
  T: Serialize + for<'de> Deserialize<'de> + Clone,
{
  pub fn new(t: T) -> Self {
    Reactive {
      value: Arc::new(Mutex::new(t)),
    }
  }

  pub fn into_inner(self) -> T {
    let value = self.value.lock().unwrap();
    value.clone()
  }
}

impl<T> Serialize for Reactive<T>
where
  T: Serialize + Clone + for<'de> Deserialize<'de> + std::marker::Send + 'static,
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

// @codyduong this code is definetly wrong, but we can bypass it with our LuaSerializer, so idk maybe impl one day LOL!
// impl<'de, T> Deserialize<'de> for Reactive<T>
// where
//   T: Serialize + Clone + for<'de2> Deserialize<'de2> + Send + 'static,
// {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'de>,
//   {
//     let lua = Lua::new();
//     log::error!("visited sex god");
//     let lua_data: String = Deserialize::deserialize(deserializer)?;
//     let lua_value: Value = lua.load(&lua_data).eval().map_err(serde::de::Error::custom)?;
//     let unpacked: Self = lua.from_value(lua_value).map_err(serde::de::Error::custom)?;
//     Ok(unpacked)
//   }
// }

impl<'lua, T> FromLua<'lua> for Reactive<T>
where
  T: Serialize + Clone + for<'de2> Deserialize<'de2> + std::marker::Send + for<'lua2> mlua::FromLua<'lua2> + 'static,
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
  T: Serialize + for<'de> Deserialize<'de> + Clone + Send + 'static,
{
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get", |lua, this, _: ()| {
      let value = this.value.lock().unwrap();

      // log::info!(
      //   "lua got arc at: {:?}",
      //   std::sync::Arc::<std::sync::Mutex<T>>::as_ptr(&this.value)
      // );

      let lua_value: Value = lua.to_value(&*value)?;
      Ok(lua_value)
    });

    methods.add_method_mut("set", |lua, this, new_value: Value| {
      let mut value = this.value.lock().unwrap();

      // log::info!(
      //   "lua set arc at: {:?}",
      //   std::sync::Arc::<std::sync::Mutex<T>>::as_ptr(&this.value)
      // );

      // Deserialize and validate the new value
      let deserialized_value: T = lua.from_value(new_value.clone())?;
      *value = deserialized_value.clone();

      // Serialize back to Lua value to ensure it's valid
      // let lua_value: Value = lua.to_value(&deserialized_value)?;
      // println!("Value updated to: {:?}", lua_value); // Output the changes
      Ok(())
    });
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
    // Convert the byte buffer back to a usize, then to a raw pointer
    let mut array = [0u8; std::mem::size_of::<usize>()];
    array.copy_from_slice(value);
    let ptr_value = usize::from_ne_bytes(array);
    let ptr = ptr_value as *const std::sync::Mutex<std::string::String>;

    let raw_arc = unsafe { Arc::from_raw(ptr) };

    let strong_count = Arc::strong_count(&raw_arc);

    if strong_count == 0 {
      panic!("we dropped this reactive... @codyduong make this more meaningful")
    }

    // @codyduong TODO this is cursed...
    Ok(ReactiveString::Reactive(Reactive { value: raw_arc }))
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

impl From<Reactive<String>> for ReactiveString {
  fn from(value: Reactive<String>) -> Self {
    ReactiveString::Reactive(value)
  }
}

impl From<ReactiveString> for Reactive<String> {
  fn from(value: ReactiveString) -> Self {
    match value {
      ReactiveString::Str(str) => Reactive::new(str),
      ReactiveString::Reactive(reactive) => reactive,
    }
  }
}

impl From<ReactiveString> for String {
  fn from(value: ReactiveString) -> Self {
    match value {
      ReactiveString::Str(str) => str,
      ReactiveString::Reactive(reactive) => reactive.into_inner(),
    }
  }
}
