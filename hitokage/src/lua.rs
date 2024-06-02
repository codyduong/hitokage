use luahelper::ValuePrinter;
use mlua::{Lua, Table, Value, Variadic};

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

pub fn get_or_create_sub_module<'lua>(
    lua: &'lua Lua,
    name: &str,
) -> anyhow::Result<mlua::Table<'lua>> {
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
            item @ _ => {
                let item = format!("{:#?}", ValuePrinter(item));
                output.push_str(&item);
            }
        }
    }
    output
}

pub fn make_lua() -> anyhow::Result<Lua> {
    let lua = Lua::new();

    {
        let globals = lua.globals();
        globals.set("loaded", lua.create_table()?)?;
        let hitokage_mod = get_or_create_module(&lua, "hitokage")?;

        hitokage_mod.set(
            // https://github.com/wez/wezterm/blob/b8f94c474ce48ac195b51c1aeacf41ae049b774e/lua-api-crates/logging/src/lib.rs#L17
            "print",
            lua.create_function(|_, args: Variadic<Value>| {
                let output = print_helper(args);
                log::info!("lua: {}", output);
                Ok(())
            })?,
        )?;

        hitokage_mod.set(
            "position",
            lua.create_function(|_, args: Variadic<Value>| {
                let output = print_helper(args);
                log::info!("lua: {}", output);
                Ok(())
            })?,
        )?;

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
