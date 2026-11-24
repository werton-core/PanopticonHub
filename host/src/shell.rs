use std::{ffi::OsStr,os::windows::ffi::OsStrExt,ptr};
use windows_sys::Win32::{Foundation::{CloseHandle,GetLastError,ERROR_ALREADY_EXISTS,HANDLE},System::Threading::CreateMutexW,UI::{Shell::ShellExecuteW,WindowsAndMessaging::{MessageBoxW,MB_ICONERROR,SW_SHOWNORMAL}}};
pub const WINDOW_CLASS: &str = "PanopticonHub.Window";

pub fn round_window_corners(window: isize) -> bool {
    use windows_sys::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_BORDER_COLOR, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWA_COLOR_NONE,
        DWMWCP_ROUND,
    };
    let window = window as _;
    unsafe {
        let none = DWMWA_COLOR_NONE;
        DwmSetWindowAttribute(window, DWMWA_BORDER_COLOR as u32, (&none as *const u32).cast(), 4);
        let round = DWMWCP_ROUND;
        DwmSetWindowAttribute(window, DWMWA_WINDOW_CORNER_PREFERENCE as u32, (&round as *const i32).cast(), 4) >= 0
    }
}

pub fn clip_window_corners(window: isize, width: u32, height: u32, scale: f64, maximized: bool) {
    use windows_sys::Win32::Graphics::Gdi::{CreateRoundRectRgn, DeleteObject, SetWindowRgn};
    let window = window as _;
    unsafe {
        if maximized {
            SetWindowRgn(window, ptr::null_mut(), 1);
            return;
        }
        let diameter = (16.0 * scale).round() as i32;
        let region = CreateRoundRectRgn(0, 0, width as i32 + 1, height as i32 + 1, diameter, diameter);
        if !region.is_null() && SetWindowRgn(window, region, 1) == 0 {
            DeleteObject(region);
        }
    }
}
fn wide(s:&OsStr)->Vec<u16>{s.encode_wide().chain([0]).collect()}
pub fn open(target:&OsStr)->Result<(),String>{unsafe{
    let res=ShellExecuteW(ptr::null_mut(),wide(OsStr::new("open")).as_ptr(),wide(target).as_ptr(),ptr::null(),ptr::null(),SW_SHOWNORMAL);
    if res as isize<=32{Err(format!("Windows не смогла открыть ресурс ({}).",res as isize))}else{Ok(())}
}}
pub fn error(message:&str){unsafe{MessageBoxW(ptr::null_mut(),wide(OsStr::new(message)).as_ptr(),wide(OsStr::new("Panopticon Hub")).as_ptr(),MB_ICONERROR);}}
pub fn activate(){unsafe{
    use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW,ShowWindow,SetForegroundWindow,SW_RESTORE};
    let hwnd=FindWindowW(wide(OsStr::new(WINDOW_CLASS)).as_ptr(),ptr::null());
    if !hwnd.is_null(){ShowWindow(hwnd,SW_RESTORE);SetForegroundWindow(hwnd);}
}}
pub struct Instance(HANDLE);
impl Instance{
    pub fn acquire()->Result<Option<Self>,String>{unsafe{
        let lock=CreateMutexW(ptr::null(),0,wide(OsStr::new("Local\\PanopticonHub.ReleasePort")).as_ptr());
        if lock.is_null(){return Err(std::io::Error::last_os_error().to_string())}
        if GetLastError()==ERROR_ALREADY_EXISTS{CloseHandle(lock);Ok(None)}else{Ok(Some(Self(lock)))}
    }}
}
impl Drop for Instance{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
