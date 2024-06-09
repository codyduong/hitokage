pub mod bar;
pub mod clock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Widgets {
  Clock(clock::ClockProps)
}