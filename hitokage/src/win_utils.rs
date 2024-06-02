// use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;


// pub fn get_taskbar_height() -> i32 {
//   unsafe {
//       let mut appbar_data = APPBARDATA {
//           cbSize: std::mem::size_of::<APPBARDATA>() as u32,
//           ..Default::default()
//       };

//       let result = SHAppBarMessage(ABM_GETTASKBARPOS, &mut appbar_data);

//       if result == 0 {
//           // Failed to get the taskbar position, assume height of 40
//           40
//       } else {
//           match appbar_data.uEdge {
//               ABE_TOP | ABE_BOTTOM => appbar_data.rc.bottom - appbar_data.rc.top,
//               ABE_LEFT | ABE_RIGHT => appbar_data.rc.right - appbar_data.rc.left,
//               _ => 40, // Default height if unknown
//           }
//       }
//   }
// }

pub fn get_borders() -> (i32, i32) {
  unsafe {
      // im not entirely sure what this is LOL -@codyduong
      let caption_padding = GetSystemMetrics(SM_CXPADDEDBORDER);

      // this is off by a factor of 1/2 on my machine, idk if this works for everyone LOL -@codyduong
      // let x_border = 2 * (GetSystemMetrics(SM_CXSIZEFRAME) + caption_padding);
      // let y_border = 2 * (GetSystemMetrics(SM_CYSIZEFRAME) + caption_padding);

      println!("{:?}", (GetSystemMetrics(SM_CXFIXEDFRAME)));

      let x_border = GetSystemMetrics(SM_CXEDGE);
      let y_border = GetSystemMetrics(SM_CYSIZEFRAME);
      
      (x_border, y_border)
  }
}

pub fn get_primary_width() -> i32 {
  unsafe {
    GetSystemMetrics(SM_CXSCREEN)
  }
}