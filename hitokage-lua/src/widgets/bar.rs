use super::WidgetUserDataVec;
use crate::{impl_getter_fn, impl_setter_fn};
use hitokage_core::deserializer::LuaDeserializer;
use hitokage_core::structs::{Align, Monitor, MonitorGeometry};
use hitokage_core::widgets::bar::BarLuaHook::BoxHook;
use hitokage_core::widgets::bar::BarLuaHook::GetGeometry;
use hitokage_core::widgets::bar::{BarMsg, BarProps};
use hitokage_core::widgets::base::BaseMsgHook::{
  GetClass, GetHalign, GetHeight, GetHeightRequest, GetHexpand, GetSizeRequest, GetValign, GetVexpand, GetWidth,
  GetWidthRequest, SetClass, SetHalign, SetHeightRequest, SetHexpand, SetSizeRequest, SetValign, SetVexpand,
  SetWidthRequest,
};
use hitokage_core::widgets::r#box::BoxMsgHook::BaseHook;
use hitokage_core::widgets::r#box::BoxMsgHook::{GetHomogeneous, GetWidgets, SetHomogeneous};
use hitokage_macros::impl_lua_base;
use mlua::{FromLuaMulti, Table};
use mlua::{
  Lua, LuaSerdeExt, UserData, UserDataMethods,
  Value::{self},
};
use relm4::{Component, ComponentSender};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

struct BarUserData {
  sender: Arc<Mutex<Option<relm4::Sender<BarMsg>>>>,
}

impl BarUserData {
  fn is_ready(&self) -> bool {
    let sender = self.sender.lock().unwrap();
    if sender.is_some() {
      return true;
    }
    false
  }

  fn sender(&self) -> Result<relm4::Sender<BarMsg>, crate::HitokageError> {
    let sender = self.sender.lock().unwrap();

    match &*sender {
      Some(sender) => Ok(sender.clone()),
      None => Err(crate::HitokageError::RustError(
        "Bar is not ready, did we wait for Bar.ready or Bar:is_ready()".to_string(),
      )),
    }
  }

  // BASE PROPERTIES START
  impl_getter_fn!(get_class, BarMsg::LuaHook, BoxHook, BaseHook, GetClass, Vec<String>);
  impl_setter_fn!(set_class, BarMsg::LuaHook, BoxHook, BaseHook, SetClass, Vec<String>);

  impl_getter_fn!(get_height, BarMsg::LuaHook, BoxHook, BaseHook, GetHeight, i32);
  impl_getter_fn!(
    get_height_request,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    GetHeightRequest,
    i32
  );
  impl_setter_fn!(
    set_height_request,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    SetHeightRequest,
    Option<i32>
  );

  impl_getter_fn!(get_halign, BarMsg::LuaHook, BoxHook, BaseHook, GetHalign, Align);
  impl_setter_fn!(set_halign, BarMsg::LuaHook, BoxHook, BaseHook, SetHalign, Align);

  impl_getter_fn!(
    get_hexpand,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    GetHexpand,
    Option<bool>
  );
  impl_setter_fn!(
    set_hexpand,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    SetHexpand,
    Option<bool>
  );

  impl_getter_fn!(
    get_size_request,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    GetSizeRequest,
    (i32, i32)
  );
  impl_setter_fn!(
    set_size_request,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    SetSizeRequest,
    (Option<i32>, Option<i32>)
  );

  impl_getter_fn!(get_valign, BarMsg::LuaHook, BoxHook, BaseHook, GetValign, Align);
  impl_setter_fn!(set_valign, BarMsg::LuaHook, BoxHook, BaseHook, SetValign, Align);

  impl_getter_fn!(
    get_vexpand,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    GetVexpand,
    Option<bool>
  );
  impl_setter_fn!(
    set_vexpand,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    SetVexpand,
    Option<bool>
  );

  impl_getter_fn!(get_width, BarMsg::LuaHook, BoxHook, BaseHook, GetWidth, i32);
  impl_getter_fn!(
    get_width_request,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    GetWidthRequest,
    i32
  );
  impl_setter_fn!(
    set_width_request,
    BarMsg::LuaHook,
    BoxHook,
    BaseHook,
    SetWidthRequest,
    Option<i32>
  );
  // BASE PROPERTIES END

  // BOX PROPERTIES START
  impl_getter_fn!(get_homogeneous, BarMsg::LuaHook, BoxHook, GetHomogeneous, bool);
  impl_setter_fn!(set_homogeneous, BarMsg::LuaHook, BoxHook, SetHomogeneous, bool);

  impl_getter_fn!(get_widgets, BarMsg::LuaHook, BoxHook, GetWidgets, WidgetUserDataVec);
  // BOX PROPERTIES END

  impl_getter_fn!(get_geometry, BarMsg::LuaHook, GetGeometry, MonitorGeometry);
}

#[impl_lua_base]
impl UserData for BarUserData {
  fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("is_ready", |_, instance, ()| Ok(instance.is_ready()));

