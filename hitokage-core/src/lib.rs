pub mod common;
pub mod flowbox;
pub mod lua;
pub mod widgets;
pub mod win_utils;
use std::any::Any;

pub trait RelmContainerExtManual: 'static {
  fn container_add<T: glib::object::IsA<gtk4::Widget>>(&self, widget: &T);
}
