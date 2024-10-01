use hitokage_core::event::LUA_ACTION_REQUESTS;
use mlua::Lua;

pub fn make<'lua>(lua: &'lua Lua) -> anyhow::Result<mlua::Table> {
  let table = lua.create_table()?;

  {
    table.set(
      "get_unread",
      lua.create_function({
        move |lua_inner, _: ()| {
          let mut callbacks = LUA_ACTION_REQUESTS.write();

          // log::error!("my apples! {:?}", callbacks);

          let mut res = Vec::new();
          for v in callbacks.drain(..) {
            res.push(lua_inner.pack(v)?);
          }

          // log::error!("my mead! {:?}", res);

          Ok(lua_inner.pack(res))
        }
      })?,
    )?;
  }

  Ok(table)
}
