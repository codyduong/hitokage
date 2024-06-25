use hitokage_core::lua::monitor::{Monitor, MonitorGeometry};
use hitokage_core::widgets::bar::BarLuaHook::{RequestGeometry, RequestWidgets};
use hitokage_core::widgets::bar::{Bar, BarMsg, BarProps};
use hitokage_core::widgets::WidgetSender;
use mlua::FromLuaMulti;
use mlua::{
  Lua, LuaSerdeExt, UserData, UserDataMethods,
  Value::{self},
};
use relm4::{Component, ComponentSender};
use std::sync::{mpsc, Arc, Mutex};

struct BarInstanceUserData {
  id: u32,
  sender: Arc<Mutex<Option<ComponentSender<Bar>>>>,
  widgets: Arc<Mutex<Vec<WidgetSender>>>,
  geometry: Arc<Mutex<MonitorGeometry>>,
}

macro_rules! impl_getter_fn {
  ($fn_name:ident, $field:ident, $request_enum:expr, $ret:ty) => {
    fn $fn_name(&self) -> Result<$ret, crate::HitokageError> {
      let sender = self.assert_ready()?;

      let (tx, rx) = mpsc::channel::<()>();
      let arc = Arc::clone(&self.$field);
      sender.input(BarMsg::LuaHook($request_enum(arc, tx)));
      rx.recv().unwrap();
      let lock = self.$field.lock().unwrap();

      let ret_val: $ret = if std::any::TypeId::of::<$ret>() == std::any::TypeId::of::<Vec<WidgetSender>>() {
        lock.clone() as $ret
      } else {
        (*lock).clone() as $ret
      };

      Ok(ret_val)
    }
  };
}

impl BarInstanceUserData {
  fn is_ready(&self) -> bool {
    let sender = self.sender.lock().unwrap();
    if sender.is_some() {
      return true;
    }
    return false;
  }

  fn assert_ready(&self) -> Result<ComponentSender<Bar>, crate::HitokageError> {
    let sender = self.sender.lock().unwrap();

    match &*sender {
      Some(sender) => Ok(sender.clone()),
      None => Err(crate::HitokageError::RustError(
        "Bar is not ready, did we wait for BarInstance.ready or BarInstance:is_ready()".to_string(),
      )),
    }
  }

  impl_getter_fn!(get_widgets, widgets, RequestWidgets, Vec<WidgetSender>);
  impl_getter_fn!(get_geometry, geometry, RequestGeometry, MonitorGeometry);
}

impl UserData for BarInstanceUserData {
  fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_id", |_, instance, ()| Ok(instance.id.clone()));

    methods.add_method("is_ready", |_, instance, ()| Ok(instance.is_ready()));

    methods.add_method("get_widgets", |lua, instance, ()| {
      Ok(lua.pack(instance.get_widgets()?)?)
    });

    methods.add_method("get_geometry", |lua, instance, ()| {
      Ok(lua.to_value(&instance.get_geometry()?)?)
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
  let id = Arc::new(Mutex::new(0));
  let table = lua.create_table()?;

  {
    table.set(
      "create",
      lua.create_function({
        let sender = sender.clone();
        move |lua_inner, props_extended: BarPropsExtended| {
          let BarPropsExtended { monitor, props } = props_extended;
          let bar_sender: Arc<Mutex<Option<ComponentSender<Bar>>>> = Arc::new(Mutex::new(None));

          let mut id_l = id.lock().unwrap();
          sender.input(<C as Component>::Input::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::CreateBar(monitor, props, *id_l, {
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

          let bar_instance = BarInstanceUserData {
            id: *id_l,
            sender: bar_sender,
            widgets: Arc::new(Mutex::new(Vec::new())),
            geometry: Arc::new(Mutex::new(MonitorGeometry::default())),
          };

          *id_l += 1;
          drop(id_l);

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
  use relm4::{ComponentParts, ComponentSender, SimpleComponent};

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
      let _ = tests().unwrap();

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
