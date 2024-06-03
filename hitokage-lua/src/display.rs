use mlua::{Lua, LuaSerdeExt, Value, Variadic};
use serde::Serialize;
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{BOOL, LPARAM},
    Graphics::Gdi::{
      EnumDisplayDevicesW, EnumDisplaySettingsW, GetDeviceCaps, GetMonitorInfoW, DEVMODEW,
      DISPLAY_DEVICEW, ENUM_CURRENT_SETTINGS, HDC, HORZRES, MONITORINFOEXW, VERTRES,
    },
  },
};

#[repr(C)]
#[derive(Debug)]
struct MonitorInfoExtern {
  width: u32,
  height: u32,
  name: [u16; 32],
}

#[derive(Serialize)]
struct MonitorInfo {
  width: u32,
  height: u32,
  id: u32,
  name: String,
}

fn all() -> Vec<MonitorInfo> {
  let mut monitors: Vec<MonitorInfo> = Vec::new();
  unsafe {
    for monitor in unsafe_all() {
      monitors.push(MonitorInfo {
        width: monitor.width,
        height: monitor.height,
        id: 0,
        name: String::from_utf16_lossy(&monitor.name),
      })
    }
  }
  return monitors;
}

unsafe extern "system" fn unsafe_all() -> Vec<MonitorInfoExtern> {
  let mut monitors = Vec::new();
  let mut device_num = 0;
  let mut display_device: DISPLAY_DEVICEW = std::mem::zeroed();
  display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

  while EnumDisplayDevicesW(PCWSTR::null(), device_num, &mut display_device, 0).as_bool() {
    let mut dev_mode: DEVMODEW = std::mem::zeroed();
    dev_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

    if EnumDisplaySettingsW(
      PCWSTR(display_device.DeviceName.as_ptr()),
      ENUM_CURRENT_SETTINGS,
      &mut dev_mode,
    )
    .as_bool()
    {
      let width = dev_mode.dmPelsWidth;
      let height = dev_mode.dmPelsHeight;

      monitors.push(MonitorInfoExtern {
        width,
        height,
        name: display_device.DeviceName,
        // @codyduong - TODO add primary here from reading display_device.StateFlags
      });
    }

    device_num += 1;
  }

  monitors
}

unsafe extern "system" fn monitor_enum_proc(
  hmonitor: windows::Win32::Graphics::Gdi::HMONITOR,
  hdc: HDC,
  _lprect: *mut windows::Win32::Foundation::RECT,
  _lparam: LPARAM,
) -> BOOL {
  let mut monitor_info: MONITORINFOEXW = std::mem::zeroed();
  monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

  if GetMonitorInfoW(hmonitor, &mut monitor_info as *mut MONITORINFOEXW as *mut _).as_bool() {
    let device_name = PCWSTR::from_raw(monitor_info.szDevice.as_ptr());
    let width = GetDeviceCaps(hdc, HORZRES);
    let height = GetDeviceCaps(hdc, VERTRES);

    println!("Monitor: {:?}", device_name);
    println!("Width: {}", width);
    println!("Height: {}", height);
  }

  true.into()
}

fn current() {}

fn primary() {}

pub fn make_display<'lua>(lua: &'lua Lua) -> anyhow::Result<mlua::Table<'lua>> {
  let hitokage_display = lua.create_table()?;

  hitokage_display.set(
    "all",
    lua.create_function(|lua, _args: Variadic<Value>| {
      let repr = all();
      Ok(lua.to_value(&repr))
    })?,
  )?;

  hitokage_display.set("current", lua.create_function(|_, _args: Variadic<Value>| Ok(()))?)?;

  hitokage_display.set("primary", lua.create_function(|_, _args: Variadic<Value>| Ok(()))?)?;

  Ok(hitokage_display)
}
