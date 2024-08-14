use clock::ClockUserData;
use cpu::CpuUserData;
use hitokage_core::widgets::{
  clock::ClockMsg, cpu::CpuMsg, icon::IconMsg, label::LabelMsg, r#box::BoxMsg, workspace::WorkspaceMsg,
  WidgetUserData as CoreWidgetUserData,
};
use icon::IconUserData;
use label::LabelUserData;
use mlua::{IntoLua, Lua};
use r#box::BoxUserData;
use std::sync::Arc;
use workspace::WorkspaceUserData;

pub mod bar;
pub mod r#box;
pub mod clock;
pub mod cpu;
pub mod icon;
pub mod label;
pub mod workspace;

enum WidgetUserData {
  Box(relm4::Sender<BoxMsg>),
  Clock(relm4::Sender<ClockMsg>),
  Cpu(relm4::Sender<CpuMsg>),
  Icon(relm4::Sender<IconMsg>),
  Label(relm4::Sender<LabelMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
}

impl<'lua> IntoLua<'lua> for WidgetUserData {
  fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
    match self {
      WidgetUserData::Box(sender) => {
        let box_userdata = BoxUserData {
          r#type: "Box".to_string(),
          sender,
        };
        lua.pack(box_userdata)
      }
      WidgetUserData::Clock(sender) => {
        let clock_userdata = ClockUserData {
          r#type: "Clock".to_string(),
          sender,
        };
        lua.pack(clock_userdata)
      }
      WidgetUserData::Cpu(sender) => {
        let clock_userdata = CpuUserData {
          r#type: "Clock".to_string(),
          sender,
        };
        lua.pack(clock_userdata)
      }
      WidgetUserData::Icon(sender) => {
        let image_userdata = IconUserData {
          r#type: "Icon".to_string(),
          sender,
        };
        lua.pack(image_userdata)
      }
      WidgetUserData::Label(sender) => {
        let label_userdata = LabelUserData {
          r#type: "Label".to_string(),
          sender,
        };
        lua.pack(label_userdata)
      }
      WidgetUserData::Workspace(sender) => {
        let workspace_userdata = WorkspaceUserData {
          r#type: "Workspace".to_string(),
          sender,
        };
        lua.pack(workspace_userdata)
      }
    }
  }
}

impl From<CoreWidgetUserData> for WidgetUserData {
  fn from(sender: CoreWidgetUserData) -> Self {
    match sender {
      CoreWidgetUserData::Box(sender) => WidgetUserData::Box(sender),
      CoreWidgetUserData::Clock(sender) => WidgetUserData::Clock(sender),
      CoreWidgetUserData::Cpu(sender) => WidgetUserData::Cpu(sender),
      CoreWidgetUserData::Icon(sender) => WidgetUserData::Icon(sender),
      CoreWidgetUserData::Label(sender) => WidgetUserData::Label(sender),
      CoreWidgetUserData::Workspace(sender) => WidgetUserData::Workspace(sender),
    }
  }
}

struct WidgetUserDataVec(Vec<WidgetUserData>);

impl From<Vec<CoreWidgetUserData>> for WidgetUserDataVec {
  fn from(value: Vec<CoreWidgetUserData>) -> Self {
    value.into_iter().map(|o| o.into()).collect()
  }
}

impl FromIterator<WidgetUserData> for WidgetUserDataVec {
  fn from_iter<I: IntoIterator<Item = WidgetUserData>>(iter: I) -> Self {
    let mut vec = Vec::new();
    for item in iter {
      vec.push(item);
    }
    WidgetUserDataVec(vec)
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
    fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum($request_enum(tx))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, $ret:ty) => {
    fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
      use std::sync::mpsc;

      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum1($msg_enum2($request_enum(tx)))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $msg_enum3:path, $request_enum:path, $ret:ty) => {
    fn $fn_name(&self) -> Result<$ret, $crate::HitokageError> {
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

pub fn convert_variadic_to_vec<'lua, T>(
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
    fn $fn_name(&self, lua: &mlua::Lua, args: mlua::Variadic<Value>) -> Result<(), mlua::Error> {
      use crate::widgets::convert_variadic_to_vec;

      let sender = self.sender()?;
      let value: Vec<$from> = convert_variadic_to_vec(lua, args, stringify!($fn_name), stringify!($from))?;

      sender.send($msg_enum($request_enum(value))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum:path, $request_enum:path, $from:ty) => {
    fn $fn_name(&self, lua: &mlua::Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender.send($msg_enum($request_enum(value))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, Vec<$from:ty>) => {
    fn $fn_name(&self, lua: &mlua::Lua, args: mlua::Variadic<Value>) -> Result<(), mlua::Error> {
      use crate::widgets::convert_variadic_to_vec;

      let sender = self.sender()?;
      let value: Vec<$from> = convert_variadic_to_vec(lua, args, stringify!($fn_name), stringify!($from))?;

      sender.send($msg_enum1($msg_enum2($request_enum(value)))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $request_enum:path, $from:ty) => {
    fn $fn_name(&self, lua: &mlua::Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender.send($msg_enum1($msg_enum2($request_enum(value)))).unwrap();

      Ok(())
    }
  };
  ($fn_name:ident, $msg_enum1:path, $msg_enum2:path, $msg_enum3:path, $request_enum:path, Vec<$from:ty>) => {
    fn $fn_name(&self, lua: &mlua::Lua, args: mlua::Variadic<Value>) -> Result<(), mlua::Error> {
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
    fn $fn_name(&self, lua: &mlua::Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender
        .send($msg_enum1($msg_enum2($msg_enum3($request_enum(value)))))
        .unwrap();

      Ok(())
    }
  };
}
