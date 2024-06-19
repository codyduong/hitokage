use std::ops::{Div, Mul};

use crate::win_utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MonitorGeometry {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl Default for MonitorGeometry {
  fn default() -> Self {
    MonitorGeometry {
      x: 0,
      y: 0,
      width: win_utils::get_primary_width(),
      height: win_utils::get_primary_width(),
    }
  }
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

impl From<windows::Win32::Foundation::RECT> for MonitorGeometry {
  fn from(rect: windows::Win32::Foundation::RECT) -> Self {
    MonitorGeometry {
      x: rect.left,
      y: rect.top,
      width: rect.right - rect.left,
      height: rect.bottom - rect.top,
    }
  }
}

impl PartialEq for MonitorGeometry {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
  }
}

impl Div<MonitorScaleFactor> for MonitorGeometry {
  type Output = MonitorGeometry;

  fn div(self, scale: MonitorScaleFactor) -> MonitorGeometry {
      MonitorGeometry {
          x: (self.x as f32 / scale.x).round() as i32,
          y: (self.y as f32 / scale.y).round() as i32,
          width: (self.width as f32 / scale.x).round() as i32,
          height: (self.height as f32 / scale.y).round() as i32,
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
  pub device: String,
  pub device_id: String,
  pub id: isize,
  pub name: String,
  pub scale_factor: MonitorScaleFactor,
  // @codyduong If this ends up being something someone needs... but you can usually just match with a komorebi state if you really need this...
  // pub size: windows::Win32::Foundation::RECT,
  // pub work_area_size: windows::Win32::Foundation::RECT,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MonitorScaleFactor {
  pub x: f32,
  pub y: f32,
}

impl Default for MonitorScaleFactor {
  fn default() -> Self {
    MonitorScaleFactor {
      x: 1.0,
      y: 1.0,
    }
  }
}