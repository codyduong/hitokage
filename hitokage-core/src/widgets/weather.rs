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
use relm4::prelude::AsyncComponentParts;
use relm4::prelude::AsyncComponentSender;
use relm4::prelude::*;
use relm4::tokio::sync::Semaphore;
use relm4::view;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WeatherOptions {
  #[serde(default)]
  icons: WeatherIcons,
  #[serde(default)]
  windy_threshold: u32, // in kmph, default 15
}

impl Default for WeatherOptions {
  fn default() -> Self {
    Self {
      icons: WeatherIcons::default(),
      windy_threshold: 15,
    }
  }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WeatherIcons {
  #[serde(default)]
  day: String,
  // #[serde(default)]
  // day_windy: String,
  #[serde(default)]
  day_cloudy: String,
  // #[serde(default)]
  // day_cloudy_windy: String,
  #[serde(default)]
  day_overcast: String,
  // #[serde(default)]
  // day_overcast_windy: String,
  #[serde(default)]
  day_foggy: String,
  // #[serde(default)]
  // day_foggy_windy: String,
  #[serde(default)]
  day_drizzle: String,
  // #[serde(default)]
  // day_drizzle_windy: String,
  #[serde(default)]
  day_rain: String,
  // #[serde(default)]
  // day_rain_windy: String,
  #[serde(default)]
  day_showers: String,
  // #[serde(default)]
  // day_showers_windy: String,
  #[serde(default)]
  day_freezing_rain: String,
  // #[serde(default)]
  // day_freezing_rain_windy: String,
  #[serde(default)]
  day_snow: String,
  // #[serde(default)]
  // day_snow_windy: String,
  #[serde(default)]
  day_thunderstorm: String,
  // #[serde(default)]
  // day_thunderstorm_windy: String,
  #[serde(default)]
  day_hail: String,
  // #[serde(default)]
  // day_hail_windy: String,
  #[serde(default)]
  night: String,
  // #[serde(default)]
  // night_windy: String,
  #[serde(default)]
  night_cloudy: String,
  // #[serde(default)]
  // night_cloudy_windy: String,
  #[serde(default)]
  night_overcast: String,
  // #[serde(default)]
  // night_overcast_windy: String,
  #[serde(default)]
  night_foggy: String,
  // #[serde(default)]
  // night_foggy_windy: String,
  #[serde(default)]
  night_drizzle: String,
  // #[serde(default)]
  // night_drizzle_windy: String,
  #[serde(default)]
  night_rain: String,
  // #[serde(default)]
  // night_rain_windy: String,
  #[serde(default)]
  night_showers: String,
  // #[serde(default)]
  // night_showers_windy: String,
  #[serde(default)]
  night_freezing_rain: String,
  // #[serde(default)]
  // night_freezing_rain_windy: String,
  #[serde(default)]
  night_snow: String,
  // #[serde(default)]
  // night_snow_windy: String,
  #[serde(default)]
  night_thunderstorm: String,
  // #[serde(default)]
  // night_thunderstorm_windy: String,
  #[serde(default)]
  night_hail: String,
  // #[serde(default)]
  // night_hail_windy: String,
  #[serde(default)]
  unknown: String,
}

impl Default for WeatherIcons {
  fn default() -> Self {
    Self {
      day: "\u{E30D}".to_owned(), // nf-weather-day_sunny
      // day_windy: "\u{E37D}".to_owned(), // nf-weather-day_windy
      day_cloudy: "\u{E302}".to_owned(), // nf-weather-day_cloudy
      // day_cloudy_windy: "\u{E301}".to_owned(), // nf-weather-day_cloudy_windy
      day_overcast: "\u{E30C}".to_owned(), // nf-weather-day_sunny_overcast
      // day_overcast_windy: "\u{E30C}".to_owned(), // nf-weather-day_sunny_overcast
      day_foggy: "\u{E303}".to_owned(), // nf-weather-day_fog
      // day_foggy_windy: "\u{E303}".to_owned(), // nf-weather-day_fog
      day_drizzle: "\u{E30B}".to_owned(), // nf-weather-day_sprinkle
      // day_drizzle_windy: "\u{E30B}".to_owned(), // nf-weather-day_sprinkle
      day_rain: "\u{E305}".to_owned(), // nf-weather-day_rain
      // day_rain_windy: "\u{E307}".to_owned(), // nf-weather-day_rain_wind
      day_showers: "\u{E309}".to_owned(), // nf-weather-day_showers
      // day_showers_windy: "\u{E309}".to_owned(), // nf-weather-day_showers
      day_freezing_rain: "\u{E306}".to_owned(), // nf-weather-day_rain_mix
      // day_freezing_rain_windy: "\u{E306}".to_owned(), // nf-weather-day_rain_mix
      day_snow: "\u{E30A}".to_owned(), // nf-weather-day_snow
      // day_snow_windy: "\u{E35F}".to_owned(), // nf-weather-day_snow_wind
      day_thunderstorm: "\u{E30F}".to_owned(), // nf-weather-day_thunderstorm
      // day_thunderstorm_windy: "\u{E30F}".to_owned(), // nf-weather-day_thunderstorm
      day_hail: "\u{E365}".to_owned(), // nf-weather-day_snow_thunderstorm
      // day_hail_windy: "\u{E365}".to_owned(), // nf-weather-day_snow_thunderstorm
      night: "\u{E32B}".to_owned(), // nf-weather-weather_night_clear
      // night_windy: "\u{E32B}".to_owned(), // nf-weather-weather_night_clear // hmm, there is no night windy
      night_cloudy: "\u{E37E}".to_owned(), // nf-weather-night_alt_cloudy
      // night_cloudy_windy: "\u{E320}".to_owned(), // nf-weather-night_alt_cloudy_windy
      night_overcast: "\u{E312}".to_owned(), // nf-weather-cloudy
      // night_overcast_windy: "\u{E311}".to_owned(), // nf-weather-cloudy_windy
      night_foggy: "\u{E346}".to_owned(), // nf-weather-night_fog
      // night_foggy_windy: "\u{E346}".to_owned(), // nf-weather-night_fog
      night_drizzle: "\u{E328}".to_owned(), // nf-weather-night_alt_sprinkle
      // night_drizzle_windy: "\u{E328}".to_owned(), // nf-weather-night_alt_sprinkle
      night_rain: "\u{E325}".to_owned(), // nf-weather-night_alt_rain
      // night_rain_windy: "\u{E324}".to_owned(), // nf-weather-night_alt_rain_wind
      night_showers: "\u{E326}".to_owned(), // nf-weather-night_alt_showers
      // night_showers_windy: "\u{E326}".to_owned(), // nf-weather-night_alt_showers
      night_freezing_rain: "\u{E323}".to_owned(), // nf-weather-night_alt_rain_mix
      // night_freezing_rain_windy: "\u{E323}".to_owned(), // nf-weather-night_alt_rain_mix
      night_snow: "\u{E327}".to_owned(), // nf-weather-night_alt_snow
      // night_snow_windy: "\u{E361}".to_owned(), // nf-weather-night_alt_snow_wind
      night_thunderstorm: "\u{E32A}".to_owned(), // nf-weather-night_alt_thunderstorm
      // night_thunderstorm_windy: "\u{E32A}".to_owned(), // nf-weather-night_alt_thunderstorm
      night_hail: "\u{E367}".to_owned(), // nf-weather-night_alt_snow_thunderstorm
      // night_hail_windy: "\u{E367}".to_owned(), // nf-weather-night_alt_snow_thunderstorm
      unknown: "\u{F128}".to_owned(), // nf-fa-question
    }
  }
}

#[derive(Debug, Clone)]
pub enum WeatherMsgHook {
  BaseHook(BaseMsgHook),
  GetFormat(Sender<String>),
  GetFormatReactive(Sender<Reactive<String>>),
  SetFormat(String),
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
  #[serde(flatten, default)]
  pub weather_options: WeatherOptions,
  format: ReactiveString,
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
  format: Reactive<String>,
  #[tracker::do_not_track]
  map: WeatherIcons
}

#[relm4::component(async, pub)]
impl AsyncComponent for Weather {
  type Input = WeatherMsg;
  type Output = WeatherMsgOut;
  type Init = WeatherProps;
  type Widgets = WeatherWidgets;
  type CommandOutput = ();

