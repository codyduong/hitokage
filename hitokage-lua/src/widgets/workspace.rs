use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::common::Align;
use hitokage_core::widgets::workspace::WorkspaceMsg;
use hitokage_core::widgets::workspace::WorkspaceMsgHook::{
  GetHalign, GetItemHeight, GetItemWidth, SetHalign, SetItemHeight, SetItemWidth,
};
use mlua::Lua;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct WorkspaceUserData {
  pub r#type: String,
  pub sender: relm4::Sender<WorkspaceMsg>,
}

impl WorkspaceUserData {
  fn sender(&self) -> Result<relm4::Sender<WorkspaceMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_halign, WorkspaceMsg::LuaHook, GetHalign, Align);
  impl_setter_fn!(set_halign, WorkspaceMsg::LuaHook, SetHalign, Align);

  impl_getter_fn!(get_item_height, WorkspaceMsg::LuaHook, GetItemHeight, u32);
  impl_setter_fn!(set_item_height, WorkspaceMsg::LuaHook, SetItemHeight, u32);

  impl_getter_fn!(get_item_width, WorkspaceMsg::LuaHook, GetItemWidth, u32);
  impl_setter_fn!(set_item_width, WorkspaceMsg::LuaHook, SetItemWidth, u32);
}

impl UserData for WorkspaceUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_halign", |lua, this, _: ()| lua.to_value(&this.get_halign()?));
    methods.add_method("set_halign", |lua, this, value: mlua::Value| {
      Ok(this.set_halign(lua, value)?)
    });

    methods.add_method("get_item_height", |_, this, _: ()| Ok(this.get_item_height()?));
    methods.add_method("set_item_height", |lua, this, value: mlua::Value| {
      Ok(this.set_item_height(lua, value)?)
    });

    methods.add_method("get_item_width", |_, this, _: ()| Ok(this.get_item_width()?));
    methods.add_method("set_item_width", |lua, this, value: mlua::Value| {
      Ok(this.set_item_width(lua, value)?)
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
