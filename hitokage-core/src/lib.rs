pub mod common;
pub mod flowbox;
pub mod lua;
pub mod widgets;
pub mod win_utils;

use relm4::{component::Controller, Component};
use std::any::Any;

pub trait RelmContainerExtManual: 'static {
  fn container_add<T: glib::object::IsA<gtk4::Widget>>(&self, widget: &T);
}

trait BarController: Any {
  fn as_any(&self) -> &dyn Any;
}

// impl<T: Component> BarController for Controller<T> {
//   fn as_any(&self) -> &dyn Any {
//     self
//   }
// }
