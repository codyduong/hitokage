use crate::RelmContainerExtManual;
use gtk4::prelude::*;
use gtk4::FlowBox;

// https://github.com/Relm4/Relm4/issues/587
impl RelmContainerExtManual for FlowBox {
  fn container_add<T: IsA<gtk4::Widget>>(&self, widget: &T) {
    self.append(widget);
  }
}
