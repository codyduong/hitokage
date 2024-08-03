use crate::structs::CssClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseProps {
  pub class: Option<CssClass>,
}

impl From<BaseProps> for Base {
  fn from(item: BaseProps) -> Self {
    Base {
      classes: item.class.unwrap_or_default().into(),
    }
  }
}

pub struct Base {
  pub classes: Vec<String>,
}
