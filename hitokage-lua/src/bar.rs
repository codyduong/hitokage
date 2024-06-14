use std::sync::{Arc, Mutex};

use hitokage_core::widgets::bar::{Bar, BarProps, BAR};
use mlua::{Lua, LuaSerdeExt, UserData, UserDataMethods, Value};
use relm4::{Component, ComponentSender};

struct BarInstance {
  id: u32,
  // sender: Option<ComponentSender<Bar>>,
  sender: Arc<Mutex<Option<ComponentSender<Bar>>>>,
}

impl BarInstance {
  fn is_ready(&self) -> bool {
    let sender = self.sender.lock().unwrap();
    if sender.is_some() {
      return true;
    }
    return false;
  }
}

impl UserData for BarInstance {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("get_id", |_, instance, ()| Ok(instance.id.clone()));

    methods.add_method("is_ready", |_, instance, ()| {
      Ok(instance.is_ready())
    });
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
        move |lua_inner, value: Value| {
          let props: BarProps = lua_inner.from_value(value)?;
          let bar_sender: Arc<Mutex<Option<ComponentSender<Bar>>>> = Arc::new(Mutex::new(None));

          let mut id_l = id.lock().unwrap();
          sender.input(<C as Component>::Input::LuaHook(crate::LuaHook {
            t: crate::LuaHookType::CreateBar(props, *id_l, {
              let bar_sender = Arc::clone(&bar_sender);
              {
                Box::new(move |s| {
                  let mut bar_sender_l = bar_sender.lock().unwrap();
                  *bar_sender_l = Some(s);
                  drop(bar_sender_l);
                })
              }
            }),
            // callback: Box::new(|_| Ok(())),
          }));
          // let bar_sender_l = bar_sender.lock().unwrap();

          let bar_instance = BarInstance {
            id: *id_l,
            // sender: bar_sender_l.clone(),
            sender: bar_sender,
          };

          // println!("maybebar {:?}", bar_sender_l.is_some());

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
