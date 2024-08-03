pub mod common;
pub mod event;
pub mod flowbox;
pub mod structs;
pub mod widgets;
pub mod win_utils;

pub trait RelmContainerExtManual: 'static {
  fn container_add<T: glib::object::IsA<gtk4::Widget>>(&self, widget: &T);
}
