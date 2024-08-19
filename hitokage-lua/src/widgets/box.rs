use super::WidgetUserDataVec;
use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::Align;
use hitokage_core::widgets::base::BaseMsgHook::{
  GetClass, GetHalign, GetHeight, GetHeightRequest, GetHexpand, GetSizeRequest, GetValign, GetVexpand, GetWidth,
  GetWidthRequest, SetClass, SetHalign, SetHeightRequest, SetHexpand, SetSizeRequest, SetValign, SetVexpand,
  SetWidthRequest,
};
use hitokage_core::widgets::r#box::BoxMsg;
use hitokage_core::widgets::r#box::BoxMsgHook::BaseHook;
use hitokage_core::widgets::r#box::BoxMsgHook::{GetHomogeneous, GetWidgets, SetHomogeneous};
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct BoxUserData {
  pub r#type: String,
  pub sender: relm4::Sender<BoxMsg>,
}

#[impl_lua_base(BoxMsg::LuaHook)]
impl BoxUserData {
  fn sender(&self) -> Result<relm4::Sender<BoxMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  // BOX PROPERTIES START
  impl_getter_fn!(get_homogeneous, BoxMsg::LuaHook, GetHomogeneous, bool);
  impl_setter_fn!(set_homogeneous, BoxMsg::LuaHook, SetHomogeneous, bool);

  impl_getter_fn!(get_widgets, BoxMsg::LuaHook, GetWidgets, WidgetUserDataVec);
  // BOX PROPERTIES END
}

#[impl_lua_base]
impl UserData for BoxUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    // BOX PROPERTIES START
    methods.add_method("get_homogeneous", |lua, instance, ()| {
      lua.to_value(&instance.get_homogeneous()?)
    });
    methods.add_method("set_homogeneous", |lua, this, value: mlua::Value| {
      this.set_homogeneous(lua, value)
    });

    methods.add_method("get_widgets", |lua, instance, ()| lua.pack(instance.get_widgets()?));
    // BOX PROPERTIES END

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
