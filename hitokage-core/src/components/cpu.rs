use super::app::AppMsg;
use super::base::Base;
use super::base::BaseProps;
use super::r#box::BoxMsg;
use crate::components::base::BaseMsgHook;
use crate::generate_base_match_arms;
use crate::handlebar::register_hitokage_helpers;
use crate::prepend_css_class;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::lua_fn::LuaFn;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::AsReactive;
use crate::structs::reactive::Reactive;
use crate::structs::reactive_string_fn::ReactiveStringFn;
use gtk4::prelude::*;
use handlebars::Handlebars;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use systemstat::CPULoad;
use systemstat::DelayedMeasurement;
use systemstat::Platform;
use systemstat::System;

#[derive(Debug, Clone)]
pub enum CpuMsgHook {
  BaseHook(BaseMsgHook),
  GetFormat(Sender<String>),
  GetFormatReactive(Sender<Reactive<String>>),
  SetFormat(String),
}

#[derive(Debug, Clone)]
pub enum CpuMsg {
  LuaHook(CpuMsgHook),
  React,
  Tick,
  Callback(std::sync::mpsc::Sender<mlua::Value>)
}

#[derive(Debug)]
pub enum CpuMsgOut {
  RequestLuaAction(
    Arc<mlua::RegistryKey>,
    serde_json::Value,
    std::sync::mpsc::Sender<mlua::Value>,
  ),
}

impl From<CpuMsgOut> for AppMsg {
  fn from(value: CpuMsgOut) -> Self {
    match value {
      CpuMsgOut::RequestLuaAction(a, b, c) => AppMsg::RequestLuaAction(a, b, c),
      #[allow(unreachable_patterns)]
      _ => AppMsg::NoOp,
    }
  }
}

impl From<CpuMsgOut> for BoxMsg {
  fn from(value: CpuMsgOut) -> Self {
    match value {
      CpuMsgOut::RequestLuaAction(a, b, c) => BoxMsg::AppMsg(AppMsg::RequestLuaAction(a, b, c)),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct CpuProps {
  #[serde(flatten)]
  base: BaseProps,
  format: ReactiveStringFn,
}

#[tracker::track]
pub struct Cpu {
  #[tracker::do_not_track]
  base: Base,
  cpu: CPULoadWrapper,
  #[tracker::do_not_track]
  cpu_inflight: std::io::Result<DelayedMeasurement<Vec<CPULoad>>>,
  #[tracker::do_not_track]
  source_id: Option<glib::SourceId>,
  #[tracker::do_not_track]
  format: Reactive<String>,
  react: bool,
  #[tracker::do_not_track]
  callback: Option<LuaFn>,
}

#[relm4::component(pub)]
impl Component for Cpu {
  type Input = CpuMsg;
  type Output = CpuMsgOut;
  type Init = CpuProps;
  type Widgets = CpuWidgets;
  type CommandOutput = ();

  view! {
    gtk::Label {
      #[track = "model.changed(Cpu::react() | Cpu::cpu())"]
      set_label: format_cpu(&model.format.get(), &model.cpu.clone().into()).as_str(),
    }
  }

  fn init(props: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let callback = props.format.as_fn();
    let reactive = props
      .format
      .as_reactive(create_react_sender(sender.input_sender(), CpuMsg::React));

    let source_id = {
      let sender = sender.clone();
      let reactive = reactive.clone();

      let res = match callback {
        Some(_) => glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
          sender.input(CpuMsg::Tick);
          let (tx, rx) = std::sync::mpsc::channel::<_>();
          sender.input(CpuMsg::Callback(tx.clone()));
          match rx.try_recv() {
            Ok(v) => match v {
              mlua::Value::String(s) => {
                reactive.set(s.to_string_lossy());
              }
              _ => {
                log::error!("Expected string for battery callback, received: {:?}", v);
              }
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
              log::error!("Battery callback dropped");
            }
          }
          glib::ControlFlow::Continue
        }),
        None => glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
          sender.input(CpuMsg::Tick);
          glib::ControlFlow::Continue
        }),
      };

      res
    };

