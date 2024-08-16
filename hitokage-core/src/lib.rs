pub mod common;
pub mod deserializer;
pub mod event;
pub mod flowbox;
pub mod handlebar;
pub mod structs;
pub mod widgets;
pub mod win_utils;

pub trait RelmContainerExtManual: 'static {
  fn container_add<T: glib::object::IsA<gtk4::Widget>>(&self, widget: &T);
}

pub fn get_hitokage_asset(s: impl Into<String>) -> std::path::PathBuf {
  let mut base = if cfg!(feature = "development") {
    let mut path = std::env::current_dir().unwrap();
    path.push(match std::env::var("HITOKAGE_DEV_USE_EXAMPLE") {
      Ok(v) => match v.as_str() {
        "minimal" => "./examples/minimal/",
        "testbench" => "./examples/",
        _ => {
          log::error!("Unsupported example: {}", v);
          "./examples/testbench/"
        }
      },
      Err(err) => {
        match err {
          std::env::VarError::NotPresent => (),
          std::env::VarError::NotUnicode(_) => {
            log::error!("Failed to read HITOKAGE_DEV_USE_EXAMPLE: {:?}", err);
          }
        };
        "./examples/testbench/"
      }
    });
    path
  } else {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".config/hitokage/");
    path
  };

  base.push(s.into());
  base
}
