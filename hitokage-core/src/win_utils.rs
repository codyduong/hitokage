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
