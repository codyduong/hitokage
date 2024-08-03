use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

pub const HITOKAGE_STATUSBAR_HEIGHT: i32 = 24;

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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CssClass {
  Str(String),
  Vec(Vec<String>),
}

impl Default for CssClass {
  fn default() -> Self {
    CssClass::Vec(Vec::new())
  }
}

impl From<CssClass> for Vec<String> {
  fn from(css_class: CssClass) -> Vec<String> {
    match css_class {
      CssClass::Str(s) => s.split_whitespace().map(String::from).collect::<IndexSet<String>>(),
      CssClass::Vec(v) => v
        .into_iter()
        .flat_map(|s| s.split_whitespace().map(String::from).collect::<Vec<_>>())
        .collect::<IndexSet<String>>(),
    }
    .into_iter()
    .collect()
  }
}

impl IntoIterator for CssClass {
  type Item = String;
  type IntoIter = std::vec::IntoIter<String>;

  fn into_iter(self) -> Self::IntoIter {
    Into::<Vec<String>>::into(self).into_iter()
  }
}

#[macro_export]
macro_rules! prepend_css_class {
  ($prepend:expr, $class:expr) => {
    std::iter::once($prepend.to_string())
      .chain($class.into_iter())
      .collect::<IndexSet<String>>()
      .into_iter()
      .collect::<Vec<String>>()
  };
}
