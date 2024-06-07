use relm4::SharedState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventNotif {
  // this stores the previous state just in case we need it
  pub state: serde_json::Value,
  pub event: serde_json::Value,
}

// pub static STATE: SharedState<Option<komorebi_client::State>> = SharedState::new();
pub static STATE: SharedState<serde_json::Value> = SharedState::new(); // this only stores the newest state
pub static EVENT: SharedState<Vec<EventNotif>> = SharedState::new();
pub static NEW_EVENT: SharedState<bool> = SharedState::new(); // if the state has changed since we last read the state

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MonitorGeometry {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl From<gdk4::Rectangle> for MonitorGeometry {
  fn from(item: gdk4::Rectangle) -> Self {
    MonitorGeometry {
      x: item.x(),
      y: item.y(),
      width: item.width(),
      height: item.height(),
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct Monitor {
  pub connecter: Option<String>,
  pub description: Option<String>,
  pub geometry: MonitorGeometry,
  pub manufacturer: Option<String>,
  pub model: Option<String>,
  pub refresh_rate: i32,
  pub is_primary: bool,
}