use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::components::icon::IconMsg;
use hitokage_core::components::icon::IconMsgHook::BaseHook;
use hitokage_core::components::icon::IconMsgHook::{GetFile, GetFileReactive, SetFile};
use hitokage_core::structs::reactive::Reactive;
use hitokage_core::structs::Align;
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct IconUserData {
  pub r#type: String,
  pub sender: relm4::Sender<IconMsg>,
}

#[impl_lua_base(IconMsg::LuaHook)]
impl IconUserData {
  fn sender(&self) -> Result<relm4::Sender<IconMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_image, IconMsg::LuaHook, GetFile, String);
  impl_getter_fn!(get_image_reactive, IconMsg::LuaHook, GetFileReactive, Reactive<String>);
  impl_setter_fn!(set_image, IconMsg::LuaHook, SetFile, String);
}

#[impl_lua_base]
impl UserData for IconUserData {
  fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_image", |_, this, _: ()| Ok(this.get_image()?));
    methods.add_method("get_image_reactive", |_, this, _: ()| Ok(this.get_image_reactive()?));
    methods.add_method("set_image", |lua, this, value: mlua::Value| this.set_image(lua, value));

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()?.as_ref() {
            "type" => Ok(lua.to_value(&instance.r#type.clone())?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}
