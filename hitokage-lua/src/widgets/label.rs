use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::reactive::Reactive;
use hitokage_core::structs::Align;
use hitokage_core::widgets::label::LabelMsg;
use hitokage_core::widgets::label::LabelMsgHook::BaseHook;
use hitokage_core::widgets::label::LabelMsgHook::{GetLabel, GetLabelReactive, SetLabel};
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct LabelUserData {
  pub r#type: String,
  pub sender: relm4::Sender<LabelMsg>,
}

#[impl_lua_base(LabelMsg::LuaHook)]
impl LabelUserData {
  fn sender(&self) -> Result<relm4::Sender<LabelMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_label, LabelMsg::LuaHook, GetLabel, String);
  impl_getter_fn!(
    get_label_reactive,
    LabelMsg::LuaHook,
    GetLabelReactive,
    Reactive<String>
  );
  impl_setter_fn!(set_label, LabelMsg::LuaHook, SetLabel, String);
}

#[impl_lua_base]
impl UserData for LabelUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_label", |_, this, _: ()| Ok(this.get_label()?));
    methods.add_method("get_label_reactive", |_, this, _: ()| Ok(this.get_label_reactive()?));
    methods.add_method("set_label", |lua, this, value: mlua::Value| this.set_label(lua, value));

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
