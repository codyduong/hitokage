use super::{ChildUserDataVec, HoldsChildren};
use crate::{impl_getter_fn, impl_lua_get_child_by_id, impl_setter_fn};
use hitokage_core::components::app::{LuaHook, LuaHookType};
use hitokage_core::components::bar::BarLuaHook::BoxHook;
use hitokage_core::components::bar::BarLuaHook::GetGeometry;
use hitokage_core::components::bar::{BarMsg, BarProps};
use hitokage_core::components::r#box::BoxMsgHook::BaseHook;
use hitokage_core::components::r#box::BoxMsgHook::{GetChildren, GetHomogeneous, SetHomogeneous};
use hitokage_core::deserializer::LuaDeserializer;
use hitokage_core::structs::{Align, Monitor, MonitorGeometry};
use hitokage_macros::impl_lua_base;
use mlua::Table;
use mlua::{
  Lua, LuaSerdeExt, UserData, UserDataMethods,
  Value::{self},
};
use relm4::{Component, ComponentSender};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

pub(crate) struct BarUserData {
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
  #[rustfmt::skip]
  impl_getter_fn!(get_height_request, BarMsg::LuaHook, BoxHook, BaseHook, GetHeightRequest, i32);
  #[rustfmt::skip]
  impl_setter_fn!(set_height_request, BarMsg::LuaHook, BoxHook, BaseHook, SetHeightRequest, Option<i32>);

  impl_getter_fn!(get_halign, BarMsg::LuaHook, BoxHook, BaseHook, GetHalign, Align);
  impl_setter_fn!(set_halign, BarMsg::LuaHook, BoxHook, BaseHook, SetHalign, Align);

  #[rustfmt::skip]
  impl_getter_fn!(get_hexpand, BarMsg::LuaHook, BoxHook, BaseHook, GetHexpand, Option<bool>);
  #[rustfmt::skip]
  impl_setter_fn!(set_hexpand, BarMsg::LuaHook, BoxHook, BaseHook, SetHexpand, Option<bool>);

  impl_getter_fn!(get_id, BarMsg::LuaHook, BoxHook, BaseHook, GetId, Option<String>);

  #[rustfmt::skip]
  impl_getter_fn!(get_size_request, BarMsg::LuaHook, BoxHook, BaseHook, GetSizeRequest, (i32, i32));
  #[rustfmt::skip]
  impl_setter_fn!(set_size_request, BarMsg::LuaHook, BoxHook, BaseHook, SetSizeRequest, (Option<i32>, Option<i32>));

  impl_getter_fn!(get_valign, BarMsg::LuaHook, BoxHook, BaseHook, GetValign, Align);
  impl_setter_fn!(set_valign, BarMsg::LuaHook, BoxHook, BaseHook, SetValign, Align);

  #[rustfmt::skip]
  impl_getter_fn!(get_vexpand, BarMsg::LuaHook, BoxHook, BaseHook, GetVexpand, Option<bool>);
  #[rustfmt::skip]
  impl_setter_fn!(set_vexpand, BarMsg::LuaHook, BoxHook, BaseHook, SetVexpand, Option<bool>);

  impl_getter_fn!(get_width, BarMsg::LuaHook, BoxHook, BaseHook, GetWidth, i32);
  #[rustfmt::skip]
  impl_getter_fn!(get_width_request, BarMsg::LuaHook, BoxHook, BaseHook, GetWidthRequest, i32);
  #[rustfmt::skip]
  impl_setter_fn!(set_width_request, BarMsg::LuaHook, BoxHook, BaseHook, SetWidthRequest, Option<i32>);
  // BASE PROPERTIES END

  // BOX PROPERTIES START
  impl_getter_fn!(get_homogeneous, BarMsg::LuaHook, BoxHook, GetHomogeneous, bool);
  impl_setter_fn!(set_homogeneous, BarMsg::LuaHook, BoxHook, SetHomogeneous, bool);
  // BOX PROPERTIES END

  impl_getter_fn!(get_geometry, BarMsg::LuaHook, GetGeometry, MonitorGeometry);
}

impl HoldsChildren for BarUserData {
  impl_getter_fn!(,get_children, BarMsg::LuaHook, BoxHook, GetChildren, ChildUserDataVec);
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

    methods.add_method("get_children", |lua, instance, ()| lua.pack(instance.get_children()?));
    methods.add_method("get_widgets", |lua, instance, ()| lua.pack(instance.get_children()?));
    // BOX PROPERTIES END

    methods.add_method("get_geometry", |lua, instance, ()| {
      lua.to_value(&instance.get_geometry()?)
    });

    impl_lua_get_child_by_id!(methods);

    methods.add_meta_method(
      "__index",
      |lua, instance, value| -> Result<mlua::Value<'lua>, mlua::Error> {
        match value {
          Value::String(s) => match s.to_str()? {
            "ready" => Ok(lua.to_value(&instance.is_ready())?),
            "children" => Ok(lua.pack(instance.get_children()?)?),
            "widgets" => Ok(lua.pack(instance.get_children()?)?),
            "geometry" => Ok(lua.to_value(&instance.get_geometry()?)?),
            _ => Ok(Value::Nil),
          },
          _ => Ok(Value::Nil),
        }
      },
    )
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
        move |lua, value: (mlua::AnyUserData, Table)| {
          let opts = mlua::serde::de::Options::new().deny_unsupported_types(false);

          let monitor = value.0.borrow::<Monitor>()?;
          let props = BarProps::deserialize(LuaDeserializer::new(lua, mlua::Value::Table(value.1), opts))?;

          let bar_sender: Arc<Mutex<Option<relm4::Sender<BarMsg>>>> = Arc::new(Mutex::new(None));

          sender.input(<C as Component>::Input::LuaHook(LuaHook {
            t: LuaHookType::CreateBar(Box::new(monitor.clone()), props, {
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
