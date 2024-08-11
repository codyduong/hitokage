use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::reactive::Reactive;
use hitokage_core::structs::Align;
use hitokage_core::widgets::base::BaseMsgHook::{
  GetClass, GetHalign, GetHexpand, GetValign, GetVexpand, SetClass, SetHalign, SetHexpand, SetValign, SetVexpand,
};
use hitokage_core::widgets::label::LabelMsg;
use hitokage_core::widgets::label::LabelMsgHook::BaseHook;
use hitokage_core::widgets::label::LabelMsgHook::{GetLabel, GetLabelReactive, SetLabel};
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct LabelUserData {
  pub r#type: String,
  pub sender: relm4::Sender<LabelMsg>,
}

impl LabelUserData {
  fn sender(&self) -> Result<relm4::Sender<LabelMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  // BASE PROPERTIES START
  impl_getter_fn!(get_class, LabelMsg::LuaHook, BaseHook, GetClass, Vec<String>);
  impl_setter_fn!(set_class, LabelMsg::LuaHook, BaseHook, SetClass, Vec<String>);

  impl_getter_fn!(get_halign, LabelMsg::LuaHook, BaseHook, GetHalign, Align);
  impl_setter_fn!(set_halign, LabelMsg::LuaHook, BaseHook, SetHalign, Align);

  impl_getter_fn!(get_hexpand, LabelMsg::LuaHook, BaseHook, GetHexpand, Option<bool>);
  impl_setter_fn!(set_hexpand, LabelMsg::LuaHook, BaseHook, SetHexpand, Option<bool>);

  impl_getter_fn!(get_valign, LabelMsg::LuaHook, BaseHook, GetValign, Align);
  impl_setter_fn!(set_valign, LabelMsg::LuaHook, BaseHook, SetValign, Align);

  impl_getter_fn!(get_vexpand, LabelMsg::LuaHook, BaseHook, GetVexpand, Option<bool>);
  impl_setter_fn!(set_vexpand, LabelMsg::LuaHook, BaseHook, SetVexpand, Option<bool>);
  // BASE PROPERTIES END

  impl_getter_fn!(get_label, LabelMsg::LuaHook, GetLabel, String);
  impl_getter_fn!(
    get_label_reactive,
    LabelMsg::LuaHook,
    GetLabelReactive,
    Reactive<String>
  );
  impl_setter_fn!(set_label, LabelMsg::LuaHook, SetLabel, String);
}

impl UserData for LabelUserData {
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
