pub mod bar;
pub mod r#box;
pub mod clock;
pub mod workspace;

use clock::{Clock, ClockMsg};
use mlua::{LuaSerdeExt, Value};
use mlua::{IntoLua, Lua, UserData, UserDataMethods};
use r#box::Box;
use relm4::component::Connector;
use relm4::ComponentController;
use relm4::Controller;
use serde::{Deserialize, Serialize};
use workspace::{Workspace, WorkspaceMsg};

#[derive(Debug, Deserialize, Serialize)]
pub enum WidgetProps {
  Clock(clock::ClockProps),
  Workspace(workspace::WorkspaceProps),
  Box(r#box::BoxProps),
}

pub enum WidgetController {
  Clock(Controller<Clock>),
  Workspace(Controller<Workspace>),
  Box(Controller<Box>),
}

#[derive(Debug, Clone)]
pub enum WidgetSender {
  Clock(relm4::Sender<ClockMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
  Box(relm4::Sender<()>),
}

#[derive(Debug, Clone)]
pub struct ClockUserData {
  r#type: String,
}

#[derive(Debug, Clone)]
pub struct WorkspaceUserData {
  r#type: String,
}

#[derive(Debug, Clone)]
pub struct BoxUserData {
  r#type: String,
}

impl UserData for ClockUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "type" => Ok(lua.to_value(&instance.r#type.clone())?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}

impl UserData for WorkspaceUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "type" => Ok(lua.to_value(&instance.r#type.clone())?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}

impl UserData for BoxUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "type" => Ok(lua.to_value(&instance.r#type.clone())?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}

impl<'lua> IntoLua<'lua> for WidgetSender {
  fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
    match self {
      WidgetSender::Clock(_) => {
        let clock_userdata = ClockUserData {
          r#type: "Clock".to_string(),
        };
        lua.pack(clock_userdata)
      }
      WidgetSender::Workspace(_) => {
        let workspace_userdata = WorkspaceUserData {
          r#type: "Workspace".to_string(),
        };
        lua.pack(workspace_userdata)
      }
      WidgetSender::Box(_) => {
        let box_userdata = BoxUserData {
          r#type: "Box".to_string(),
        };
        lua.pack(box_userdata)
      }
    }
  }
}

impl<'a> From<&'a WidgetController> for WidgetSender {
  fn from(controller: &'a WidgetController) -> Self {
    match controller {
      WidgetController::Clock(item) => WidgetSender::Clock(item.sender().clone()),
      WidgetController::Workspace(item) => WidgetSender::Workspace(item.sender().clone()),
      WidgetController::Box(item) => WidgetSender::Box(item.sender().clone()),
    }
  }
}
