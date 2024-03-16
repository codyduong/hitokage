// use serde_json::Value;
// use tokio::io::{self, AsyncReadExt};
// use tokio::process::Command;
// use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, NO_ERROR};
// use windows::Win32::Storage::FileSystem::{
//     CreateNamedPipeA, ReadFile, WriteFile, PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE,
//     PIPE_TYPE_MESSAGE, PIPE_WAIT,
// };
// use windows::Win32::System::Threading::{ConnectNamedPipe, CreateEventA};
// use std::ptr;

// use crate::cxxqt_object::qobject::KomorebiPipe;

// // Constants similar to the Python example
// const KOMOREBI_BUFF_SIZE: u32 = 64 * 1024;
// const KOMOREBI_PIPE_NAME: &str = "\\\\.\\pipe\\yasb";

// async fn create_and_connect_pipe() -> windows::core::Result<HANDLE> {
//     let pipe = unsafe {
//         CreateNamedPipeA(
//             KOMOREBI_PIPE_NAME,
//             PIPE_ACCESS_DUPLEX,
//             PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
//             1,
//             KOMOREBI_BUFF_SIZE,
//             KOMOREBI_BUFF_SIZE,
//             0,
//             ptr::null_mut(),
//         )
//     };

//     if pipe.is_invalid() {
//         return Err(windows::core::Error::from_win32());
//     }

//     // Connect the pipe asynchronously
//     let event = unsafe { CreateEventA(ptr::null_mut(), true, false, None) };
//     if event.is_invalid() {
//         return Err(windows::core::Error::from_win32());
//     }

//     tokio::spawn(async move {
//         if unsafe { ConnectNamedPipe(pipe, ptr::null_mut()) }.as_bool() {
//             println!("Komorebi Connected");
//         } else {
//             let error = unsafe { GetLastError() };
//             if error != NO_ERROR {
//                 println!("Failed to connect Komorebi: {:?}", error);
//             }
//         }
//     });

//     Ok(pipe)
// }

// async fn read_from_pipe(pipe: HANDLE, my_rust_object: KomorebiPipe) {
//   let mut buffer = vec![0u8; KOMOREBI_BUFF_SIZE as usize];
//   loop {
//       let mut bytes_read = 0;
//       let success = unsafe {
//           ReadFile(
//               pipe,
//               buffer.as_mut_ptr() as *mut _,
//               buffer.len() as u32,
//               &mut bytes_read,
//               ptr::null_mut(),
//           )
//       };
//       if success.as_bool() && bytes_read > 0 {
//           let data: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
//           if !data.trim().is_empty() {
//               let event: Value = serde_json::from_str(&data).expect("JSON parsing error");
//               println!("Received event: {:?}", event);

//               // Update the Qt UI via your Rust object's method
//               // Assuming you have a method to update the data that notifies QML
//               my_rust_object.set_data(data);
//           }
//       } else {
//           // Handle errors or disconnection
//           break;
//       }
//   }

//   unsafe { CloseHandle(pipe) };
// }