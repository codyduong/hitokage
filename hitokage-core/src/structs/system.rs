use crate::handlebar::register_hitokage_helpers;
use handlebars::Handlebars;
use relm4::tokio::sync::Semaphore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use systemstat::Platform;
use systemstat::System;

#[derive(Clone)]
pub struct SystemWrapper {
  system: Arc<System>,
  last_battery: Arc<Mutex<Option<(Instant, systemstat::BatteryLife)>>>,
  battery_semaphore: Arc<Semaphore>,
}

impl fmt::Debug for SystemWrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("SystemWrapper")
      .field("system", &"todo")
      .field("last_battery", &self.last_battery)
      .field("battery_semaphore", &self.battery_semaphore)
      .finish()
  }
}

impl SystemWrapper {
  pub fn new() -> Self {
    Self {
      system: Arc::new(System::new()),
      last_battery: Arc::new(Mutex::new(None)),
      battery_semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  pub async fn battery_life(&self) -> anyhow::Result<systemstat::BatteryLife> {
    let _permit = self.battery_semaphore.acquire().await.unwrap();

    let now = Instant::now();

    if let Some(last_battery) = self.last_battery.lock().unwrap().as_ref() {
      if now.duration_since(last_battery.0) < Duration::from_millis(500) {
        log::debug!("Using cached battery");
        return Ok(last_battery.1.clone().into());
      }
    }

    log::debug!("Getting battery information");

    let battery = self.system.battery_life()?;

    let mut last_battery_guard = self.last_battery.lock().unwrap();
    *last_battery_guard = Some((Instant::now(), battery.clone()));

    Ok(battery)
  }
}

#[derive(Debug, Clone)]
pub struct BatteryWrapper {
  battery: Option<systemstat::BatteryLife>,
}

impl Default for BatteryWrapper {
  fn default() -> Self {
    Self { battery: None }
  }
}

impl BatteryWrapper {
  pub fn format_with(&self, icons: &BatteryIcons, format: &String) -> String {
    let reg = register_hitokage_helpers(Handlebars::new());

    let mut args = HashMap::new();

    if let Some(battery) = &self.battery {
      let time = battery.remaining_time;
      let capacity = battery.remaining_capacity;
      let capacity_icon = match capacity {
        0.0 => icons.p0.clone(),
        0.0..=0.1 => icons.p10.clone(),
        0.1..=0.2 => icons.p20.clone(),
        0.2..=0.3 => icons.p30.clone(),
        0.3..=0.4 => icons.p40.clone(),
        0.4..=0.5 => icons.p50.clone(),
        0.5..=0.6 => icons.p60.clone(),
        0.6..=0.7 => icons.p70.clone(),
        0.7..=0.8 => icons.p80.clone(),
        0.8..=0.9 => icons.p90.clone(),
        0.9..=1.0 => icons.p100.clone(),
        _ => icons.unknown.clone(),
      };

      args.insert("icon".to_string(), capacity_icon);
      args.insert("capacity".to_string(), capacity.to_string());
      args.insert("seconds_left".to_string(), time.as_secs().to_string());
    } else {
      args.insert("icon".to_string(), icons.unknown.clone());
    }

    match reg.render_template(format, &args) {
      Ok(name) => return name,
      Err(err) => {
        log::error!("{:?}", err);
      }
    };

    "".to_owned()
  }
}

impl PartialEq for BatteryWrapper {
  fn ne(&self, other: &Self) -> bool {
    self.battery.clone().zip(other.battery.clone()).map_or(false, |(a, b)| {
      a.remaining_capacity != b.remaining_capacity || a.remaining_time != b.remaining_time
    })
  }

  fn eq(&self, other: &Self) -> bool {
    !self.ne(other)
  }
}

impl From<anyhow::Result<systemstat::BatteryLife>> for BatteryWrapper {
  fn from(value: anyhow::Result<systemstat::BatteryLife>) -> Self {
    match value {
      Ok(value) => BatteryWrapper { battery: Some(value) },
      Err(err) => {
        log::error!("Failed to fetch battery information: {}", err);
        BatteryWrapper { battery: None }
      }
    }
  }
}

impl From<systemstat::BatteryLife> for BatteryWrapper {
  fn from(value: systemstat::BatteryLife) -> Self {
    BatteryWrapper { battery: Some(value) }
  }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BatteryIcons {
  #[serde(default)]
  p0: String,
  #[serde(default)]
  p10: String,
  #[serde(default)]
  p20: String,
  #[serde(default)]
  p30: String,
  #[serde(default)]
  p40: String,
  #[serde(default)]
  p50: String,
  #[serde(default)]
  p60: String,
  #[serde(default)]
  p70: String,
  #[serde(default)]
  p80: String,
  #[serde(default)]
  p90: String,
  #[serde(default)]
  p100: String,
  #[serde(default)]
  p0_charging: String,
  #[serde(default)]
  p10_charging: String,
  #[serde(default)]
  p20_charging: String,
  #[serde(default)]
  p30_charging: String,
  #[serde(default)]
  p40_charging: String,
  #[serde(default)]
  p50_charging: String,
  #[serde(default)]
  p60_charging: String,
  #[serde(default)]
  p70_charging: String,
  #[serde(default)]
  p80_charging: String,
  #[serde(default)]
  p90_charging: String,
  #[serde(default)]
  p100_charging: String,
  #[serde(default)]
  unknown: String,
}

impl Default for BatteryIcons {
  fn default() -> Self {
    Self {
      p0: "\u{F008E}".to_string(),
      p10: "\u{F007A}".to_string(),
      p20: "\u{F007B}".to_string(),
      p30: "\u{F007C}".to_string(),
      p40: "\u{F007D}".to_string(),
      p50: "\u{F007E}".to_string(),
      p60: "\u{F007F}".to_string(),
      p70: "\u{F0080}".to_string(),
      p80: "\u{F0081}".to_string(),
      p90: "\u{F0082}".to_string(),
      p100: "\u{F0079}".to_string(),
      p0_charging: "\u{F089F}".to_string(),
      p10_charging: "\u{F089C}".to_string(),
      p20_charging: "\u{F0086}".to_string(),
      p30_charging: "\u{F0087}".to_string(),
      p40_charging: "\u{F0088}".to_string(),
      p50_charging: "\u{F089D}".to_string(),
      p60_charging: "\u{F0089}".to_string(),
      p70_charging: "\u{F089E}".to_string(),
      p80_charging: "\u{F008A}".to_string(),
      p90_charging: "\u{F008B}".to_string(),
      p100_charging: "\u{F0085}".to_string(),
      unknown: "\u{F0091}".to_string(),
    }
  }
}
