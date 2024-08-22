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
use relm4::prelude::AsyncComponentParts;
use relm4::prelude::AsyncComponentSender;
use relm4::prelude::*;
use relm4::Worker;
use relm4::WorkerController;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

struct WeatherIcons {}

impl Default for WeatherIcons {
  fn default() -> Self {
    Self {}
  }
}

#[derive(Debug, Clone)]
pub enum WeatherMsgHook {
  BaseHook(BaseMsgHook),
  // GetFormat(Sender<String>),
  // GetFormatReactive(Sender<Reactive<String>>),
  // SetFormat(String),
}

#[derive(Debug, Clone)]
pub enum WeatherMsg {
  LuaHook(WeatherMsgHook),
  React,
  RequestForecast,
}

#[derive(Debug)]
pub enum WeatherMsgOut {
  RequestWeatherStation(
    relm4::tokio::sync::oneshot::Sender<WeatherStation>,
    Option<WeatherStationConfig>,
  ),
  DropWeatherStation,
}

impl From<WeatherMsgOut> for AppMsg {
  fn from(value: WeatherMsgOut) -> Self {
    match value {
      WeatherMsgOut::RequestWeatherStation(a, b) => AppMsg::RequestWeatherStation(a, b),
      WeatherMsgOut::DropWeatherStation => AppMsg::DropWeatherStation,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherProps {
  #[serde(flatten)]
  pub base: BaseProps,
  #[serde(flatten)]
  pub config: Option<WeatherStationConfig>,
}

#[derive(Debug)]
#[tracker::track]
pub struct Weather {
  #[tracker::do_not_track]
  base: Base,
  #[tracker::do_not_track]
  source_id: Option<glib::SourceId>,
  react: bool,
  #[tracker::do_not_track]
  weather_station: WeatherStation,
  forecast: WeatherForecast,
}

#[relm4::component(async, pub)]
impl AsyncComponent for Weather {
  type Input = WeatherMsg;
  type Output = WeatherMsgOut;
  type Init = WeatherProps;
  type Widgets = WeatherWidgets;
  type CommandOutput = ();

  view! {
    gtk::Label {
      // #[track = "model.changed(Weather::react() | Weather::forecast())"]
      // set_label: &format_temperature(model.temperature, model.format.clone().get()),
      set_label: "foo"
    }
  }

  async fn init(props: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
    let (tx, rx) = relm4::tokio::sync::oneshot::channel::<_>();
    // get the weather station if there is one already, if not use new config
    let _ = sender.output(WeatherMsgOut::RequestWeatherStation(tx, props.config));
    let weather_station = match rx.await {
      Ok(v) => v,
      Err(err) => {
        log::error!("err: {}", err);
        panic!("rip weather TODO fix")
      }
    };

    let sender_clone = sender.clone();
    let source_id = glib::timeout_add_local(std::time::Duration::from_secs(60), move || {
      sender_clone.input(WeatherMsg::RequestForecast);
      glib::ControlFlow::Continue
    });

    let mut model = Weather {
      base: props.base.clone().into(),
      source_id: Some(source_id),
      // format: props
      //   .format
      //   .as_reactive_string(create_react_sender(sender.input_sender(), WeatherMsg::React)),
      react: false,
      tracker: 0,
      forecast: request_forecast_from_station(&weather_station).await,
      weather_station: weather_station,
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
        } // WeatherMsgHook::GetFormat(tx) => {
          //   tx.send(self.format.clone().get()).unwrap();
          // }
          // WeatherMsgHook::GetFormatReactive(tx) => {
          //   tx.send(self.format.clone()).unwrap();
          // }
          // WeatherMsgHook::SetFormat(format) => {
          //   let arc = self.format.value.clone();
          //   let mut str = arc.lock().unwrap();
          //   *str = format;
          //   self.set_react(!self.react);
          // }
      },
      WeatherMsg::React => {
        self.set_react(!self.react);
      }
      WeatherMsg::RequestForecast => {
        // self.forecast = request_forecast_from_station(&self.weather_station).await;
        self.set_react(!self.react);
      }
    }
  }

  fn shutdown(&mut self, _widgets: &mut Self::Widgets, sender: relm4::Sender<Self::Output>) {
    sender.send(WeatherMsgOut::DropWeatherStation).unwrap();
    self.source_id.take().map(glib::SourceId::remove);
  }
}

async fn request_forecast_from_station(weather_station: &WeatherStation) -> WeatherForecast {
  weather_station.get_forecast().await.unwrap_or_else(|err| {
    log::error!("{}", err.to_string());
    WeatherForecast::default()
  })
}

// https://github.com/glzr-io/zebar/blob/15f34d02cb351ea7d96f6a9c6c286d5eb23cdabf/packages/desktop/src/providers/weather/open_meteo_res.rs
#[derive(Deserialize, Debug, Clone)]
struct OpenMeteoRes {
  pub current_weather: OpenMeteoWeather,
}

#[derive(Deserialize, Debug, Clone)]
struct OpenMeteoWeather {
  pub temperature: f32,
  #[serde(rename = "windspeed")]
  pub wind_speed: f32,
  #[serde(rename = "winddirection")]
  pub wind_direction: f32,
  #[serde(rename = "weathercode")]
  pub weather_code: u32,
  pub is_day: u32,
}

#[derive(Debug, Clone)]
pub struct WeatherForecast {
  temperature: f32,
}

impl Default for WeatherForecast {
  fn default() -> Self {
    Self { temperature: 0.0 }
  }
}

impl PartialEq for WeatherForecast {
  fn ne(&self, other: &Self) -> bool {
    self.temperature != other.temperature
  }

