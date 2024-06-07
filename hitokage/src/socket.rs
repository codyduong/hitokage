use anyhow::Result;
use hitokage_lua::AppMsg;
use std::io::BufRead;
use std::sync::Arc;
use std::sync::Mutex;

const NAME: &str = "hitokage.sock";

pub fn start_async_reader_new(sender: relm4::ComponentSender<crate::App>) {
  let socket = Arc::new(Mutex::new(
    komorebi_client::subscribe(NAME).expect("Failed to open socket"),
  ));

  {
    std::thread::spawn(move || -> Result<()> {
      for incoming in socket.lock().expect("Failed to lock socket").incoming() {
        match incoming {
          Ok(data) => {
            let reader = std::io::BufReader::new(data.try_clone().expect(""));

            for line in reader.lines().flatten() {
              // let notification: komorebi_client::Notification = match serde_json::from_str(&line) {
              let notification: serde_json::Value = match serde_json::from_str(&line) {
                Ok(notification) => notification,
                Err(error) => {
                  log::debug!("discarding malformed komorebi notification: {error}");
                  continue;
                }
              };

              // match and filter on desired notifications
              match notification {
                _ => {
                  // TODO @codyduong LOL why did we convert to json just to convert to string then back to json in main.rs LOL!!
                  let line = notification.to_string();
                  sender.input(AppMsg::Komorebi(line));
                }
              }
            }
          }
          Err(error) => {
            log::debug!("{error}");
          }
        }
      }

      Ok(())
    });
  };
}
