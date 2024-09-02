use super::bar::BarMsg;
use super::bar::BarProps;
use super::weather::WeatherStation;
use super::weather::WeatherStationConfig;
use crate::event::EventNotif;
use crate::structs::Monitor;
use systemstat::System;

#[derive(Debug)]
pub enum AppMsg {
  Komorebi(EventNotif),
  KomorebiErr(String),
  LuaHook(LuaHook),
  DestroyActual,
  RequestWeatherStation(
    relm4::tokio::sync::oneshot::Sender<WeatherStation>,
    Option<WeatherStationConfig>,
  ),
  DropWeatherStation,
  RequestBatteryLife(relm4::tokio::sync::oneshot::Sender<systemstat::BatteryLife>),
}

pub enum LuaHookType {
  SubscribeState, // subscribe to a value in global state
  WriteState,     //
  ReadEvent,      // This should probably exclusively be used for initializing configurations, it does not subscribe!
  CreateBar(Box<Monitor>, BarProps, Box<dyn Fn(relm4::Sender<BarMsg>) + Send>),
  CheckConfigUpdate,
  NoAction, // These hooks are used for Relm4 hooking into, so it is very possible we don't need to handle anything
}

impl std::fmt::Debug for LuaHookType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LuaHookType::SubscribeState => write!(f, "SubscribeState"),
      LuaHookType::WriteState => write!(f, "WriteState"),
      LuaHookType::ReadEvent => write!(f, "ReadEvent"),
      LuaHookType::CreateBar(monitor, props, _) => f
        .debug_struct("CreateBar")
        .field("monitor", monitor)
        .field("props", props)
        .field("callback", &"<function>")
        .finish(),
      &LuaHookType::CheckConfigUpdate => write!(f, "CheckConfigUpdate"),
      LuaHookType::NoAction => write!(f, "NoAction"),
    }
  }
}

#[derive(Debug)]
pub struct LuaHook {
  pub t: LuaHookType,
}
