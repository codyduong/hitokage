use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Default)]
pub enum Align {
  #[default]
  Fill,
  Start,
  End,
  Center,
  Baseline,
  // BaselineFill,
  // BaselineCenter,
  __Unknown(i32),
}

impl From<Align> for gtk4::Align {
  fn from(item: Align) -> Self {
    match item {
      Align::Fill => gtk4::Align::Fill,
      Align::Start => gtk4::Align::Start,
      Align::End => gtk4::Align::End,
      Align::Center => gtk4::Align::Center,
      Align::Baseline => gtk4::Align::Baseline,
      Align::__Unknown(i) => gtk4::Align::__Unknown(i),
    }
  }
}

impl From<gtk4::Align> for Align {
  fn from(item: gtk4::Align) -> Self {
    match item {
      gtk4::Align::Fill => Align::Fill,
      gtk4::Align::Start => Align::Start,
      gtk4::Align::End => Align::End,
      gtk4::Align::Center => Align::Center,
      gtk4::Align::Baseline => Align::Baseline,
      gtk4::Align::__Unknown(i) => Align::__Unknown(i),
      _ => Align::__Unknown(0),
    }
  }
}
