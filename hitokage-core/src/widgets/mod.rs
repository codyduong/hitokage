pub mod bar;
pub mod base;
pub mod r#box;
pub mod clock;
pub mod cpu;
pub mod icon;
pub mod label;
pub mod workspace;

use clock::{Clock, ClockMsg};
use cpu::Cpu;
use cpu::CpuMsg;
use icon::Icon;
use icon::IconMsg;
use label::Label;
use label::LabelMsg;
use r#box::BoxMsg;
use r#box::HitokageBox;
use relm4::ComponentController;
use relm4::Controller;
use serde::de;
use serde::{Deserialize, Serialize};
use std::fmt;
use workspace::{Workspace, WorkspaceMsg};

#[derive(Debug, Deserialize, Serialize)]
pub enum WidgetProps {
  Box(r#box::BoxProps),
  Clock(clock::ClockProps),
  Cpu(cpu::CpuProps),
  Icon(icon::IconProps),
  Label(label::LabelProps),
  Workspace(workspace::WorkspaceProps),
}

pub enum WidgetController {
  Box(Controller<HitokageBox>),
  Clock(Controller<Clock>),
  Cpu(Controller<Cpu>),
  Icon(Controller<Icon>),
  Label(Controller<Label>),
  Workspace(Controller<Workspace>),
}

#[derive(Debug, Clone)]
pub enum WidgetUserData {
  Box(relm4::Sender<BoxMsg>),
  Clock(relm4::Sender<ClockMsg>),
  Cpu(relm4::Sender<CpuMsg>),
  Icon(relm4::Sender<IconMsg>),
  Label(relm4::Sender<LabelMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
}

impl<'a> From<&'a WidgetController> for WidgetUserData {
  fn from(controller: &'a WidgetController) -> Self {
    match controller {
      WidgetController::Box(item) => WidgetUserData::Box(item.sender().clone()),
      WidgetController::Clock(item) => WidgetUserData::Clock(item.sender().clone()),
      WidgetController::Cpu(item) => WidgetUserData::Cpu(item.sender().clone()),
      WidgetController::Icon(item) => WidgetUserData::Icon(item.sender().clone()),
      WidgetController::Label(item) => WidgetUserData::Label(item.sender().clone()),
      WidgetController::Workspace(item) => WidgetUserData::Workspace(item.sender().clone()),
    }
  }
}

pub fn deserialize_empty_or_seq<'de, D>(deserializer: D) -> Result<Option<Vec<WidgetProps>>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct SeqOrEmpty;

  impl<'de> serde::de::Visitor<'de> for SeqOrEmpty {
    type Value = Option<Vec<WidgetProps>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a sequence or an empty table")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
      A: de::SeqAccess<'de>,
    {
      let vec: Vec<WidgetProps> = Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
      Ok(Some(vec))
    }

    fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
    where
      M: de::MapAccess<'de>,
    {
      // If it's an empty map, we treat it as an empty sequence
      Ok(Some(Vec::new()))
    }
  }

  deserializer.deserialize_any(SeqOrEmpty)
}
