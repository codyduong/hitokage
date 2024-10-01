use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::components::clock::ClockMsg;
use hitokage_core::components::clock::ClockMsgHook::BaseHook;
use hitokage_core::components::clock::ClockMsgHook::{GetFormat, GetFormatReactive, SetFormat};
use hitokage_core::structs::reactive::Reactive;
use hitokage_core::structs::Align;
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct ClockUserData {
  pub r#type: String,
  pub sender: relm4::Sender<ClockMsg>,
}

#[impl_lua_base(ClockMsg::LuaHook)]
impl ClockUserData {
  fn sender(&self) -> Result<relm4::Sender<ClockMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_format, ClockMsg::LuaHook, GetFormat, String);
  impl_getter_fn!(
    get_format_reactive,
    ClockMsg::LuaHook,
    GetFormatReactive,
    Reactive<String>
  );
  impl_setter_fn!(set_format, ClockMsg::LuaHook, SetFormat, String);
}

#[impl_lua_base]
impl UserData for ClockUserData {
  fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_format", |_, this, _: ()| Ok(this.get_format()?));
    methods.add_method("get_format_reactive", |_, this: &ClockUserData, _: ()| {
      Ok(this.get_format_reactive()?)
    });
    methods.add_method("set_format", |lua, this, value: mlua::Value| {
      this.set_format(lua, value)
    });

    methods.add_meta_method::<_, mlua::String, _>(
      "__index",
      |lua, instance, key| -> Result<mlua::Value, mlua::Error> {
        match key.to_str()?.as_ref() {
          "type" => Ok(lua.to_value(&instance.r#type.clone())?),
          _ => Ok(Value::Nil),
        }
      },
    );
  }
}