  view! {
    gtk::Box {
      #[name="weather"]
      gtk::Label {
        #[track = "model.changed(Weather::react() | Weather::forecast())"]
        set_label: &format_temperature(&model.forecast, &model.map, &model.format.clone().get()),
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
      format: props
        .format
        .as_reactive_string(create_react_sender(sender.input_sender(), WeatherMsg::React)),
      react: false,
      tracker: 0,
      forecast: request_forecast_from_station(&weather_station).await,
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

async fn request_forecast_from_station(weather_station: &WeatherStation) -> WeatherForecast {
  weather_station.get_forecast().await.unwrap_or_else(|err| {
    log::error!("{}", err.to_string());
    WeatherForecast::default()
  })
}

// https://github.com/glzr-io/zebar/blob/15f34d02cb351ea7d96f6a9c6c286d5eb23cdabf/packages/desktop/src/providers/weather/open_meteo_res.rs
#[derive(Deserialize, Debug, Clone, Copy)]
struct OpenMeteoRes {
  pub current_weather: OpenMeteoWeather,
}

#[derive(Deserialize, Debug, Clone, Copy)]
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
  wind_speed: f32,
  wind_direction: f32,
  weather_code: u32,
  is_day: bool,
}

impl WeatherForecast {
  pub fn temperature_fahrenheit(&self) -> f32 {
    let fahrenheit = (self.temperature * 9.) / 5. + 32.;
    (fahrenheit * 10.0).round() / 10.0
  }

  pub fn weather_code_to_icon(&self, map: &WeatherIcons) -> String {
    match (self.is_day, self.weather_code) {
      (true, 0) => map.day.clone(),
      (true, 1..=3) => map.day_cloudy.clone(),
      (true, 45..=48) => map.day_foggy.clone(),
      (true, 51..=55) => map.day_drizzle.clone(),
      (true, 56..=57) => map.day_freezing_rain.clone(),
      (true, 61..=65) => map.day_rain.clone(),
      (true, 66..=67) => map.day_freezing_rain.clone(),
      (true, 71..=77) => map.day_snow.clone(),
      (true, 80..=82) => map.day_showers.clone(),
      (true, 85..=86) => map.day_snow.clone(),
      (true, 95) => map.day_thunderstorm.clone(),
      (true, 96..=99) => map.day_hail.clone(),
      (false, 0) => map.night.clone(),
      (false, 1..=3) => map.night_cloudy.clone(),
      (false, 45..=48) => map.night_foggy.clone(),
      (false, 51..=55) => map.night_drizzle.clone(),
      (false, 56..=57) => map.night_freezing_rain.clone(),
      (false, 61..=65) => map.night_rain.clone(),
      (false, 66..=67) => map.night_freezing_rain.clone(),
      (false, 71..=77) => map.night_snow.clone(),
      (false, 80..=82) => map.night_showers.clone(),
      (false, 85..=86) => map.night_snow.clone(),
      (false, 95) => map.night_thunderstorm.clone(),
      (false, 96..=99) => map.night_hail.clone(),
      (_, _) => map.unknown.clone(),
    }
  }
}

impl Default for WeatherForecast {
  fn default() -> Self {
    Self {
      temperature: 0.0,
      wind_speed: 0.0,
      wind_direction: 0.0,
      weather_code: 0,
      is_day: true,
    }
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

impl From<OpenMeteoRes> for WeatherForecast {
  fn from(value: OpenMeteoRes) -> Self {
    Self {
      temperature: value.current_weather.temperature,
      wind_speed: value.current_weather.wind_speed,
      wind_direction: value.current_weather.wind_direction,
      weather_code: value.current_weather.weather_code,
      is_day: value.current_weather.is_day == 1,
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
  last_forecast: Arc<Mutex<Option<(Instant, WeatherForecast)>>>,
  semaphore: Arc<Semaphore>,
}

impl WeatherStation {
  pub async fn get_forecast(&self) -> anyhow::Result<WeatherForecast> {
    let _permit = self.semaphore.acquire().await.unwrap();

    let now = Instant::now();

    if let Some(last_forecast) = self.last_forecast.lock().unwrap().as_ref() {
      if now.duration_since(last_forecast.0) < Duration::from_secs(55) {
        log::debug!("Using cached weather forecast");
        return Ok(last_forecast.1.clone().into());
      }
    }

    log::debug!("Getting weather forecast");

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

    let mut last_forecast_guard = self.last_forecast.lock().unwrap();
    *last_forecast_guard = Some((Instant::now(), res.into()));

    log::debug!("Received forecast: {:?}", res);

    Ok(res.into())
  }

  pub fn new(config: WeatherStationConfig) -> Self {
    WeatherStation {
      client: Client::new(),
      latitude: config.latitude,
      longitude: config.longitude,
      last_forecast: Arc::new(Mutex::new(None)),
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }
}

fn format_temperature(forecast: &WeatherForecast, map: &WeatherIcons, format: &String) -> String {
  let reg = register_hitokage_helpers(Handlebars::new());

  let mut args = HashMap::new();

  args.insert("temp_celsius".to_string(), forecast.temperature.to_string());
  args.insert(
    "temp_fahrenheit".to_string(),
    forecast.temperature_fahrenheit().to_string(),
  );
  args.insert("icon".to_string(), forecast.weather_code_to_icon(map));

  match reg.render_template(format, &args) {
    Ok(name) => return name,
    Err(err) => {
      log::error!("{:?}", err);
    }
  };

  String::new()
}
