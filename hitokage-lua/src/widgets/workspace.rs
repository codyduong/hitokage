use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::components::workspace::WorkspaceMsg;
use hitokage_core::components::workspace::WorkspaceMsgHook::BaseHook;
use hitokage_core::components::workspace::WorkspaceMsgHook::{
  GetItemHeight, GetItemWidth, SetItemHeight, SetItemWidth,
};
use hitokage_core::structs::Align;
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct WorkspaceUserData {
  pub r#type: String,
  pub sender: relm4::Sender<WorkspaceMsg>,
}

#[impl_lua_base(WorkspaceMsg::LuaHook)]
impl WorkspaceUserData {
  fn sender(&self) -> Result<relm4::Sender<WorkspaceMsg>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  impl_getter_fn!(get_item_height, WorkspaceMsg::LuaHook, GetItemHeight, u32);
  impl_setter_fn!(set_item_height, WorkspaceMsg::LuaHook, SetItemHeight, u32);

  impl_getter_fn!(get_item_width, WorkspaceMsg::LuaHook, GetItemWidth, u32);
  impl_setter_fn!(set_item_width, WorkspaceMsg::LuaHook, SetItemWidth, u32);
}

#[impl_lua_base]
impl UserData for WorkspaceUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_type", |_, this, _: ()| Ok(this.r#type.clone()));

    methods.add_method("get_item_height", |_, this, _: ()| Ok(this.get_item_height()?));
    methods.add_method("set_item_height", |lua, this, value: mlua::Value| {
      this.set_item_height(lua, value)
    });

    methods.add_method("get_item_width", |_, this, _: ()| Ok(this.get_item_width()?));
    methods.add_method("set_item_width", |lua, this, value: mlua::Value| {
      this.set_item_width(lua, value)
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