    // BOX PROPERTIES START
    methods.add_method("get_homogeneous", |lua, instance, ()| {
      lua.to_value(&instance.get_homogeneous()?)
    });
    methods.add_method("set_homogeneous", |lua, this, value: mlua::Value| {
      this.set_homogeneous(lua, value)
    });

    methods.add_method("get_widgets", |lua, instance, ()| lua.pack(instance.get_widgets()?));
    // BOX PROPERTIES END

    methods.add_method("get_geometry", |lua, instance, ()| {
      lua.to_value(&instance.get_geometry()?)
    });

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "ready" => Ok(lua.to_value(&instance.is_ready())?),
            "widgets" => Ok(lua.pack(instance.get_widgets()?)?),
            "geometry" => Ok(lua.to_value(&instance.get_geometry()?)?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
  }
}

#[derive(Serialize, Deserialize)]
struct BarPropsExtended {
  monitor: Monitor,
  props: BarProps,
}

impl<'lua> FromLuaMulti<'lua> for BarPropsExtended {
  fn from_lua_multi(values: mlua::MultiValue<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
    let mut iter = values.into_iter();

    let monitor: Monitor = match iter.next() {
      Some(value) => lua.from_value(value)?,
      None => {
        return Err(mlua::Error::BadArgument {
          to: Some("create".to_string()),
          pos: 0,
          name: Some("monitor".to_string()),
          cause: Arc::new(mlua::Error::FromLuaConversionError {
            from: "Value",
            to: "Monitor",
            message: Some("Expected a Value of type Monitor for the first argument, received Nil".to_string()),
          }),
        })
      }
    };
    let props: BarProps = match iter.next() {
      Some(value) => lua.from_value(value)?,
      None => {
        return Err(mlua::Error::BadArgument {
          to: Some("create".to_string()),
          pos: 1,
          name: Some("bar_props".to_string()),
          cause: Arc::new(mlua::Error::FromLuaConversionError {
            from: "Value",
            to: "Monitor",
            message: Some("Expected a Value of type BarProps for the second argument, received Nil".to_string()),
          }),
        })
      }
    };

    Ok(BarPropsExtended { monitor, props })
  }
}

pub fn make<'lua, C>(lua: &'lua Lua, sender: &ComponentSender<C>) -> anyhow::Result<mlua::Table<'lua>>
where
  C: Component<Input = crate::AppMsg>,
  <C as Component>::Output: std::marker::Send,
{
  let table = lua.create_table()?;

  {
    table.set(
      "create",
      lua.create_function({
        let sender = sender.clone();
        move |lua, value: (Table, Table)| {
          let opts = mlua::serde::de::Options::new().deny_unsupported_types(false);

          let monitor: Monitor = lua.from_value(mlua::Value::Table(value.0))?;
          let props = BarProps::deserialize(LuaDeserializer::new(mlua::Value::Table(value.1), opts))?;
          // let props: BarProps = lua.from_value_with(mlua::Value::Table(value.1), opts)?;

          let bar_sender: Arc<Mutex<Option<relm4::Sender<BarMsg>>>> = Arc::new(Mutex::new(None));

          sender.input(<C as Component>::Input::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::CreateBar(Box::new(monitor), props, {
              let bar_sender = Arc::clone(&bar_sender);
              {
                Box::new(move |s| {
                  let mut bar_sender_l = bar_sender.lock().unwrap();
                  *bar_sender_l = Some(s);
                  drop(bar_sender_l);
                })
              }
            }),
          }));

          let bar_instance = BarUserData { sender: bar_sender };

          Ok(bar_instance)
        }
      })?,
    )?;
  }

  Ok(table)
}

#[cfg(test)]
mod tests {
  use crate::{assert_lua_type, AppMsg};
  use gtk4::prelude::*;
  use mlua::{Lua, Table, Value};
  use relm4::prelude::*;
  use relm4::{ComponentParts, SimpleComponent};

  struct DummyComponent {}

  #[relm4::component]
  impl SimpleComponent for DummyComponent {
    type Input = crate::AppMsg;
    type Output = ();
    type Init = ();

    view! {
      gtk::ApplicationWindow {
        set_visible: false,
      },
    }

    fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
      let model = DummyComponent {};

      let tests = || -> anyhow::Result<()> {
        let lua = Lua::new();
        let table = super::make(&lua, &sender)?;
        lua.globals().set("table", table)?;
        let value: Value = lua.globals().get("table")?;

        let table: Table = assert_lua_type!(value, mlua::Table);
        assert_lua_type!(table.get::<&str, Value>("create")?, mlua::Function);
        assert_eq!(table.len()?, 0);
        assert_eq!(table.pairs::<String, Value>().count(), 1);

        relm4::main_application().quit();

        Ok(())
      };
      tests().unwrap();

      let widgets = view_output!();

      ComponentParts { model, widgets }
    }

    fn update(&mut self, _: AppMsg, _: ComponentSender<Self>) {}
  }

  #[test]
  fn test_all() -> anyhow::Result<()> {
    let app = RelmApp::new("com.example.hitokagetest");
    app.run::<DummyComponent>(());

    Ok(())
  }
}
