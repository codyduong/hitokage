use crate::common::MonitorGeometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum BarPosition {
  Top,
  Bottom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BarProps {
  pub position: Option<BarPosition>,
  pub geometry: Option<MonitorGeometry>,
}
