use std::{os::windows::io::FromRawHandle, ptr};
use tokio::{
    io::AsyncReadExt, process::Command, time::{sleep, timeout, Duration}
};
use serde_json::{Result as JsonResult, Value as JsonValue};
use windows::{
    core::{Error as WinError, HRESULT, PCSTR},
    Win32::{
        Foundation::{GetLastError, ERROR_MORE_DATA, ERROR_PIPE_CONNECTED, HANDLE},
        Storage::FileSystem::{ReadFile, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX},
        System::{Pipes::*, IO::OVERLAPPED},
    },
};

const KOMOREBI_BUFF_SIZE: u32 = 64 * 1024;
const KOMOREBI_PIPE_NAME_PLAIN: &str = "hitokage";
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
            ptr::null(),
        )
    };

    if pipe.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    // Attempt to connect komorebi to our pipe, if not after 30 seconds explode!

    let max_retries = 3; // Number of retries
    let retry_delay = Duration::from_secs(1); // Delay between retries

    for attempt in 1..=max_retries {
        println!("Attempt {}/{} to start komorebic.exe", attempt, max_retries);

        let mut child = Command::new("komorebic.exe")
            .arg("subscribe-pipe")
            .arg("hitokage")
            .stdout(std::process::Stdio::piped()) // Capture stdout
            .stderr(std::process::Stdio::piped()) // Capture stderr
            .spawn()
            .expect("Failed to start komorebic.exe process");

        let timeout_result = timeout(Duration::from_secs(30), child.wait_with_output()).await;

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
            },
            Ok(Ok(output)) => {
                println!("{:?}", output);
                println!("komorebic execution failed.");
            },
            Ok(Err(e)) => {
                // The command timed out or failed to execute
                println!("Failed to execute komorebic: {}", e);
            },
            Err(_) => {
                // The command timed out
                println!("komorebic execution timed out.");
            },
        }

        println!("Retrying in {:?}...", retry_delay);
        sleep(retry_delay).await;
    }

    Ok(pipe)
}

pub async fn read_from_pipe(pipe: HANDLE) -> windows::core::Result<JsonValue> {
    let mut buffer = vec![0u8; KOMOREBI_BUFF_SIZE.try_into().unwrap()];
    let mut total_data = Vec::new();

    loop {
        let mut bytes_read: u32 = 0;
        let success = unsafe {
            ReadFile(
                pipe,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                &mut bytes_read,
                ptr::null_mut(),
            )
        };

        if success.as_bool() {
            total_data.extend_from_slice(&buffer[..bytes_read as usize]);
            if (bytes_read as usize) < buffer.len() {
                break;
            }
        } else {
            let error = unsafe { GetLastError() };
            if error == ERROR_MORE_DATA {
                // There's more data to read, continue the loop.
                total_data.extend_from_slice(&buffer[..bytes_read as usize]);
                continue;
            } else {
                // An actual error occurred.
                return Err(windows::core::Error::from_win32());
            }
        }
    }

    let data_as_str = String::from_utf8(total_data)
        .map_err(|_| WinError::from_win32())?; // TODO: more specific error for UTF-8 parsing failure

    let json: JsonResult<JsonValue> = serde_json::from_str(&data_as_str);
    match json {
        Ok(json_value) => Ok(json_value),
        Err(_) => Err(WinError::from_win32()), // TODO: more specific error for JSON parsing failure
    }
}
