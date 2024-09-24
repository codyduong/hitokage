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
use r#box::BoxMsgPortable;
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
pub enum Child {
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

pub enum ChildController {
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

impl ChildController {
  fn widget(&self) -> gtk4::Widget {
    match self {
      ChildController::Battery(c) => c.widget().clone().into(),
      ChildController::Box(c) => c.widget().clone().into(),
      ChildController::Clock(c) => c.widget().clone().into(),
      ChildController::Cpu(c) => c.widget().clone().into(),
      ChildController::Icon(c) => c.widget().clone().into(),
      ChildController::Label(c) => c.widget().clone().into(),
      ChildController::Memory(c) => c.widget().clone().into(),
      ChildController::Weather(c) => c.widget().clone().into(),
      ChildController::Workspace(c) => c.widget().clone().into(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ChildUserData {
  Battery(relm4::Sender<BatteryMsg>),
  Box(relm4::Sender<BoxMsgPortable>),
  Clock(relm4::Sender<ClockMsg>),
  Cpu(relm4::Sender<CpuMsg>),
  Icon(relm4::Sender<IconMsg>),
  Label(relm4::Sender<LabelMsg>),
  Memory(relm4::Sender<MemoryMsg>),
  Weather(relm4::Sender<WeatherMsg>),
  Workspace(relm4::Sender<WorkspaceMsg>),
}

impl<'a> From<&'a ChildController> for ChildUserData {
  fn from(controller: &'a ChildController) -> Self {
    match controller {
      ChildController::Battery(item) => ChildUserData::Battery(item.sender().clone()),
      ChildController::Box(item) => {
        let (sender, receiver) = relm4::channel::<BoxMsgPortable>();
        relm4::spawn_local(receiver.forward(item.sender().clone(), |m| m.into()));
        ChildUserData::Box(sender)
      }
      ChildController::Clock(item) => ChildUserData::Clock(item.sender().clone()),
      ChildController::Cpu(item) => ChildUserData::Cpu(item.sender().clone()),
      ChildController::Icon(item) => ChildUserData::Icon(item.sender().clone()),
      ChildController::Label(item) => ChildUserData::Label(item.sender().clone()),
      ChildController::Memory(item) => ChildUserData::Memory(item.sender().clone()),
      ChildController::Weather(item) => ChildUserData::Weather(item.sender().clone()),
      ChildController::Workspace(item) => ChildUserData::Workspace(item.sender().clone()),
    }
  }
}

pub(crate) fn deserialize_empty_or_seq<'de, D>(deserializer: D) -> Result<Option<Vec<Child>>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct SeqOrEmpty;

  impl<'de> serde::de::Visitor<'de> for SeqOrEmpty {
    type Value = Option<Vec<Child>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a sequence or an empty table")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
      A: de::SeqAccess<'de>,
    {
      let vec: Vec<Child> = Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
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
