use api::{event, monitor, reactive};
use hitokage_core::components::app::AppMsg;
use luahelper::ValuePrinter;
use mlua::{AnyUserData, Lua, Table, Value, Variadic};
use relm4::{Component, ComponentSender};
use std::fmt;
use widgets::bar;

pub mod api;
pub mod widgets;

// Thanks @wez https://github.com/wez/wezterm/blob/b8f94c474ce48ac195b51c1aeacf41ae049b774e/config/src/lua.rs#L211

pub fn get_or_create_module<'lua>(lua: &'lua Lua, name: &str) -> anyhow::Result<mlua::Table<'lua>> {
  let globals: Table = lua.globals();
  // let package: Table = globals.get("package")?;
  // let loaded: Table = package.get("loaded")?;
  let loaded: Table = globals.get("loaded")?;

  let module = loaded.get(name)?;
  match module {
    Value::Nil => {
      let module = lua.create_table()?;
      loaded.set(name, module.clone())?;
      Ok(module)
    }
    Value::Table(table) => Ok(table),
    wat => anyhow::bail!(
      "cannot register module {} as package.loaded.{} is already set to a value of type {}",
      name,
      name,
      wat.type_name()
    ),
  }
}

pub fn get_or_create_sub_module<'lua>(lua: &'lua Lua, name: &str) -> anyhow::Result<mlua::Table<'lua>> {
  let hitokage_mod = get_or_create_module(lua, "hitokage")?;
  let sub = hitokage_mod.get(name)?;
  match sub {
    Value::Nil => {
      let sub = lua.create_table()?;
      hitokage_mod.set(name, sub.clone())?;
      Ok(sub)
    }
    Value::Table(sub) => Ok(sub),
    wat => anyhow::bail!(
      "cannot register module hitokage.{name} as it is already set to a value of type {}",
      wat.type_name()
    ),
  }
}

fn print_helper(args: Variadic<Value>) -> String {
  let mut output = String::new();
  for (idx, item) in args.into_iter().enumerate() {
    if idx > 0 {
      output.push(' ');
    }

    match item {
      Value::String(s) => match s.to_str() {
        Ok(s) => output.push_str(s),
        Err(_) => {
          let item = String::from_utf8_lossy(s.as_bytes());
          output.push_str(&item);
        }
      },
      item => {
        let item = format!("{:#?}", ValuePrinter(item));
        output.push_str(&item);
      }
    }
  }
  output
}

// https://github.com/wez/wezterm/blob/e4b18c41e650718b031dcc8ef0f93f23a1013aaa/lua-api-crates/time-funcs/src/lib.rs#L193
async fn sleep_ms<'lua>(_: &'lua Lua, milliseconds: u64) -> mlua::Result<()> {
  let duration = std::time::Duration::from_millis(milliseconds);
  smol::Timer::after(duration).await;
  Ok(())
}

pub fn make<C>(sender: ComponentSender<C>) -> anyhow::Result<Lua>
where
  C: Component<Input = AppMsg>,
  <C as Component>::Output: std::marker::Send,
{
  let lua = Lua::new();

  {
    let globals = lua.globals();
    globals.set("loaded", lua.create_table()?)?;
    let hitokage_mod = get_or_create_module(&lua, "hitokage")?;
    let monitor: AnyUserData = monitor::make(&lua)?;
    let bar: Table = bar::make(&lua, &sender)?;
    let event: Table = event::make(&lua, &sender)?;
    let reactive: Table = reactive::make(&lua)?;

    let unstable: Table = lua.create_table()?;
    unstable.set("reactive", reactive)?;

    hitokage_mod.set("monitor", monitor)?;
    hitokage_mod.set("bar", bar)?;
    hitokage_mod.set("event", event)?;
    hitokage_mod.set("unstable", unstable)?;

    hitokage_mod.set(
      // https://github.com/wez/wezterm/blob/b8f94c474ce48ac195b51c1aeacf41ae049b774e/lua-api-crates/logging/src/lib.rs#L17
      "info",
      lua.create_function(|_, args: Variadic<Value>| {
        let output = print_helper(args);
        log::info!("lua: {}", output);
        Ok(())
      })?,
    )?;

    hitokage_mod.set(
      // https://github.com/wez/wezterm/blob/b8f94c474ce48ac195b51c1aeacf41ae049b774e/lua-api-crates/logging/src/lib.rs#L17
      "debug",
      lua.create_function(|_, args: Variadic<Value>| {
        let output = print_helper(args);
        log::debug!("lua: {}", output);
        Ok(())
      })?,
    )?;

    hitokage_mod.set(
      "error",
      lua.create_function(|_, args: Variadic<Value>| {
        let output = print_helper(args);
        log::error!("lua: {}", output);
        Ok(())
      })?,
    )?;

    hitokage_mod.set("sleep_ms", lua.create_async_function(sleep_ms)?)?;

    globals.set("hitokage", hitokage_mod)?;

    globals.set(
      "print",
      lua.create_function(|_, args: Variadic<Value>| {
        let output = print_helper(args);
        log::info!("lua: {}", output);
        Ok(())
      })?,
    )?;
  }

  Ok(lua)
}

trait FromLuaValue<'lua>: Sized {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self>;
}

impl<'lua> FromLuaValue<'lua> for Table<'lua> {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self> {
    match value {
      Value::Table(table) => Ok(table),
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "Table",
        message: None,
      }),
    }
  }
}

impl<'lua> FromLuaValue<'lua> for mlua::Function<'lua> {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self> {
    match value {
      Value::Function(function) => Ok(function),
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "Function",
        message: None,
      }),
    }
  }
}

impl<'lua> FromLuaValue<'lua> for mlua::String<'lua> {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self> {
    match value {
      Value::String(string) => Ok(string),
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "String",
        message: None,
      }),
    }
  }
}

impl<'lua> FromLuaValue<'lua> for mlua::Integer {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self> {
    match value {
      Value::Integer(integer) => Ok(integer),
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "Integer",
        message: None,
      }),
    }
  }
}

impl<'lua> FromLuaValue<'lua> for bool {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self> {
    match value {
      Value::Boolean(boolean) => Ok(boolean),
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "Boolean",
        message: None,
      }),
    }
  }
}

impl<'lua> FromLuaValue<'lua> for AnyUserData<'lua> {
  fn from_lua_value(value: Value<'lua>) -> mlua::Result<Self> {
    match value {
      Value::UserData(userdata) => Ok(userdata),
      _ => Err(mlua::Error::FromLuaConversionError {
        from: value.type_name(),
        to: "UserData",
        message: None,
      }),
    }
  }
}

// Macro to assert the type of a Lua value and return the inner value if it matches
#[macro_export]
macro_rules! assert_lua_type {
  ($value:expr, $type:ty) => {
    <$type as $crate::FromLuaValue>::from_lua_value($value)
      .expect(&format!("Expected Lua value to be of type {}", stringify!($type)))
  };
}

#[derive(Debug)]
enum HitokageError {
  RustError(String),
}

impl fmt::Display for HitokageError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      HitokageError::RustError(msg) => write!(f, "{}", msg),
    }
  }
}

impl std::error::Error for HitokageError {}

impl From<HitokageError> for mlua::Error {
  fn from(err: HitokageError) -> mlua::Error {
    mlua::Error::ExternalError(std::sync::Arc::new(err))
  }
}
