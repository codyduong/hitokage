use mlua::{LuaSerdeExt, UserData};
use std::sync::Arc;

#[derive(Debug)]
pub struct LuaActionRequest {
  pub id: Arc<mlua::RegistryKey>,
  pub args: serde_json::Value,
  pub f: Option<std::sync::mpsc::Sender<mlua::Value>>,
}

impl UserData for LuaActionRequest {
  fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
    methods.add_method_mut("call", |lua, this, _: ()| {
      log::error!("my fridge is on fire");
      let func: mlua::Function = lua.registry_value(&this.id)?;
      let args = lua.to_value(&this.args.clone()).unwrap();
      let res = func.call::<mlua::Value>(args)?;
      this
        .f
        .take()
        .expect("We attempted to send on an dropped component or an already evaluated lua action request")
        .send(res)
        .unwrap();
      Ok(())
    });
  }
}
