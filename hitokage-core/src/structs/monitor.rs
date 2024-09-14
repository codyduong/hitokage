use crate::win_utils;
use mlua::{LuaSerdeExt, MultiValue};
use serde::{Deserialize, Serialize};
use std::ops::DivAssign;

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

impl DivAssign<MonitorScaleFactor> for MonitorGeometry {
  fn div_assign(&mut self, scale: MonitorScaleFactor) {
    self.x = (self.x as f32 / scale.x).round() as i32;
    self.y = (self.y as f32 / scale.y).round() as i32;
    self.width = (self.width as f32 / scale.x).round() as i32;
    self.height = (self.height as f32 / scale.y).round() as i32;
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Monitor {
  pub connector: Option<String>,
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
  pub index: usize,
  // @codyduong If this ends up being something someone needs... but you can usually just match with a komorebi state if you really need this...
  // pub size: windows::Win32::Foundation::RECT,
  // pub work_area_size: windows::Win32::Foundation::RECT,
}

impl mlua::UserData for Monitor {
  fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("field", |_, this| Ok(this.connector.clone()));
    fields.add_field_method_get("description", |_, this| Ok(this.description.clone()));
    fields.add_field_method_get("geometry", |lua, this| lua.to_value(&this.geometry));
    fields.add_field_method_get("manufacturer", |_, this| Ok(this.manufacturer.clone()));
    fields.add_field_method_get("refresh_rate", |_, this| Ok(this.refresh_rate.clone()));
    fields.add_field_method_get("is_primary", |_, this| Ok(this.is_primary.clone()));
    fields.add_field_method_get("device", |_, this| Ok(this.device.clone()));
    fields.add_field_method_get("device_id", |_, this| Ok(this.device_id.clone()));
    fields.add_field_method_get("id", |_, this| Ok(this.id.clone()));
    fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
    fields.add_field_method_get("scale_factor", |lua, this| lua.to_value(&this.scale_factor));
    fields.add_field_method_get("index", |_, this| Ok(this.index.clone()));
  }

  fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("attach", |lua, this, props: mlua::Value| {
      let bar = lua
        .globals()
        .get::<&str, mlua::Table>("hitokage")?
        .get::<&str, mlua::Table>("bar")?
        .get::<&str, mlua::Function>("create")?;

      let args = MultiValue::from_vec(vec![lua.pack(this.clone())?, props]);

      bar.call::<MultiValue<'_>, mlua::Value>(args)
    });
  }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MonitorScaleFactor {
  pub x: f32,
  pub y: f32,
}

impl Default for MonitorScaleFactor {
  fn default() -> Self {
    MonitorScaleFactor { x: 1.0, y: 1.0 }
  }
}
