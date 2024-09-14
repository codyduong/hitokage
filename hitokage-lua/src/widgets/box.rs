use super::{WidgetUserData, WidgetUserDataVec};
use crate::impl_lua_get_widget_by_id;
use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::structs::Align;
use hitokage_core::widgets::r#box::BoxMsgPortable;
use hitokage_core::widgets::r#box::BoxMsgHook::BaseHook;
use hitokage_core::widgets::r#box::BoxMsgHook::{GetHomogeneous, GetWidgets, SetHomogeneous};
use hitokage_macros::impl_lua_base;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BoxUserData {
  pub r#type: String,
  pub sender: relm4::Sender<BoxMsgPortable>,
}

#[impl_lua_base(BoxMsgPortable::LuaHook)]
impl BoxUserData {
  fn sender(&self) -> Result<relm4::Sender<BoxMsgPortable>, crate::HitokageError> {
    Ok(self.sender.clone())
  }

  // BOX PROPERTIES START
  impl_getter_fn!(get_homogeneous, BoxMsgPortable::LuaHook, GetHomogeneous, bool);
  impl_setter_fn!(set_homogeneous, BoxMsgPortable::LuaHook, SetHomogeneous, bool);

  impl_getter_fn!(get_widgets, BoxMsgPortable::LuaHook, GetWidgets, WidgetUserDataVec);
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

    impl_lua_get_widget_by_id!(methods);

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

#[macro_export]
macro_rules! impl_lua_get_widget_by_id {
  ($methods: expr) => {
    $methods.add_method(
      "get_widget_by_id",
      |lua, instance, args: mlua::Variadic<mlua::Value>| {
        if args.len() < 1 || args.len() > 2 {
          return Err(mlua::Error::RuntimeError("Expected one or two arguments".to_string()));
        }

        let id: String = lua.from_value(args[0].clone()).map_err(|e| mlua::Error::BadArgument {
          to: Some("get_widget_by_id".to_string()),
          pos: 0,
          name: Some("id".to_string()),
          cause: Arc::new(e),
        })?;
        let recursive: Option<bool> = lua.from_value(args[1].clone()).map_err(|e| mlua::Error::BadArgument {
          to: Some("get_widget_by_id".to_string()),
          pos: 0,
          name: Some("recursive".to_string()),
          cause: Arc::new(e),
        })?;

        let mut queue = VecDeque::from(Vec::from(instance.get_widgets()?));

        while let Some(widget) = queue.pop_front() {
          let widget_id = widget.get_id();

          if widget_id.filter(|w_id| *w_id == id).is_some() {
            return lua.pack(widget);
          }

          if recursive.unwrap_or(false) {
            match widget {
              WidgetUserData::Box(box_userdata) => {
                let children = box_userdata.get_widgets()?;
                for child in children {
                  queue.push_back(child);
                }
              }
              _ => {}
            }
          }
        }

        Ok(mlua::Nil)
      },
    );
  };
}
