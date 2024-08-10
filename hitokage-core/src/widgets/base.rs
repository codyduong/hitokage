use crate::structs::{Align, CssClass};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum BaseMsgHook {
  GetClass(Sender<Vec<String>>),
  SetClass(Vec<String>),
  GetHalign(Sender<Align>),
  SetHalign(Align),
  GetHexpand(Sender<bool>),
  SetHexpand(Option<bool>),
  GetValign(Sender<Align>),
  SetValign(Align),
  GetVexpand(Sender<bool>),
  SetVexpand(Option<bool>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseProps {
  pub class: Option<CssClass>,
  pub halign: Option<Align>,
  pub hexpand: Option<bool>,
  pub valign: Option<Align>,
  pub vexpand: Option<bool>,
}

impl From<BaseProps> for Base {
  fn from(props: BaseProps) -> Self {
    Base {
      classes: props.class.unwrap_or_default().into(),
      halign: props.halign,
      hexpand: props.hexpand,
      valign: props.valign,
      vexpand: props.vexpand,
    }
  }
}

pub struct Base {
  pub classes: Vec<String>,
  pub halign: Option<Align>,
  pub hexpand: Option<bool>,
  pub valign: Option<Align>,
  pub vexpand: Option<bool>,
}

#[macro_export]
macro_rules! generate_base_match_arms {
  ($self:expr, $box_str:expr, $root:expr, $hook:expr) => {
    match $hook {
      BaseMsgHook::GetClass(tx) => {
        tx.send($self.base.classes.clone()).unwrap();
      }
      BaseMsgHook::SetClass(classes) => {
        use crate::structs::CssClass;
        prepend_css_class_to_model!($self, $box_str, CssClass::Vec(classes), $root);
      }
      BaseMsgHook::GetHalign(tx) => {
        if let Some(halign) = $self.base.halign {
          tx.send(halign).unwrap();
        } else {
          let halign: $crate::structs::Align = $root.halign().into();
          tx.send(halign).unwrap();
        }
      }
      BaseMsgHook::SetHalign(halign) => {
        $self.base.halign = Some(halign);
        $root.set_halign(halign.into());
      }
      BaseMsgHook::GetHexpand(tx) => {
        if let Some(hexpand) = $self.base.hexpand {
          tx.send(hexpand).unwrap();
        } else {
          let hexpand: bool = $root.hexpands().into();
          tx.send(hexpand).unwrap();
        }
      }
      BaseMsgHook::SetHexpand(hexpand) => {
        $self.base.hexpand = hexpand;
        if let Some(hexpand) = $self.base.hexpand {
          $root.set_hexpand_set(true);
          $root.set_hexpand(hexpand);
        } else {
          $root.set_hexpand_set(false);
        }
      }
      BaseMsgHook::GetValign(tx) => {
        if let Some(valign) = $self.base.valign {
          tx.send(valign).unwrap();
        } else {
          let valign: $crate::structs::Align = $root.valign().into();
          tx.send(valign).unwrap();
        }
      }
      BaseMsgHook::SetValign(valign) => {
        $self.base.valign = Some(valign);
        $root.set_valign(valign.into());
      }
      BaseMsgHook::GetVexpand(tx) => {
        if let Some(vexpand) = $self.base.vexpand {
          tx.send(vexpand).unwrap();
        } else {
          let vexpand: bool = $root.vexpands().into();
          tx.send(vexpand).unwrap();
        }
      }
      BaseMsgHook::SetVexpand(vexpand) => {
        $self.base.vexpand = vexpand;
        if let Some(vexpand) = $self.base.vexpand {
          $root.set_vexpand_set(true);
          $root.set_vexpand(vexpand);
        } else {
          $root.set_vexpand_set(false);
        }
      }
    }
  };
}

#[macro_export]
macro_rules! set_initial_base_props {
  ($self: expr,$root:expr) => {
    if let Some(halign) = $self.base.halign {
      $root.set_halign(halign.into());
    }
    if let Some(hexpand) = $self.base.hexpand {
      $root.set_hexpand_set(true);
      $root.set_hexpand(hexpand);
    } else {
      $root.set_hexpand_set(false);
    }
    if let Some(valign) = $self.base.valign {
      $root.set_valign(valign.into());
    }
    if let Some(vexpand) = $self.base.vexpand {
      $root.set_vexpand_set(true);
      $root.set_vexpand(vexpand);
    } else {
      $root.set_vexpand_set(false);
    }
  };
}
