pub mod app;
pub mod bar;
pub mod base;
pub mod battery;
pub mod r#box;
pub mod clock;
pub mod cpu;
pub mod icon;
pub mod label;
pub mod memory;
pub mod weather;
pub mod workspace;

use battery::Battery;
use battery::BatteryMsg;
use clock::Clock;
use clock::ClockMsg;
use cpu::Cpu;
use cpu::CpuMsg;
use icon::Icon;
use icon::IconMsg;
use label::Label;
use label::LabelMsg;
use memory::Memory;
use memory::MemoryMsg;
use r#box::BoxMsg;
use r#box::HitokageBox;
use relm4::component::AsyncComponentController;
use relm4::prelude::AsyncController;
use relm4::ComponentController;
use relm4::Controller;
use serde::de;
use serde::{Deserialize, Serialize};
use std::fmt;
use weather::Weather;
use weather::WeatherMsg;
use workspace::{Workspace, WorkspaceMsg};

#[derive(Debug, Deserialize, Serialize)]
pub enum WidgetProps {
  Battery(battery::BatteryProps),
  Box(r#box::BoxProps),
  Clock(clock::ClockProps),
  Cpu(cpu::CpuProps),
  Icon(icon::IconProps),
  Label(label::LabelProps),
  Memory(memory::MemoryProps),
  Weather(weather::WeatherProps),
  Workspace(workspace::WorkspaceProps),
}

pub enum WidgetController {
  Battery(AsyncController<Battery>),
  Box(Controller<HitokageBox>),
  Clock(Controller<Clock>),
  Cpu(Controller<Cpu>),
  Icon(Controller<Icon>),
  Label(Controller<Label>),
  Memory(Controller<Memory>),
  Weather(AsyncController<Weather>),
  Workspace(Controller<Workspace>),
}

#[derive(Debug, Clone)]
pub enum WidgetUserData {
  Battery(relm4::Sender<BatteryMsg>),
  Box(relm4::Sender<BoxMsg>),
  Clock(relm4::Sender<ClockMsg>),
  Cpu(relm4::Sender<CpuMsg>),
  Icon(relm4::Sender<IconMsg>),
  Label(relm4::Sender<LabelMsg>),
  Memory(relm4::Sender<MemoryMsg>),
  Weather(relm4::Sender<WeatherMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
}

impl<'a> From<&'a WidgetController> for WidgetUserData {
  fn from(controller: &'a WidgetController) -> Self {
    match controller {
      WidgetController::Battery(item) => WidgetUserData::Battery(item.sender().clone()),
      WidgetController::Box(item) => WidgetUserData::Box(item.sender().clone()),
      WidgetController::Clock(item) => WidgetUserData::Clock(item.sender().clone()),
      WidgetController::Cpu(item) => WidgetUserData::Cpu(item.sender().clone()),
      WidgetController::Icon(item) => WidgetUserData::Icon(item.sender().clone()),
      WidgetController::Label(item) => WidgetUserData::Label(item.sender().clone()),
      WidgetController::Memory(item) => WidgetUserData::Memory(item.sender().clone()),
      WidgetController::Weather(item) => WidgetUserData::Weather(item.sender().clone()),
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
