use super::WidgetUserDataVec;
use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::CssClass;
use hitokage_core::widgets::r#box::BoxMsg;
use hitokage_core::widgets::r#box::BoxMsgHook::{GetClass, GetWidgets, SetClass};
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct BoxUserData {
  pub r#type: String,
  pub sender: relm4::Sender<BoxMsg>,
}

impl BoxUserData {
  fn sender(&self) -> Result<relm4::Sender<BoxMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_class, BoxMsg::LuaHook, GetClass, Vec<String>);
  impl_setter_fn!(set_class, BoxMsg::LuaHook, SetClass, Option<CssClass>);

  impl_getter_fn!(get_widgets, BoxMsg::LuaHook, GetWidgets, WidgetUserDataVec);
}

impl UserData for BoxUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_class", |lua, instance, ()| Ok(lua.pack(instance.get_class()?)?));
    methods.add_method("set_class", |lua, this, value: mlua::Value| {
      Ok(this.set_class(lua, value)?)
    });

    methods.add_method("get_widgets", |lua, instance, ()| {
      Ok(lua.pack(instance.get_widgets()?)?)
    });

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "type" => Ok(lua.to_value(&instance.r#type.clone())?),
            "widgets" => Ok(lua.pack(instance.get_widgets()?)?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}
