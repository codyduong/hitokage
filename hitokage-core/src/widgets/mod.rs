pub mod bar;
pub mod r#box;
pub mod clock;
pub mod workspace;

use clock::Clock;
use relm4::Controller;
use serde::{Deserialize, Serialize};
use workspace::Workspace;

#[derive(Debug, Deserialize, Serialize)]
pub enum Widget {
  Clock(clock::ClockProps),
  Workspace(workspace::WorkspaceProps),
  Box(r#box::BoxProps),
}

pub enum WidgetController {
  Clock(Controller<Clock>),
  Workspace(Controller<Workspace>),
}
