use mlua::{FromLua, Lua, LuaSerdeExt, MetaMethod, UserData, UserDataMethods, Value};
use serde::{Deserialize, Serialize, Serializer};
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

impl<T> FromLua for Reactive<T>
where
  T: Clone + Debug + Serialize + for<'de2> Deserialize<'de2> + std::marker::Send + mlua::FromLua + 'static,
{
  fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
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
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
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
      Value::String(k) => match k.to_str()?.as_ref() {
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
        Value::String(s) => match s.to_str()?.as_ref() {
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

pub(crate) trait AsReactive<T>
where
  T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
  fn as_reactive(self, sender: impl Into<Option<Sender<()>>>) -> Reactive<T>;
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
