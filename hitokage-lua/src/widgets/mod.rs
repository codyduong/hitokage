use clock::ClockUserData;
use cpu::CpuUserData;
use hitokage_core::widgets::WidgetUserData as CoreWidgetUserData;
use icon::IconUserData;
use label::LabelUserData;
use memory::MemoryUserData;
use mlua::{IntoLua, Lua};
use r#box::BoxUserData;
use std::{collections::VecDeque, sync::Arc};
use workspace::WorkspaceUserData;

pub mod bar;
pub mod r#box;
pub mod clock;
pub mod cpu;
pub mod icon;
pub mod label;
pub mod memory;
pub mod workspace;

pub(crate) enum WidgetUserData {
  Box(BoxUserData),
  Clock(ClockUserData),
  Cpu(CpuUserData),
  Icon(IconUserData),
  Label(LabelUserData),
  Memory(MemoryUserData),
  Workspace(WorkspaceUserData),
}

impl WidgetUserData {
  fn get_id(&self) -> Option<String> {
    match self {
      WidgetUserData::Box(userdata) => userdata.get_id().unwrap(),
      WidgetUserData::Clock(userdata) => userdata.get_id().unwrap(),
      WidgetUserData::Cpu(userdata) => userdata.get_id().unwrap(),
      WidgetUserData::Icon(userdata) => userdata.get_id().unwrap(),
      WidgetUserData::Label(userdata) => userdata.get_id().unwrap(),
      WidgetUserData::Memory(userdata) => userdata.get_id().unwrap(),
      WidgetUserData::Workspace(userdata) => userdata.get_id().unwrap(),
    }
  }
}

impl<'lua> IntoLua<'lua> for WidgetUserData {
  fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
    match self {
      WidgetUserData::Box(userdata) => lua.pack(userdata),
      WidgetUserData::Clock(userdata) => lua.pack(userdata),
      WidgetUserData::Cpu(userdata) => lua.pack(userdata),
      WidgetUserData::Icon(userdata) => lua.pack(userdata),
      WidgetUserData::Label(userdata) => lua.pack(userdata),
      WidgetUserData::Memory(userdata) => lua.pack(userdata),
      WidgetUserData::Workspace(userdata) => lua.pack(userdata),
    }
  }
}

impl From<CoreWidgetUserData> for WidgetUserData {
  fn from(sender: CoreWidgetUserData) -> Self {
    match sender {
      CoreWidgetUserData::Box(sender) => WidgetUserData::Box(BoxUserData {
        r#type: "Box".to_string(),
        sender,
      }),
      CoreWidgetUserData::Clock(sender) => WidgetUserData::Clock(ClockUserData {
        r#type: "Clock".to_string(),
        sender,
      }),
      CoreWidgetUserData::Cpu(sender) => WidgetUserData::Cpu(CpuUserData {
        r#type: "Cpu".to_string(),
        sender,
      }),
      CoreWidgetUserData::Icon(sender) => WidgetUserData::Icon(IconUserData {
        r#type: "Icon".to_string(),
        sender,
      }),
      CoreWidgetUserData::Label(sender) => WidgetUserData::Label(LabelUserData {
        r#type: "Label".to_string(),
        sender,
      }),
      CoreWidgetUserData::Memory(sender) => WidgetUserData::Memory(MemoryUserData {
        r#type: "Memory".to_string(),
        sender,
      }),
      CoreWidgetUserData::Workspace(sender) => WidgetUserData::Workspace(WorkspaceUserData {
        r#type: "Workspace".to_string(),
        sender,
      }),
    }
  }
}

pub(crate) struct WidgetUserDataVec(Vec<WidgetUserData>);

impl From<Vec<CoreWidgetUserData>> for WidgetUserDataVec {
  fn from(value: Vec<CoreWidgetUserData>) -> Self {
    value.into_iter().map(|o| o.into()).collect()
  }
}

impl FromIterator<WidgetUserData> for WidgetUserDataVec {
  fn from_iter<I: IntoIterator<Item = WidgetUserData>>(iter: I) -> Self {
    WidgetUserDataVec(iter.into_iter().collect())
  }
}

impl IntoIterator for WidgetUserDataVec {
  type Item = WidgetUserData;
  type IntoIter = std::vec::IntoIter<WidgetUserData>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl From<WidgetUserDataVec> for Vec<WidgetUserData> {
  fn from(value: WidgetUserDataVec) -> Self {
    value.0
  }
}

impl<'lua> IntoLua<'lua> for WidgetUserDataVec {
  fn into_lua(self, lua: &'lua Lua) -> Result<mlua::Value<'lua>, mlua::Error> {
    lua.pack(self.0)
  }
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
  args: mlua::Variadic<mlua::Value<'lua>>,
  name: &str,
  t: &str,
) -> mlua::Result<Vec<T>>
where
  T: mlua::FromLua<'lua>,
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
            to: "Array or Element",
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
      use crate::widgets::convert_variadic_to_vec;

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
      use crate::widgets::convert_variadic_to_vec;

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
      use crate::widgets::convert_variadic_to_vec;

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
