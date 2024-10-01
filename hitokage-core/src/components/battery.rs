use super::app::AppMsg;
use super::base::Base;
use super::base::BaseProps;
use super::r#box::BoxMsg;
use super::r#box::ChildMsg;
use crate::components::base::BaseMsgHook;
use crate::generate_base_match_arms;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::AsReactive;
use crate::structs::reactive::Reactive;
use crate::structs::reactive_string::ReactiveString;
use crate::structs::system::BatteryIcons;
use crate::structs::system::BatteryWrapper;
use crate::structs::system::SystemWrapper;
use gtk4::prelude::*;
use relm4::prelude::*;
use serde::Deserialize;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum BatteryMsgHook {
  BaseHook(BaseMsgHook),
  GetFormat(Sender<String>),
  GetFormatReactive(Sender<Reactive<String>>),
  SetFormat(String),
}

#[derive(Debug, Clone)]
pub enum BatteryMsg {
  LuaHook(BatteryMsgHook),
  React,
  RequestBatteryLife,
}

#[derive(Debug)]
pub enum BatteryMsgOut {
  RequestSystem(relm4::tokio::sync::oneshot::Sender<SystemWrapper>),
  Destroy(gtk4::Widget),
}

impl From<BatteryMsgOut> for AppMsg {
  fn from(value: BatteryMsgOut) -> Self {
    match value {
      BatteryMsgOut::RequestSystem(tx) => AppMsg::RequestSystem(tx),
      _ => AppMsg::NoOp,
    }
  }
}

impl From<BatteryMsgOut> for BoxMsg {
  fn from(value: BatteryMsgOut) -> Self {
    match value {
      BatteryMsgOut::RequestSystem(tx) => BoxMsg::AppMsg(AppMsg::RequestSystem(tx)),
      BatteryMsgOut::Destroy(widget) => BoxMsg::ChildMsg(ChildMsg::Remove(widget)),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct BatteryProps {
  #[serde(flatten)]
  base: BaseProps,
  format: ReactiveString,
  #[serde(default)]
  icons: BatteryIcons,
}

#[tracker::track]
pub struct Battery {
  #[tracker::do_not_track]
  base: Base,
  #[tracker::do_not_track]
  source_id: Option<glib::SourceId>,
  #[tracker::do_not_track]
  format: Reactive<String>,
  #[tracker::do_not_track]
  icons: BatteryIcons,
  #[tracker::do_not_track]
  system: SystemWrapper,
  battery: BatteryWrapper,
  react: bool,
}

#[relm4::component(async, pub)]
impl AsyncComponent for Battery {
  type Input = BatteryMsg;
  type Output = BatteryMsgOut;
  type Init = BatteryProps;
  type Widgets = BatteryWidgets;
  type CommandOutput = ();

  view! {
    gtk::Box {
      #[name="battery"]
      gtk::Label {
        #[track = "model.changed(Battery::react() | Battery::battery())"]
        set_label: &model.battery.clone().format_with(&model.icons, &model.format.get()),
      }
    }
  }

  async fn init(props: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
    let (tx, rx) = relm4::tokio::sync::oneshot::channel::<_>();
    let _ = sender.output(BatteryMsgOut::RequestSystem(tx));
    let system = match rx.await {
      Ok(v) => v,
      Err(err) => {
        log::error!("err: {}", err);
        panic!("rip battery TODO fix")
      }
    };

    let sender_clone = sender.clone();
    let source_id = glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
      sender_clone.input(BatteryMsg::RequestBatteryLife);
      glib::ControlFlow::Continue
    });

    let battery = match system.battery_life().await {
      Ok(b) => b.into(),
      Err(e) => {
        if e.to_string() == "Battery absent" {
          log::info!("No battery detected, removing battery component");
          sender.output(BatteryMsgOut::Destroy(root.clone().into())).unwrap();
        }
        BatteryWrapper::default()
      }
    };

    let mut model = Battery {
      base: props.base.clone().into(),
      source_id: Some(source_id),
      format: props
        .format
        .as_reactive(create_react_sender(sender.input_sender(), BatteryMsg::React)),
      react: false,
      tracker: 0,
      battery,
      system,
      icons: props.icons,
    };

    prepend_css_class_to_model!("battery", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    root.show();

    AsyncComponentParts { model, widgets }
  }

  async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>, root: &Self::Root) {
    match msg {
      BatteryMsg::LuaHook(hook) => match hook {
        BatteryMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "format", root, base)
        }
        BatteryMsgHook::GetFormat(tx) => {
          tx.send(self.format.get()).unwrap();
        }
        BatteryMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        BatteryMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
          self.set_react(!self.react);
        }
      },
      BatteryMsg::React => {
        self.set_react(!self.react);
      }
      BatteryMsg::RequestBatteryLife => {
        self.set_battery(self.system.battery_life().await.into());
      }
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, _sender: relm4::Sender<Self::Output>) {
    self.source_id.take().map(glib::SourceId::remove);
  }
}
