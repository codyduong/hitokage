use serde::{Deserialize, Serialize};

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
