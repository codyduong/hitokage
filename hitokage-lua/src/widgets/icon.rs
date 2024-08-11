use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::reactive::Reactive;
use hitokage_core::structs::Align;
use hitokage_core::widgets::base::BaseMsgHook::{
  GetClass, GetHalign, GetHexpand, GetValign, GetVexpand, SetClass, SetHalign, SetHexpand, SetValign, SetVexpand,
};
use hitokage_core::widgets::icon::IconMsg;
use hitokage_core::widgets::icon::IconMsgHook::BaseHook;
use hitokage_core::widgets::icon::IconMsgHook::{GetFile, GetFileReactive, SetFile};
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct IconUserData {
  pub r#type: String,
  pub sender: relm4::Sender<IconMsg>,
}

impl IconUserData {
  fn sender(&self) -> Result<relm4::Sender<IconMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  // BASE PROPERTIES START
  impl_getter_fn!(get_class, IconMsg::LuaHook, BaseHook, GetClass, Vec<String>);
  impl_setter_fn!(set_class, IconMsg::LuaHook, BaseHook, SetClass, Vec<String>);

  impl_getter_fn!(get_halign, IconMsg::LuaHook, BaseHook, GetHalign, Align);
  impl_setter_fn!(set_halign, IconMsg::LuaHook, BaseHook, SetHalign, Align);

  impl_getter_fn!(get_hexpand, IconMsg::LuaHook, BaseHook, GetHexpand, Option<bool>);
  impl_setter_fn!(set_hexpand, IconMsg::LuaHook, BaseHook, SetHexpand, Option<bool>);

  impl_getter_fn!(get_valign, IconMsg::LuaHook, BaseHook, GetValign, Align);
  impl_setter_fn!(set_valign, IconMsg::LuaHook, BaseHook, SetValign, Align);

  impl_getter_fn!(get_vexpand, IconMsg::LuaHook, BaseHook, GetVexpand, Option<bool>);
  impl_setter_fn!(set_vexpand, IconMsg::LuaHook, BaseHook, SetVexpand, Option<bool>);
  // BASE PROPERTIES END

  impl_getter_fn!(get_image, IconMsg::LuaHook, GetFile, String);
  impl_getter_fn!(get_image_reactive, IconMsg::LuaHook, GetFileReactive, Reactive<String>);
  impl_setter_fn!(set_image, IconMsg::LuaHook, SetFile, String);
}

impl UserData for IconUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    // BASE PROPERTIES START
    methods.add_method("get_class", |lua, instance, ()| lua.pack(instance.get_class()?));
    methods.add_method("set_class", |lua, this, args: mlua::Variadic<Value>| {
      this.set_class(lua, args)
    });

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

    methods.add_method("get_image", |_, this, _: ()| Ok(this.get_image()?));
    methods.add_method("get_image_reactive", |_, this, _: ()| Ok(this.get_image_reactive()?));
    methods.add_method("set_image", |lua, this, value: mlua::Value| this.set_image(lua, value));

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