    let sys = System::new();

    let mut model = Cpu {
      base: props.base.clone().into(),
      cpu: CPULoadWrapper::new(Vec::new()),
      cpu_inflight: sys.cpu_load(),
      source_id: Some(source_id),
      format: reactive,
      callback,
      react: false,
      tracker: 0,
    };

    prepend_css_class_to_model!("cpu", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    root.show();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      CpuMsg::LuaHook(hook) => match hook {
        CpuMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "format", root, base)
        }
        CpuMsgHook::GetFormat(tx) => {
          tx.send(self.format.get()).unwrap();
        }
        CpuMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        CpuMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
          self.set_react(!self.react);
        }
      },
      CpuMsg::React => {
        self.set_react(!self.react);
      }
      CpuMsg::Tick => {
        match &self.cpu_inflight {
          Ok(res) => {
            let res = match res.done() {
              Ok(res) => res,
              Err(err) => {
                log::error!("Failed to obtain CPU usage information: {}", err);
                Vec::new()
              }
            };
            self.base.classes_temp = generate_cpu_classes(&res);
            let joined = prepend_css_class!(self.base.classes.clone(), self.base.classes_temp.clone());
            let classes_ref: Vec<&str> = joined.iter().map(AsRef::as_ref).collect();
            root.set_css_classes(&classes_ref);
            self.set_cpu(res.into());
          }
          Err(err) => {
            log::error!("Failed to obtain CPU usage information: {}", err);
          }
        };
      }
      CpuMsg::Callback(tx) => {
        if let Some(callback) = &self.callback {
          let _ = sender.output(CpuMsgOut::RequestLuaAction(
            callback.r.clone(),
            serde_json::to_value(&self.cpu.as_lua_args()).unwrap(),
            tx.clone(),
          ));
        }
      }
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
    self.source_id.take().map(glib::SourceId::remove);
  }
}

#[derive(Debug, Clone)]
struct CPULoadWrapper {
  cpu_loads: Vec<CPULoad>,
}

impl CPULoadWrapper {
  fn new(value: impl Into<Vec<CPULoad>>) -> Self {
    CPULoadWrapper {
      cpu_loads: value.into(),
    }
  }

  fn as_lua_args(&self) -> CpuLoadInfo {
    let mut total_user = 0.0;
    let mut total_nice = 0.0;
    let mut total_system = 0.0;
    let mut total_interrupt = 0.0;
    let mut total_idle = 0.0;
    let mut overall_usage = 0.0;
    let core_count = self.cpu_loads.len();
    let mut cores: Vec<CpuLoadCoreInfo> = Vec::new();

    for cpu in &self.cpu_loads {
      total_user += cpu.user;
      total_nice += cpu.nice;
      total_system += cpu.system;
      total_interrupt += cpu.interrupt;
      total_idle += cpu.idle;
  
      let total_usage = 1.0 - cpu.idle;
      overall_usage += total_usage;
  
      cores.push(CpuLoadCoreInfo {
        user: cpu.user,
        nice: cpu.nice,
        system: cpu.system,
        interrupt: cpu.interrupt,
        idle: cpu.idle,
        usage: total_usage,
      });
    }

    let total_usage = overall_usage / core_count as f32;

    CpuLoadInfo {
      cores,
      user: total_user,
      nice: total_nice,
      system: total_system,
      interrupt: total_interrupt,
      idle: total_idle,
      usage: total_usage,
    }
  }
}

impl PartialEq for CPULoadWrapper {
  fn ne(&self, other: &Self) -> bool {
    self
      .cpu_loads
      .clone()
      .last()
      .zip(other.cpu_loads.clone().last())
      .map_or(false, |(a, b)| {
        (a.user != b.user)
          || (a.nice != b.nice)
          || (a.system != b.system)
          || (a.interrupt != b.interrupt)
          || (a.idle != b.idle)
      })
  }

  fn eq(&self, other: &Self) -> bool {
    !self.ne(other)
  }
}

