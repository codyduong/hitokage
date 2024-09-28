use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::components::cpu::CpuMsg;
use hitokage_core::components::cpu::CpuMsgHook::BaseHook;
use hitokage_core::components::cpu::CpuMsgHook::{GetFormat, GetFormatReactive, SetFormat};
use hitokage_core::structs::reactive::Reactive;
use hitokage_core::structs::Align;
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct CpuUserData {
  pub r#type: String,
  pub sender: relm4::Sender<CpuMsg>,
}

#[impl_lua_base(CpuMsg::LuaHook)]
impl CpuUserData {
  fn sender(&self) -> Result<relm4::Sender<CpuMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_format, CpuMsg::LuaHook, GetFormat, String);
  impl_getter_fn!(
    get_format_reactive,
    CpuMsg::LuaHook,
    GetFormatReactive,
    Reactive<String>
  );
  impl_setter_fn!(set_format, CpuMsg::LuaHook, SetFormat, String);
}

#[impl_lua_base]
impl UserData for CpuUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_format", |_, this, _: ()| Ok(this.get_format()?));
    methods.add_method("get_format_reactive", |_, this, _: ()| Ok(this.get_format_reactive()?));
    methods.add_method("set_format", |lua, this, value: mlua::Value| {
      this.set_format(lua, value)
    });

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