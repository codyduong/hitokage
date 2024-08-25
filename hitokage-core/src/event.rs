use std::collections::VecDeque;

use relm4::SharedState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventNotif {
  // this stores the previous state just in case we need it
  pub state: serde_json::Value,
  pub event: serde_json::Value,
}

pub static STATE: SharedState<serde_json::Value> = SharedState::new(); // this only stores the newest state
pub static EVENT: SharedState<VecDeque<EventNotif>> = SharedState::new();
pub static NEW_EVENT: SharedState<bool> = SharedState::new(); // if the state has changed since we last read the state
pub static CONFIG_UPDATE: SharedState<bool> = SharedState::new(); // if we updated init.lua