impl From<Vec<CPULoad>> for CPULoadWrapper {
  fn from(value: Vec<CPULoad>) -> Self {
    CPULoadWrapper { cpu_loads: value }
  }
}

impl Into<Vec<CPULoad>> for CPULoadWrapper {
  fn into(self) -> Vec<CPULoad> {
    self.cpu_loads
  }
}

#[derive(Debug, Clone, Serialize)]
struct CpuLoadCoreInfo {
  user: f32,
  nice: f32,
  system: f32,
  interrupt: f32,
  idle: f32,
  usage: f32,
}

#[derive(Debug, Clone, Serialize)]
struct CpuLoadInfo {
  cores: Vec<CpuLoadCoreInfo>,
  user: f32,
  nice: f32,
  system: f32,
  interrupt: f32,
  idle: f32,
  usage: f32,
}

fn format_cpu(format: &String, cpu_loads: &Vec<CPULoad>) -> String {
  let reg = register_hitokage_helpers(Handlebars::new());

  let mut args = HashMap::new();
  let mut total_user = 0.0;
  let mut total_nice = 0.0;
  let mut total_system = 0.0;
  let mut total_interrupt = 0.0;
  let mut total_idle = 0.0;
  let mut overall_usage = 0.0;
  let mut cores = 64;

  for i in 0..cores {
    args.insert(format!("core{}_user", i), 0.0);
    args.insert(format!("core{}_nice", i), 0.0);
    args.insert(format!("core{}_system", i), 0.0);
    args.insert(format!("core{}_interrupt", i), 0.0);
    args.insert(format!("core{}_idle", i), 0.0);
    args.insert(format!("core{}_usage", i), 0.0);
  }

  cores = cpu_loads.len();

  for (index, cpu) in cpu_loads.iter().enumerate() {
    total_user += cpu.user;
    total_nice += cpu.nice;
    total_system += cpu.system;
    total_interrupt += cpu.interrupt;
    total_idle += cpu.idle;

    args.insert(format!("core{}_user", index), cpu.user);
    args.insert(format!("core{}_nice", index), cpu.nice);
    args.insert(format!("core{}_system", index), cpu.system);
    args.insert(format!("core{}_interrupt", index), cpu.interrupt);
    args.insert(format!("core{}_idle", index), cpu.idle);

    let overall_core_usage = 1.0 - cpu.idle;
    overall_usage += overall_core_usage;

    args.insert(format!("core{}_usage", index), overall_core_usage);
  }

  let overall_cpu_usage = overall_usage / cores as f32;

  args.insert("user".to_string(), total_user);
  args.insert("nice".to_string(), total_nice);
  args.insert("system".to_string(), total_system);
  args.insert("interrupt".to_string(), total_interrupt);
  args.insert("idle".to_string(), total_idle);
  args.insert(
    "usage".to_string(),
    if overall_cpu_usage.is_nan() {
      0.0
    } else {
      overall_cpu_usage
    },
  );

  match reg.render_template(format, &args) {
    Ok(name) => return name,
    Err(err) => {
      log::error!("{:?}", err);
    }
  };

  "".to_owned()
}

// prepend numbers
fn generate_cpu_classes(cpu_loads: &Vec<CPULoad>) -> Vec<String> {
  let mut class_names = vec!["cpu".to_string()];
  let mut total_usage = 0.0;
  let num_cores = cpu_loads.len();

  for (index, cpu) in cpu_loads.iter().enumerate() {
    let usage = 1.0 - cpu.idle;
    let usage_percentage = (usage * 100.0).trunc() as i32;

    total_usage += usage;

    for percent in (10..=usage_percentage).step_by(10) {
      class_names.push(format!("core{}-usage-{}", index, percent));
    }
  }

  let overall_usage = total_usage / num_cores as f32;
  let overall_usage_percentage = (overall_usage * 100.0).trunc() as i32;

  for percent in (10..=overall_usage_percentage).step_by(10) {
    class_names.push(format!("usage-{}", percent));
  }

  class_names
}
