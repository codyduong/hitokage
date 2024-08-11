use anyhow::Result;
use hitokage_core::event::EventNotif;
use hitokage_lua::AppMsg;
use komorebi_client::send_message;
use komorebi_client::SocketMessage;
use std::io::BufReader;
use std::io::Read;
use std::time::Duration;

const NAME: &str = "hitokage.sock";

pub fn start(sender: relm4::ComponentSender<crate::App>) -> std::thread::JoinHandle<Result<(), anyhow::Error>> {
  let listener = komorebi_client::subscribe(NAME).expect("Failed to open socket");

  std::thread::spawn(move || -> Result<()> {
    for client in listener.incoming() {
      match client {
        Ok(subscription) => {
          let mut buffer = Vec::new();
          let mut reader = BufReader::new(subscription);

          // this is when we know a shutdown has been sent
          if matches!(reader.read_to_end(&mut buffer), Ok(0)) {
            log::debug!("Komorebi shutdown");
            // keep trying to reconnect to komorebi
            while komorebi_client::send_message(&SocketMessage::AddSubscriberSocket(NAME.to_string()))
              .is_err()
            {
              log::debug!("Attempting to reconnect to komorebi");
              std::thread::sleep(Duration::from_secs(1));
            }
          }

          if let Ok(notification) =
            // serde_json::from_str::<komorebi_client::Notification>(&String::from_utf8(buffer).unwrap())
            serde_json::from_str::<EventNotif>(&String::from_utf8(buffer).unwrap())
          {
            sender.input(AppMsg::Komorebi(notification));
          }
        }
        Err(error) => {
          log::error!("Failed to get komorebi event subscription: {error}");
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
