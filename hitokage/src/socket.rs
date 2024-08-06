use anyhow::Result;
use hitokage_core::event::EventNotif;
use hitokage_lua::AppMsg;
use komorebi_client::send_message;
use std::io::BufRead;
use std::sync::Arc;
use std::sync::Mutex;

const NAME: &str = "hitokage.sock";

pub fn start(sender: relm4::ComponentSender<crate::App>) -> std::thread::JoinHandle<Result<(), anyhow::Error>> {
  let socket = Arc::new(Mutex::new(
    komorebi_client::subscribe(NAME).expect("Failed to open socket"),
  ));

  std::thread::spawn(move || -> Result<()> {
    for incoming in socket.lock().expect("Failed to lock socket").incoming() {
      match incoming {
        Ok(data) => {
          let reader = std::io::BufReader::new(data.try_clone().expect(""));

          for line in reader.lines().flatten() {
            // let notification: komorebi_client::Notification = match serde_json::from_str(&line) {
            let notification: Option<EventNotif> = match serde_json::from_str(&line) {
              Ok(notification) => notification,
              Err(error) => {
                log::warn!("Failed to read notification from komorebic: {:?}", error);
                None
              }
            };

            // match and filter on desired notifications
            match notification {
              Some(notif) => {
                sender.input(AppMsg::Komorebi(notif));
              },
              _ => ()
            }
          }
        }
        Err(error) => {
          log::debug!("{error}");
        }
      }
    }

    Ok(())
  })
}

pub fn shutdown() -> Result<()> {
  send_message(&komorebi_client::SocketMessage::RemoveSubscriberSocket(
    NAME.to_string(),
  ))?;
  Ok(())
}
