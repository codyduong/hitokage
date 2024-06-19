use std::mem::MaybeUninit;
use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};
use windows::Win32::{System::Threading::GetCurrentThreadId, UI::WindowsAndMessaging::*};

pub fn get_primary_width() -> i32 {
  unsafe { GetSystemMetrics(SM_CXSCREEN) }
}

pub fn get_primary_height() -> i32 {
  unsafe { GetSystemMetrics(SM_CYSCREEN) }
}

pub fn get_current_thread_id() -> u32 {
  unsafe { GetCurrentThreadId() }
}

pub fn get_windows_version() -> u32 {
  unsafe {
    let mut os_info = MaybeUninit::<OSVERSIONINFOW>::zeroed();
    let os_info_ptr = os_info.as_mut_ptr();
    (*os_info_ptr).dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;

    if GetVersionExW(os_info_ptr).is_ok() {
      let build = os_info.assume_init().dwBuildNumber;
      if build >= 22000 {
        11
      } else if build >= 10240 {
        10
      } else {
        0
      }
    } else {
      panic!("Failed to get Windows version")
    }
  }
}
