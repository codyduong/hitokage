use battery::BatteryUserData;
use clock::ClockUserData;
use cpu::CpuUserData;
use hitokage_core::components::ChildUserData as CoreChildUserData;
use icon::IconUserData;
use label::LabelUserData;
use memory::MemoryUserData;
use mlua::{IntoLua, Lua};
use r#box::BoxUserData;
use std::sync::Arc;
use weather::WeatherUserData;
use workspace::WorkspaceUserData;

pub mod bar;
pub mod battery;
pub mod r#box;
pub mod clock;
pub mod cpu;
pub mod icon;
pub mod label;
pub mod memory;
pub mod weather;
pub mod workspace;

pub(crate) enum ChildUserData {
  Battery(BatteryUserData),
  Box(BoxUserData),
  Clock(ClockUserData),
  Cpu(CpuUserData),
  Icon(IconUserData),
  Label(LabelUserData),
  Memory(MemoryUserData),
  Weather(WeatherUserData),
  Workspace(WorkspaceUserData),
}

impl ChildUserData {
  fn get_id(&self) -> Option<String> {
    match self {
      ChildUserData::Battery(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Box(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Clock(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Cpu(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Icon(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Label(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Memory(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Weather(userdata) => userdata.get_id().unwrap(),
      ChildUserData::Workspace(userdata) => userdata.get_id().unwrap(),
    }
  }
}

impl IntoLua for ChildUserData {
  fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
    match self {
      ChildUserData::Battery(userdata) => lua.pack(userdata),
      ChildUserData::Box(userdata) => lua.pack(userdata),
      ChildUserData::Clock(userdata) => lua.pack(userdata),
      ChildUserData::Cpu(userdata) => lua.pack(userdata),
      ChildUserData::Icon(userdata) => lua.pack(userdata),
      ChildUserData::Label(userdata) => lua.pack(userdata),
      ChildUserData::Memory(userdata) => lua.pack(userdata),
      ChildUserData::Weather(userdata) => lua.pack(userdata),
      ChildUserData::Workspace(userdata) => lua.pack(userdata),
    }
  }
}

impl From<CoreChildUserData> for ChildUserData {
  fn from(sender: CoreChildUserData) -> Self {
    match sender {
      CoreChildUserData::Battery(sender) => ChildUserData::Battery(BatteryUserData {
        r#type: "Battery".to_string(),
        sender,
      }),
      CoreChildUserData::Box(sender) => ChildUserData::Box(BoxUserData {
        r#type: "Box".to_string(),
        sender,
      }),
      CoreChildUserData::Clock(sender) => ChildUserData::Clock(ClockUserData {
        r#type: "Clock".to_string(),
        sender,
      }),
      CoreChildUserData::Cpu(sender) => ChildUserData::Cpu(CpuUserData {
        r#type: "Cpu".to_string(),
        sender,
      }),
      CoreChildUserData::Icon(sender) => ChildUserData::Icon(IconUserData {
        r#type: "Icon".to_string(),
        sender,
      }),
      CoreChildUserData::Label(sender) => ChildUserData::Label(LabelUserData {
        r#type: "Label".to_string(),
        sender,
      }),
      CoreChildUserData::Memory(sender) => ChildUserData::Memory(MemoryUserData {
        r#type: "Memory".to_string(),
        sender,
      }),
      CoreChildUserData::Weather(sender) => ChildUserData::Weather(WeatherUserData {
        r#type: "Weather".to_string(),
        sender,
      }),
      CoreChildUserData::Workspace(sender) => ChildUserData::Workspace(WorkspaceUserData {
        r#type: "Workspace".to_string(),
        sender,
      }),
    }
  }
}

pub(crate) struct ChildUserDataVec(Vec<ChildUserData>);

impl From<Vec<CoreChildUserData>> for ChildUserDataVec {
  fn from(value: Vec<CoreChildUserData>) -> Self {
    value.into_iter().map(|o| o.into()).collect()
  }
}

impl FromIterator<ChildUserData> for ChildUserDataVec {
  fn from_iter<I: IntoIterator<Item = ChildUserData>>(iter: I) -> Self {
    ChildUserDataVec(iter.into_iter().collect())
  }
}

impl IntoIterator for ChildUserDataVec {
  type Item = ChildUserData;
  type IntoIter = std::vec::IntoIter<ChildUserData>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl From<ChildUserDataVec> for Vec<ChildUserData> {
  fn from(value: ChildUserDataVec) -> Self {
    value.0
  }
}

impl IntoLua for ChildUserDataVec {
  fn into_lua(self, lua: &Lua) -> Result<mlua::Value, mlua::Error> {
    lua.pack(self.0)
  }
}

pub(crate) trait HoldsChildren {
  fn get_children(&self) -> Result<ChildUserDataVec, crate::HitokageError>;
}

#[macro_export]
macro_rules! impl_getter_fn {
  ($fn_name:ident, $msg_enum:path, $request_enum:path, $ret:ty) => {
    pub(crate) fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum($request_enum(tx))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
  ($vis:vis, $fn_name:ident, $msg_enum:path, $request_enum:path, $ret:ty) => {
    $vis fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum($request_enum(tx))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, $ret:ty) => {
    pub(crate) fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum1($msg_enum2($request_enum(tx)))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
  ($vis:vis, $fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, $ret:ty) => {
    $vis fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum1($msg_enum2($request_enum(tx)))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $msg_enum3:path, $request_enum:path, $ret:ty) => {
    pub(crate) fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender
        .send($msg_enum1($msg_enum2($msg_enum3($request_enum(tx)))))
        .unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
}

// use mlua::LuaSerdeExt;

pub(crate) fn convert_variadic_to_vec<'lua, T>(
  lua: &'lua mlua::Lua,
  args: mlua::Variadic<mlua::Value>,
  name: &str,
  t: &str,
) -> mlua::Result<Vec<T>>
where
  T: mlua::FromLua,
{
  let mut vec = Vec::with_capacity(args.len());

  if let Some(first_arg) = args.get(0) {
    if let mlua::Value::Table(table) = first_arg.clone() {
      if table.raw_len() > 0 {
        if args.len() > 1 {
          return Err(mlua::Error::RuntimeError(
            "Extra arguments are not allowed when the first argument is a sequence".into(),
          ));
        }

        for pair in table.sequence_values::<T>() {
          vec.push(pair?);
        }

        return Ok(vec);
      } else {
        return Err(mlua::Error::BadArgument {
          to: Some(name.to_owned()),
          pos: 0,
          name: None,
          cause: Arc::new(mlua::Error::FromLuaConversionError {
            from: first_arg.type_name(),
            to: "Array or Element".to_string(),
            message: Some(format!("Expected an Array or {:}, not {:}", t, first_arg.type_name())),
          }),
        });
      }
    }
  }

  for arg in args {
    let item: T = match T::from_lua(arg, lua) {
      Ok(value) => value,
      Err(_) => {
        return Err(mlua::Error::RuntimeError(
          "Failed to convert argument to the expected type".into(),
        ))
      }
    };
    vec.push(item);
  }

  Ok(vec)
}

#[macro_export]
macro_rules! impl_setter_fn {
  ($fn_name:ident, $msg_enum:path, $request_enum:path, Vec<$from:ty>) => {
    pub(crate) fn $fn_name(&self, lua: &mlua::Lua, args: mlua::Variadic<Value>) -> Result<(), mlua::Error> {
      use crate::components::convert_variadic_to_vec;

      let sender = self.sender()?;
      let value: Vec<$from> = convert_variadic_to_vec(lua, args, stringify!($fn_name), stringify!($from))?;

      sender.send($msg_enum($request_enum(value))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum:path, $request_enum:path, $from:ty) => {
    pub(crate) fn $fn_name(&self, lua: &mlua::Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender.send($msg_enum($request_enum(value))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, Vec<$from:ty>) => {
    pub(crate) fn $fn_name(&self, lua: &mlua::Lua, args: mlua::Variadic<Value>) -> Result<(), mlua::Error> {
      use crate::components::convert_variadic_to_vec;

      let sender = self.sender()?;
      let value: Vec<$from> = convert_variadic_to_vec(lua, args, stringify!($fn_name), stringify!($from))?;

      sender.send($msg_enum1($msg_enum2($request_enum(value)))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, $from:ty) => {
    pub(crate) fn $fn_name(&self, lua: &mlua::Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender.send($msg_enum1($msg_enum2($request_enum(value)))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $msg_enum3:path, $request_enum:path, Vec<$from:ty>) => {
    pub(crate) fn $fn_name(&self, lua: &mlua::Lua, args: mlua::Variadic<Value>) -> Result<(), mlua::Error> {
      use crate::components::convert_variadic_to_vec;

      let sender = self.sender()?;
      let value: Vec<$from> = convert_variadic_to_vec(lua, args, stringify!($fn_name), stringify!($from))?;

      sender
        .send($msg_enum1($msg_enum2($msg_enum3($request_enum(value)))))
        .unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $msg_enum3:path, $request_enum:path, $from:ty) => {
    pub(crate) fn $fn_name(&self, lua: &mlua::Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender
        .send($msg_enum1($msg_enum2($msg_enum3($request_enum(value)))))
        .unwrap();

      Ok(())
    }
  };
}
