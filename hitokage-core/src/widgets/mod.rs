pub mod bar;
pub mod r#box;
pub mod clock;
pub mod workspace;

use clock::{Clock, ClockMsg};
use r#box::Box;
use r#box::BoxMsg;
use relm4::ComponentController;
use relm4::Controller;
use serde::{Deserialize, Serialize};
use workspace::{Workspace, WorkspaceMsg};

#[derive(Debug, Deserialize, Serialize)]
pub enum WidgetProps {
  Clock(clock::ClockProps),
  Workspace(workspace::WorkspaceProps),
  Box(r#box::BoxProps),
}

pub enum WidgetController {
  Clock(Controller<Clock>),
  Workspace(Controller<Workspace>),
  Box(Controller<Box>),
}

#[derive(Debug, Clone)]
pub enum WidgetUserData {
  Clock(relm4::Sender<ClockMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
  Box(relm4::Sender<BoxMsg>),
}

impl<'a> From<&'a WidgetController> for WidgetUserData {
  fn from(controller: &'a WidgetController) -> Self {
    match controller {
      WidgetController::Clock(item) => WidgetUserData::Clock(item.sender().clone()),
      WidgetController::Workspace(item) => WidgetUserData::Workspace(item.sender().clone()),
      WidgetController::Box(item) => WidgetUserData::Box(item.sender().clone()),
    }
  }
}
