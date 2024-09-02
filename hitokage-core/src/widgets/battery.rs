use super::app::AppMsg;
use super::base::Base;
use super::base::BaseProps;
use crate::generate_base_match_arms;
use crate::handlebar::register_hitokage_helpers;
use crate::prepend_css_class_to_model;
use crate::set_initial_base_props;
use crate::structs::reactive::create_react_sender;
use crate::structs::reactive::Reactive;
use crate::structs::reactive::ReactiveString;
use crate::widgets::base::BaseMsgHook;
use gtk4::prelude::*;
use handlebars::Handlebars;
use relm4::loading_widgets::LoadingWidgets;
use relm4::prelude::*;
use relm4::view;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use systemstat::Platform;
use systemstat::System;

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
  Tick,
}

// #[derive(Debug)]
pub enum BatteryMsgOut {
  RequestBatteryLife(relm4::tokio::sync::oneshot::Sender<System>),
  // DropSystem,
}

impl From<BatteryMsgOut> for AppMsg {
  fn from(value: BatteryMsgOut) -> Self {
    match value {
      BatteryMsgOut::RequestBatteryLife(tx) => AppMsg::RequestBatteryLife(tx),
      // BatteryMsgOut::DropSystem => AppMsg::DropSystem,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatteryProps {
  #[serde(flatten)]
  base: BaseProps,
  format: ReactiveString,
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
  sys: System,
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
        // #[track = "model.changed(Weather::react() | Weather::forecast())"]
        // set_label: &format_temperature(&model.forecast, &model.map, &model.format.clone().get()),
        set_label: "foo",
      }
    }
  }

  fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
    view! {
      #[local]
      root {
        #[name(spinner)]
        gtk::Spinner {
          start: (),
          set_halign: gtk::Align::Center,
        }
      }
    }
    Some(LoadingWidgets::new(root, spinner))
  }

  async fn init(props: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
    let (tx, rx) = relm4::tokio::sync::oneshot::channel::<_>();
    sender.output(BatteryMsg::RequestBatteryLife(tx)).unwrap();
    let weather_station = match rx.await {
      Ok(v) => v,
      Err(err) => {
        log::error!("err: {}", err);
        panic!("rip battery life TODO fix")
      }
    };

    let sender_clone = sender.clone();
    let source_id = glib::timeout_add_local(std::time::Duration::from_secs(60), move || {
      sender_clone.input(BatteryMsg::RequestBatteryLife);
      glib::ControlFlow::Continue
    });

    let mut model = Battery {
      base: props.base.clone().into(),
      source_id: Some(source_id),
      format: props
        .format
        .as_reactive_string(create_react_sender(sender.input_sender(), BatteryMsg::React)),
      react: false,
      tracker: 0,
      weather_station: weather_station,
      map: props.weather_options.icons,
    };

    prepend_css_class_to_model!("weather", model, root);
    set_initial_base_props!(model, root, props.base);

    let widgets = view_output!();

    root.show();

    AsyncComponentParts { model, widgets }
  }

  async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>, root: &Self::Root) {
    match msg {
      WeatherMsg::LuaHook(hook) => match hook {
        WeatherMsgHook::BaseHook(base) => {
          generate_base_match_arms!(self, "format", root, base)
        }
        WeatherMsgHook::GetFormat(tx) => {
          tx.send(self.format.clone().get()).unwrap();
        }
        WeatherMsgHook::GetFormatReactive(tx) => {
          tx.send(self.format.clone()).unwrap();
        }
        WeatherMsgHook::SetFormat(format) => {
          let arc = self.format.value.clone();
          let mut str = arc.lock().unwrap();
          *str = format;
          self.set_react(!self.react);
        }
      },
      WeatherMsg::React => {
        self.set_react(!self.react);
      }
      WeatherMsg::RequestForecast => {
        self.set_forecast(request_forecast_from_station(&self.weather_station).await);
      }
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, sender: relm4::Sender<Self::Output>) {
    sender.send(WeatherMsgOut::DropWeatherStation).unwrap();
    self.source_id.take().map(glib::SourceId::remove);
  }
}

#[derive(Debug, Clone)]
struct BatteryWrapper {
  battery: Option<systemstat::BatteryLife>,
}

impl PartialEq for BatteryWrapper {
  fn ne(&self, other: &Self) -> bool {
    self.battery.clone().zip(other.battery.clone()).map_or(false, |(a, b)| {
      a.remaining_capacity != b.remaining_capacity || a.remaining_time != b.remaining_time
    }) || self
      .swap
      .clone()
      .zip(other.swap.clone())
      .map_or(false, |(a, b)| a.free != b.free || a.total != b.total)
  }

  fn eq(&self, other: &Self) -> bool {
    !self.ne(other)
  }
}

impl From<std::io::Result<systemstat::BatteryLife>> for BatteryWrapper {
  fn from(value: std::io::Result<systemstat::BatteryLife>) -> Self {
    match value {
      Ok(value) => BatteryWrapper {
        battery: Some(value.0),
      },
      Err(err) => {
        log::error!("Failed to fetch battery and swap information: {}", err);
        BatteryWrapper {
          battery: None,
        }
      }
    }
  }
}

fn format_battery(format: &String, battery: &systemstat::BatteryLife, swap: &systemstat::Swap) -> String {
  let reg = register_hitokage_helpers(Handlebars::new());

  let mut args = HashMap::new();

  match reg.render_template(format, &args) {
    Ok(name) => return name,
    Err(err) => {
      log::error!("{:?}", err);
    }
  };

  "".to_owned()
}

