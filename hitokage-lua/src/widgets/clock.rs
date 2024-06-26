use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::widgets::clock::ClockMsg;
use hitokage_core::widgets::clock::ClockMsgHook::{GetFormat, SetFormat};
use mlua::Lua;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct ClockUserData {
  pub r#type: String,
  pub sender: relm4::Sender<ClockMsg>,
}

impl ClockUserData {
  fn sender(&self) -> Result<relm4::Sender<ClockMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_format, ClockMsg::LuaHook, GetFormat, String);
  impl_setter_fn!(set_format, ClockMsg::LuaHook, SetFormat, String);
}

impl UserData for ClockUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_format", |_, this, _: ()| Ok(this.get_format()?));

    methods.add_method("set_format", |lua, this, value: mlua::Value| {
      Ok(this.set_format(lua, value)?)
    });

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "type" => Ok(lua.to_value(&instance.r#type.clone())?),
            "format" => Ok(lua.to_value(&instance.get_format()?)?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}
