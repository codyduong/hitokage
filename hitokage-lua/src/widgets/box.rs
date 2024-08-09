use super::WidgetUserDataVec;
use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::{Align, CssClass};
use hitokage_core::widgets::base::BaseMsgHook::{
  GetClass, GetHalign, GetHexpand, GetValign, GetVexpand, SetClass, SetHalign, SetHexpand, SetValign, SetVexpand,
};
use hitokage_core::widgets::r#box::BoxMsg;
use hitokage_core::widgets::r#box::BoxMsgHook::BaseHook;
use hitokage_core::widgets::r#box::BoxMsgHook::{GetHomogeneous, GetWidgets, SetHomogeneous};
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

  // BASE PROPERTIES START
  impl_getter_fn!(get_class, BoxMsg::LuaHook, BaseHook, GetClass, Vec<String>);
  impl_setter_fn!(set_class, BoxMsg::LuaHook, BaseHook, SetClass, Option<CssClass>);

  impl_getter_fn!(get_halign, BoxMsg::LuaHook, BaseHook, GetHalign, Align);
  impl_setter_fn!(set_halign, BoxMsg::LuaHook, BaseHook, SetHalign, Align);

  impl_getter_fn!(get_hexpand, BoxMsg::LuaHook, BaseHook, GetHexpand, Option<bool>);
  impl_setter_fn!(set_hexpand, BoxMsg::LuaHook, BaseHook, SetHexpand, Option<bool>);

  impl_getter_fn!(get_valign, BoxMsg::LuaHook, BaseHook, GetValign, Align);
  impl_setter_fn!(set_valign, BoxMsg::LuaHook, BaseHook, SetValign, Align);

  impl_getter_fn!(get_vexpand, BoxMsg::LuaHook, BaseHook, GetVexpand, Option<bool>);
  impl_setter_fn!(set_vexpand, BoxMsg::LuaHook, BaseHook, SetVexpand, Option<bool>);
  // BASE PROPERTIES END

  // BOX PROPERTIES START
  impl_getter_fn!(get_homogeneous, BoxMsg::LuaHook, GetHomogeneous, bool);
  impl_setter_fn!(set_homogeneous, BoxMsg::LuaHook, SetHomogeneous, bool);

  impl_getter_fn!(get_widgets, BoxMsg::LuaHook, GetWidgets, WidgetUserDataVec);
  // BOX PROPERTIES END
}

impl UserData for BoxUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    // BASE PROPERTIES START
    methods.add_method("get_class", |lua, instance, ()| lua.pack(instance.get_class()?));
    methods.add_method("set_class", |lua, this, value: mlua::Value| this.set_class(lua, value));

    methods.add_method("get_halign", |lua, instance, ()| lua.to_value(&instance.get_halign()?));
    methods.add_method("set_halign", |lua, this, value: mlua::Value| {
      this.set_halign(lua, value)
    });

    methods.add_method("get_hexpand", |lua, instance, ()| {
      lua.to_value(&instance.get_hexpand()?)
    });
    methods.add_method("set_hexpand", |lua, this, value: mlua::Value| {
      this.set_hexpand(lua, value)
    });

    methods.add_method("get_valign", |lua, instance, ()| lua.to_value(&instance.get_valign()?));
    methods.add_method("set_valign", |lua, this, value: mlua::Value| {
      this.set_valign(lua, value)
    });

    methods.add_method("get_vexpand", |lua, instance, ()| {
      lua.to_value(&instance.get_vexpand()?)
    });
    methods.add_method("set_vexpand", |lua, this, value: mlua::Value| {
      this.set_vexpand(lua, value)
    });
    // BASE PROPERTIES END

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
