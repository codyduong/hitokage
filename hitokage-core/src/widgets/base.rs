use crate::structs::{Align, CssClass};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum BaseMsgHook {
  GetClass(Sender<Vec<String>>),
  SetClass(Option<CssClass>),
  GetHalign(Sender<Align>),
  SetHalign(Align),
  GetHexpand(Sender<bool>),
  SetHexpand(Option<bool>),
  GetHomogeneous(Sender<bool>),
  SetHomogeneous(bool),
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
  pub homogeneous: Option<bool>,
  pub valign: Option<Align>,
  pub vexpand: Option<bool>,
}

impl From<BaseProps> for Base {
  fn from(props: BaseProps) -> Self {
    Base {
      classes: props.class.unwrap_or_default().into(),
      halign: props.halign,
      hexpand: props.hexpand,
      homogeneous: props.homogeneous,
      valign: props.valign,
      vexpand: props.vexpand,
    }
  }
}

pub struct Base {
  pub classes: Vec<String>,
  pub halign: Option<Align>,
  pub hexpand: Option<bool>,
  pub homogeneous: Option<bool>,
  pub valign: Option<Align>,
  pub vexpand: Option<bool>,
}

#[macro_export]
macro_rules! generate_base_match_arms {
  ($self:ident, $box_str:expr, $root:expr, $msg_type:ident, $hook:expr) => {
    match $hook {
      $msg_type::GetClass(tx) => {
        tx.send($self.base.classes.clone()).unwrap();
      }
      $msg_type::SetClass(classes) => {
        prepend_css_class_to_model!($self, $box_str, classes, $root);
      }
      $msg_type::GetHalign(tx) => {
        if let Some(halign) = $self.base.halign {
          tx.send(halign).unwrap();
        } else {
          let halign: $crate::structs::Align = $root.halign().into();
          tx.send(halign).unwrap();
        }
      }
      $msg_type::SetHalign(halign) => {
        $self.base.halign = Some(halign);
        $root.set_halign(halign.into());
      }
      $msg_type::GetHexpand(tx) => {
        if let Some(hexpand) = $self.base.hexpand {
          tx.send(hexpand).unwrap();
        } else {
          let hexpand: bool = $root.hexpands().into();
          tx.send(hexpand).unwrap();
        }
      }
      $msg_type::SetHexpand(hexpand) => {
        $self.base.hexpand = hexpand;
        if let Some(hexpand) = $self.base.hexpand {
          $root.set_hexpand_set(true);
          $root.set_hexpand(hexpand);
        } else {
          $root.set_hexpand_set(false);
        }
      }
      $msg_type::GetHomogeneous(tx) => {
        if let Some(homogeneous) = $self.base.homogeneous {
          tx.send(homogeneous).unwrap();
        } else {
          let homogeneous: bool = $root.is_homogeneous();
          tx.send(homogeneous).unwrap();
        }
      }
      $msg_type::SetHomogeneous(homogeneous) => {
        $self.base.homogeneous = Some(homogeneous);
        $root.set_homogeneous(homogeneous);
      }
      $msg_type::GetValign(tx) => {
        if let Some(valign) = $self.base.valign {
          tx.send(valign).unwrap();
        } else {
          let valign: $crate::structs::Align = $root.valign().into();
          tx.send(valign).unwrap();
        }
      }
      $msg_type::SetValign(valign) => {
        $self.base.valign = Some(valign);
        $root.set_valign(valign.into());
      }
      $msg_type::GetVexpand(tx) => {
        if let Some(vexpand) = $self.base.vexpand {
          tx.send(vexpand).unwrap();
        } else {
          let vexpand: bool = $root.vexpands().into();
          tx.send(vexpand).unwrap();
        }
      }
      $msg_type::SetVexpand(vexpand) => {
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
  ($self: ident,$root:expr) => {
    if let Some(halign) = $self.base.halign {
      $root.set_halign(halign.into());
    }
    if let Some(hexpand) = $self.base.hexpand {
      $root.set_hexpand_set(true);
      $root.set_hexpand(hexpand);
    } else {
      $root.set_hexpand_set(false);
    }
    if let Some(homogeneous) = $self.base.homogeneous {
      $root.set_homogeneous(homogeneous);
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
