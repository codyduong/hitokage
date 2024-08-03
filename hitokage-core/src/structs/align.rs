use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Align {
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
