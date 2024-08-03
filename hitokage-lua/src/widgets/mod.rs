use clock::ClockUserData;
use hitokage_core::widgets::{
  clock::ClockMsg, r#box::BoxMsg, workspace::WorkspaceMsg, WidgetUserData as CoreWidgetUserData,
};
use mlua::{IntoLua, Lua};
use r#box::BoxUserData;
use workspace::WorkspaceUserData;

pub mod bar;
pub mod r#box;
pub mod clock;
pub mod workspace;

enum WidgetUserData {
  Clock(relm4::Sender<ClockMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
  Box(relm4::Sender<BoxMsg>),
}

impl<'lua> IntoLua<'lua> for WidgetUserData {
  fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
    match self {
      WidgetUserData::Clock(sender) => {
        let clock_userdata = ClockUserData {
          r#type: "Clock".to_string(),
          sender: sender,
        };
        lua.pack(clock_userdata)
      }
      WidgetUserData::Workspace(sender) => {
        let workspace_userdata = WorkspaceUserData {
          r#type: "Workspace".to_string(),
          sender: sender,
        };
        lua.pack(workspace_userdata)
      }
      WidgetUserData::Box(sender) => {
        let box_userdata = BoxUserData {
          r#type: "Box".to_string(),
          sender: sender,
        };
        lua.pack(box_userdata)
      }
    }
  }
}

impl From<CoreWidgetUserData> for WidgetUserData {
  fn from(sender: CoreWidgetUserData) -> Self {
    match sender {
      CoreWidgetUserData::Clock(sender) => WidgetUserData::Clock(sender),
      CoreWidgetUserData::Workspace(sender) => WidgetUserData::Workspace(sender),
      CoreWidgetUserData::Box(sender) => WidgetUserData::Box(sender),
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
    fn $fn_name(&self) -> Result<$ret, crate::HitokageError> {
      let sender = self.sender()?;

      let (tx, rx) = mpsc::channel::<_>();
      sender.send($msg_enum($request_enum(tx))).unwrap();
      let ret_val: $ret = rx.recv().unwrap().into();

      Ok(ret_val)
    }
  };
}

#[macro_export]
macro_rules! impl_setter_fn {
  ($fn_name:ident, $msg_enum:path, $request_enum:path, $from:ty) => {
    fn $fn_name(&self, lua: &Lua, value: mlua::Value) -> Result<(), mlua::Error> {
      let sender = self.sender()?;
      let value: $from = lua.from_value(value)?;

      sender.send($msg_enum($request_enum(value))).unwrap();

      Ok(())
    }
  };
}
