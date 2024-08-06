use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::{Align, CssClass};
use hitokage_core::widgets::base::BaseMsgHook::{
  GetClass, GetHalign, GetHexpand, GetHomogeneous, GetValign, GetVexpand, SetClass, SetHalign, SetHexpand,
  SetHomogeneous, SetValign, SetVexpand,
};
use hitokage_core::widgets::clock::ClockMsg;
use hitokage_core::widgets::clock::ClockMsgHook::BaseHook;
use hitokage_core::widgets::clock::ClockMsgHook::{GetFormat, SetFormat};
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct ClockUserData {
  pub r#type: String,
  pub sender: relm4::Sender<ClockMsg>,
}

impl ClockUserData {
  fn sender(&self) -> Result<relm4::Sender<ClockMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  // BASE PROPERTIES START
  impl_getter_fn!(get_class, ClockMsg::LuaHook, BaseHook, GetClass, Vec<String>);
  impl_setter_fn!(set_class, ClockMsg::LuaHook, BaseHook, SetClass, Option<CssClass>);

  impl_getter_fn!(get_halign, ClockMsg::LuaHook, BaseHook, GetHalign, Align);
  impl_setter_fn!(set_halign, ClockMsg::LuaHook, BaseHook, SetHalign, Align);

  impl_getter_fn!(get_hexpand, ClockMsg::LuaHook, BaseHook, GetHexpand, Option<bool>);
  impl_setter_fn!(set_hexpand, ClockMsg::LuaHook, BaseHook, SetHexpand, Option<bool>);

  impl_getter_fn!(get_homogeneous, ClockMsg::LuaHook, BaseHook, GetHomogeneous, bool);
  impl_setter_fn!(set_homogeneous, ClockMsg::LuaHook, BaseHook, SetHomogeneous, bool);

  impl_getter_fn!(get_valign, ClockMsg::LuaHook, BaseHook, GetValign, Align);
  impl_setter_fn!(set_valign, ClockMsg::LuaHook, BaseHook, SetValign, Align);

  impl_getter_fn!(get_vexpand, ClockMsg::LuaHook, BaseHook, GetVexpand, Option<bool>);
  impl_setter_fn!(set_vexpand, ClockMsg::LuaHook, BaseHook, SetVexpand, Option<bool>);
  // BASE PROPERTIES END

  impl_getter_fn!(get_format, ClockMsg::LuaHook, GetFormat, String);
  impl_setter_fn!(set_format, ClockMsg::LuaHook, SetFormat, String);
}

impl UserData for ClockUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    // BASE PROPERTIES START
    methods.add_method("get_class", |lua, instance, ()| Ok(lua.pack(instance.get_class()?)?));
    methods.add_method("set_class", |lua, this, value: mlua::Value| {
      Ok(this.set_class(lua, value)?)
    });

    methods.add_method("get_halign", |lua, instance, ()| {
      Ok(lua.to_value(&instance.get_halign()?)?)
    });
    methods.add_method("set_halign", |lua, this, value: mlua::Value| {
      Ok(this.set_halign(lua, value)?)
    });

    methods.add_method("get_hexpand", |lua, instance, ()| {
      Ok(lua.to_value(&instance.get_hexpand()?)?)
    });
    methods.add_method("set_hexpand", |lua, this, value: mlua::Value| {
      Ok(this.set_hexpand(lua, value)?)
    });

    methods.add_method("get_homogeneous", |lua, instance, ()| {
      Ok(lua.to_value(&instance.get_homogeneous()?)?)
    });
    methods.add_method("set_homogeneous", |lua, this, value: mlua::Value| {
      Ok(this.set_homogeneous(lua, value)?)
    });

    methods.add_method("get_valign", |lua, instance, ()| {
      Ok(lua.to_value(&instance.get_valign()?)?)
    });
    methods.add_method("set_valign", |lua, this, value: mlua::Value| {
      Ok(this.set_valign(lua, value)?)
    });

    methods.add_method("get_vexpand", |lua, instance, ()| {
      Ok(lua.to_value(&instance.get_vexpand()?)?)
    });
    methods.add_method("set_vexpand", |lua, this, value: mlua::Value| {
      Ok(this.set_vexpand(lua, value)?)
    });
    // BASE PROPERTIES END

    methods.add_method("get_format", |_, this, _: ()| Ok(this.get_format()?));
    methods.add_method("set_format", |lua, this, value: mlua::Value| {
      Ok(this.set_format(lua, value)?)
    });

    methods.add_meta_method::<_, mlua::String, _>(
      "__index",
      |lua, instance, key| -> Result<mlua::Value<'lua>, mlua::Error> {
        match key.to_str().unwrap() {
          "type" => Ok(lua.to_value(&instance.r#type.clone())?),
          _ => Ok(Value::Nil),
        }
      },
    );
  }
}
