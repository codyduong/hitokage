use gtk4::prelude::*;
use gtk4::FlowBox;

// https://github.com/Relm4/Relm4/issues/587
pub trait RelmContainerExtManual: 'static {
  fn container_add<T: IsA<gtk4::Widget>>(&self, widget: &T);
}

impl RelmContainerExtManual for FlowBox {
  fn container_add<T: IsA<gtk4::Widget>>(&self, widget: &T) {
      self.append(widget);
  }
}