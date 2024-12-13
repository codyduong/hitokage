use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

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

pub fn normalize_prepend(prepend: impl IntoPrepend) -> Vec<String> {
  prepend.into_prepend()
}

pub trait IntoPrepend {
  fn into_prepend(self) -> Vec<String>;
}

impl IntoPrepend for &str {
  fn into_prepend(self) -> Vec<String> {
    vec![self.to_string()]
  }
}

impl IntoPrepend for Vec<String> {
  fn into_prepend(self) -> Vec<String> {
    self
  }
}

/// Splits a CssClass or Iterable into unique Strings seperated by whitespace while preserving ordering
#[macro_export]
macro_rules! prepend_css_class {
  ($prepend:expr, $class:expr) => {{
    use indexmap::IndexSet;
    use $crate::structs::css_class::normalize_prepend;

    let prepend_vec = normalize_prepend($prepend);

    prepend_vec
      .into_iter()
      .chain($class.into_iter())
      .collect::<IndexSet<String>>()
      .into_iter()
      .collect::<Vec<String>>()
  }};
}

#[macro_export]
macro_rules! prepend_css_class_to_model {
  ($prepend:expr, $model:expr, $root:expr) => {{
    use $crate::prepend_css_class;
    $model.base.classes = prepend_css_class!($prepend, $model.base.classes);
    let classes_ref: Vec<&str> = $model.base.classes.iter().map(AsRef::as_ref).collect();
    $root.set_css_classes(&classes_ref);
  }};
  ($self:expr, $prepend:expr, $classes:expr, $root:expr) => {
    use $crate::prepend_css_class;
    $self.base.classes = prepend_css_class!($prepend, $classes);
    let classes_ref: Vec<&str> = $self.base.classes.iter().map(AsRef::as_ref).collect();
    $root.set_css_classes(&classes_ref);
  };
}
