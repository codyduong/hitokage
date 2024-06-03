// this is so dogshit

use serde_json::{Result as JsonResult, Value as JsonValue};
use std::ptr;
use tokio::{
  process::Command,
  task,
  time::{timeout, Duration},
};
use windows::{
  core::{Error as WinError, PCSTR},
  Win32::{
    Foundation::HANDLE,
    Storage::FileSystem::{ReadFile, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX},
    System::Pipes::*,
  },
};

const KOMOREBI_BUFF_SIZE: u32 = 64 * 1024;
// const KOMOREBI_PIPE_NAME_PLAIN: &str = "hitokage";
const KOMOREBI_PIPE_NAME: &str = "\\\\.\\pipe\\hitokage";

pub async fn create_and_connect_pipe() -> Result<HANDLE, WinError> {
  let pipe_name_cstr = std::ffi::CString::new(KOMOREBI_PIPE_NAME).expect("CString::new failed");
  let pipe_name_pcstr = PCSTR(pipe_name_cstr.as_ptr() as *const _);

  let pipe = unsafe {
    CreateNamedPipeA(
      pipe_name_pcstr,
      PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
      PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
      1,
      1024,
      1024,
      0,
      Some(ptr::null()),
    )
  }
  .expect("Failed to create pipe");

  if pipe.is_invalid() {
    return Err(windows::core::Error::from_win32());
  }

  // Attempt to connect komorebi to our pipe, if not after 30 seconds explode!
  let child = Command::new("komorebic.exe")
    .arg("subscribe-pipe")
    .arg("hitokage")
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .expect("Failed to start komorebic.exe process");

  let timeout_result = timeout(Duration::from_secs(1), child.wait_with_output()).await;
  match timeout_result {
    Ok(Ok(output)) if output.status.success() => {
      let stdout = String::from_utf8_lossy(&output.stdout);
      let stderr = String::from_utf8_lossy(&output.stderr);

      if stdout.contains("Error") || stderr.contains("Error") {
        println!("komorebic reported an error in output: {}", stderr);
      } else {
        println!("komorebic executed successfully.");
        return Ok(pipe); // Success
      }
    }
    Ok(Ok(output)) => {
      println!("{:?}", output);
      println!("komorebic execution failed.");
    }
    Ok(Err(e)) => {
      // The command timed out or failed to execute
      println!("Failed to execute komorebic: {}", e);
    }
    Err(_) => {
      // The command timed out
      println!("komorebic execution timed out.");
    }
  }

  Ok(pipe)
}

#[derive(Debug)]
pub enum ReadPipeError {
  WindowsError(windows::core::Error),
  SerdeJsonError(serde_json::Error),
}

impl From<windows::core::Error> for ReadPipeError {
  fn from(error: windows::core::Error) -> Self {
    ReadPipeError::WindowsError(error)
  }
}

impl From<serde_json::Error> for ReadPipeError {
  fn from(error: serde_json::Error) -> Self {
    ReadPipeError::SerdeJsonError(error)
  }
}

pub async fn read_from_pipe(pipe: HANDLE) -> Result<Option<JsonValue>, ReadPipeError> {
  let buffer_size = KOMOREBI_BUFF_SIZE.try_into().unwrap();

  let read_result = task::spawn_blocking(move || -> windows::core::Result<Option<Vec<u8>>> {
    let mut buffer = vec![0u8; buffer_size];
    let mut bytes_read: u32 = 0;

    let _success = unsafe { ReadFile(pipe, Some(&mut buffer), Some(&mut bytes_read as *mut u32), None) };

    Ok(Some(buffer[..bytes_read as usize].to_vec()))
    // if success.as_bool() {
    //     Ok(Some(buffer[..bytes_read as usize].to_vec()))
    // } else {
    //     let error = unsafe { GetLastError() };
    //     match error {
    //         ERROR_MORE_DATA | ERROR_BROKEN_PIPE => Ok(None),
    //         ERROR_SUCCESS => Ok(None),
    //         _ => Err(windows::core::Error::from_win32()),
    //     }
    // }
  })
  .await;

  match read_result {
    Ok(result) => match result {
      Ok(Some(data)) => {
        let data_as_str = String::from_utf8(data).map_err(|_| windows::core::Error::from_win32())?;
        let json = serde_json::from_str(&data_as_str);

        // Empty pipe
        if data_as_str.trim().is_empty() {
          // return Ok(Some(serde_json::Value::Bool(false)));
          return Ok(None)
        }

        match json {
          Ok(json_value) => return Ok(Some(json_value)),
          Err(e) => Err(e.into()),
        }
      }
      Ok(None) => return Ok(None),
      Err(e) => return Err(e.into()),
    },
    Err(join_error) => {
      eprintln!("Task was cancelled or panicked: {:?}", join_error);
      return Err(windows::core::Error::from_win32().into());
    }
  }
}

pub fn start_async_reader(sender: relm4::ComponentSender<crate::AppState>) {
  // Create an async runtime
  glib::MainContext::default().spawn_local(async move {
    match create_and_connect_pipe().await {
      Ok(pipe) => {
        loop {
          match read_from_pipe(pipe).await {
            Ok(Some(json)) => {
              let line = json.to_string();
              sender.input(crate::Msg::Komorebi(line));
            }
            Ok(None) => {
              // Handle empty read, maybe break the loop if the pipe is closed
              break;
            }
            Err(e) => {
              sender.input(crate::Msg::KomorebiErr(format!("Read error: {:?}", e)));
              break;
            }
          }
        }
      }
      Err(e) => {
        sender.input(crate::Msg::KomorebiErr(format!("Pipe connection error: {:?}", e)));
      }
    }
  });
}
