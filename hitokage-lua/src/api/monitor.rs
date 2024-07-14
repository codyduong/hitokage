use gdk4::{
  glib::{self, object::Cast},
  prelude::*,
};
use hitokage_core::{
  lua::monitor::{Monitor, MonitorGeometry, MonitorScaleFactor},
  win_utils::get_windows_version,
};
use mlua::{AnyUserData, Lua, LuaSerdeExt, MetaMethod, UserData, UserDataMethods, Value};
use windows::Win32::{
  Graphics::Gdi::HMONITOR,
  UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI},
};

fn get_monitor_scaling(hmonitor: HMONITOR) -> Result<MonitorScaleFactor, win32_display_data::error::Error> {
  unsafe {
    let mut dpi_x: u32 = 0;
    let mut dpi_y: u32 = 0;
    let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);

    // https://learn.microsoft.com/en-us/windows/win32/learnwin32/dpi-and-device-independent-pixels
    let scaling_factor = MonitorScaleFactor {
      x: dpi_x as f32 / 96.0,
      y: dpi_y as f32 / 96.0,
    };
    Ok(scaling_factor)
  }
}

#[derive(Debug)]
struct MonitorTemp {
  hmonitor: isize,
  name: String,
  device: String,
  size: MonitorGeometry,
  // work_area: MonitorGeometry,
  device_id: String,
}

// modified from to ensure name consistency
// https://github.com/LGUG2Z/komorebi/blob/c022438a37c811d5aa188386987bea4a88bc2833/komorebi/src/windows_api.rs#L792
fn load_monitor_information() -> anyhow::Result<Vec<MonitorTemp>> {
  let mut monitors = Vec::new();

  for display in win32_display_data::connected_displays_all().flatten() {
    let path = display.device_path.clone();

    // https://github.com/LGUG2Z/komorebi/commit/ef9e734680acffdde69d9d0457f208a010e319db
    let (device, device_id) = if path.is_empty() {
      (String::from("UNKNOWN"), String::from("UNKNOWN"))
    } else {
      let mut split: Vec<_> = path.split('#').collect();
      split.remove(0);
      split.remove(split.len() - 1);
      let device = split[0].to_string();
      let device_id = split.join("-");
      (device, device_id)
    };

    let name = display.device_name.trim_start_matches(r"\\.\").to_string();
    let name = name.split('\\').collect::<Vec<_>>()[0].to_string();

    let m = MonitorTemp {
      hmonitor: display.hmonitor,
      // size: display.size.into(), TODO @codyduong this is broken, probably vers mismatch
      // work_area: display.work_area_size.into(), ditto
      size: MonitorGeometry {
        x: display.size.left,
        y: display.size.top,
        width: display.size.right - display.size.left,
        height: display.size.bottom - display.size.top,
      },
      name: name.clone(),
      device,
      device_id,
    };

    // OK this can sort by monitor DISPLAY1 -> DISPLAY2 -> ...
    // but this is not logically consistent with how komorebic indexes monitors, counterintuitively
    // match monitors.binary_search_by(|monitor: &MonitorTemp| monitor.name.cmp(&name)) {
    //   Ok(pos) | Err(pos) => monitors.insert(pos, m),
    // }
    monitors.push(m);
  }

  Ok(monitors)
}

fn get_monitors() -> impl Iterator<Item = Monitor> {
  let display = gdk4::Display::default().expect("Failed to get default display");

  let monitors = load_monitor_information().expect("Failed to get monitors");
  let other_monitors: Vec<gdk4_win32::Win32Monitor> = display
    .monitors()
    .into_iter()
    .filter_map(move |result| {
      result
        .ok()
        .and_then(|item: glib::Object| item.downcast::<gdk4_win32::Win32Monitor>().ok())
    })
    .collect();

  let iter = monitors.into_iter().enumerate().filter_map(move |(index, monitor)| {
    let mut geometry: MonitorGeometry = monitor.size;

    let hmonitor = HMONITOR(monitor.hmonitor);

    let mut scale_factor: MonitorScaleFactor = MonitorScaleFactor::default();

    match get_monitor_scaling(hmonitor) {
      Ok(scaling) => scale_factor = scaling,
      Err(_) => {
        log::error!("Failed to get DPI for {}", monitor.name.clone())
      }
    }

    // Due to how gdk4 handles Win10 vs Win11
    if get_windows_version() < 11 {
      geometry /= scale_factor;
    }

    // Find matching MonitorTemp based on MonitorGeometry
    if let Some(other_monitor) = other_monitors
      .iter()
      .find(|&m| Into::<MonitorGeometry>::into(m.geometry()) == geometry)
    {
      Some(Monitor {
        connecter: other_monitor.connector().map(|s| s.to_string()),
        description: other_monitor.description().map(|s| s.to_string()),
        geometry,
        manufacturer: other_monitor.manufacturer().map(|s| s.to_string()),
        model: other_monitor.model().map(|s| s.to_string()),
        refresh_rate: other_monitor.refresh_rate(),
        is_primary: geometry.x == 0 && geometry.y == 0,
        device: monitor.device.clone(),
        device_id: monitor.device_id.clone(),
        id: monitor.hmonitor,
        name: monitor.name.clone(),
        scale_factor,
        index,
      })
    } else {
      panic!("No matching monitor found for geometry: {:?}", geometry);
    }
  });

  iter
}

struct MonitorUserData {}

impl MonitorUserData {
  fn new() -> Self {
    MonitorUserData {}
  }
}

impl UserData for MonitorUserData {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_function("get_all", all);
    methods.add_function("get_primary", primary);
  }
}

fn all<'lua>(lua: &'lua Lua, _: Value) -> mlua::Result<Value<'lua>> {
  let monitors_vec: Vec<Monitor> = get_monitors().collect();

  let res = lua.to_value(&monitors_vec)?;

  Ok(res)
}

// @todo @codyduong
fn current() {}

fn primary<'lua>(lua: &'lua Lua, _: Value) -> mlua::Result<Value<'lua>> {
  let monitors_vec: Option<Monitor> = get_monitors().find(|m| m.geometry.x == 0 && m.geometry.y == 0);

  let res = lua.to_value(&monitors_vec)?;

  Ok(res)
}

pub fn make<'lua>(lua: &'lua Lua) -> anyhow::Result<AnyUserData<'lua>> {
  let userdata = lua.create_userdata(MonitorUserData::new()).unwrap();

  Ok(userdata)
}

// tests
#[cfg(test)]
mod tests {
  use mlua::{AnyUserData, AnyUserDataExt, Lua, Table, UserData, Value};

  use crate::assert_lua_type;

  fn create_userdata(lua: &Lua) -> anyhow::Result<AnyUserData> {
    return super::make(lua);
  }

  #[test]
  fn test_all() -> anyhow::Result<()> {
    {
      let lua = Lua::new();
      let userdata = create_userdata(&lua)?;
      lua.globals().set("userdata", userdata)?;
      let value: Value = lua.globals().get("userdata")?;

      let userdata: AnyUserData = assert_lua_type!(value, AnyUserData);
      // let metatable = userdata.get_metatable()?;
      assert_lua_type!(userdata.get::<&str, Value>("get_all")?, mlua::Function);
      assert_lua_type!(userdata.get::<&str, Value>("get_primary")?, mlua::Function);
      // assert_lua_type!(metatable.get("__call")?, mlua::Function);
      // assert_eq!(table.len()?, 0);
      // assert_eq!(table.pairs::<String, Value>().count(), 2);
    }

    Ok(())
  }
}