  fn eq(&self, other: &Self) -> bool {
    !self.ne(other)
  }
}

impl From<OpenMeteoWeather> for WeatherForecast {
  fn from(value: OpenMeteoWeather) -> Self {
    Self {
      temperature: value.temperature,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherStationConfig {
  latitude: f32,
  longitude: f32,
}

#[derive(Debug, Clone)]
pub struct WeatherStation {
  client: Client,
  latitude: f32,
  longitude: f32,
  last_forecast: Arc<Mutex<Option<OpenMeteoWeather>>>,
  last_time: Arc<Mutex<Option<Instant>>>,
}

impl WeatherStation {
  pub async fn get_forecast(&self) -> anyhow::Result<WeatherForecast> {
    let now = Instant::now();

    if let Some(last_time) = self.last_time.lock().unwrap().as_ref() {
      if now.duration_since(*last_time) < Duration::from_secs(60) {
        let last_forecast = self.last_forecast.lock().unwrap().clone();
        return last_forecast
          .map(|v| v.into())
          .ok_or_else(|| anyhow::anyhow!("Empty last_forecast"));
      }
    }

    // Relevant documentation: https://open-meteo.com/en/docs#weathervariables
    let res = self
      .client
      .get("https://api.open-meteo.com/v1/forecast")
      .query(&[
        ("temperature_unit", "celsius"),
        ("latitude", &self.latitude.to_string()),
        ("longitude", &self.longitude.to_string()),
        ("current_weather", "true"),
        ("daily", "sunset,sunrise"),
        ("timezone", "auto"),
      ])
      .send()
      .await?
      .json::<OpenMeteoRes>()
      .await?;

    let current_weather = res.current_weather;
    let is_daytime = current_weather.is_day == 1;

    log::info!("received forecast: {:?}", current_weather);

    Ok(current_weather.into())
  }

  pub fn new(config: WeatherStationConfig) -> Self {
    WeatherStation {
      client: Client::new(),
      latitude: config.latitude,
      longitude: config.longitude,
      last_forecast: Arc::new(Mutex::new(None)),
      last_time: Arc::new(Mutex::new(None)),
    }
  }
}

fn format_temperature(temp: f32, format: String) -> String {
  let reg = register_hitokage_helpers(Handlebars::new());

  let mut args = HashMap::new();

  let temp_fahrenheit = (temp * 9.) / 5. + 32.;

  args.insert("temp_celsius".to_string(), temp);
  args.insert("temp_fahrenheit".to_string(), temp_fahrenheit);

  match reg.render_template(&format, &args) {
    Ok(name) => return name,
    Err(err) => {
      log::error!("{:?}", err);
    }
  };

  String::new()
}
