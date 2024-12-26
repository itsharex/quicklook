use std::ptr::null_mut;

use windows::Win32::{
    Foundation::HWND, 
    Graphics::Gdi::{GetDC, GetDeviceCaps, ReleaseDC, HORZRES, LOGPIXELSX, VERTRES}
};

#[derive(Debug,serde::Serialize,Clone)]
pub struct MonitorInfo {
    width: i32,
    height: i32,
    scale: f64
}

impl Default for MonitorInfo {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default(), scale: Default::default() }
    }
}

pub fn get_monitor_info() -> MonitorInfo {
    let hwnd = HWND(null_mut());
    let hdc = unsafe { GetDC(hwnd) };
    if hdc.is_invalid() {
        return MonitorInfo::default();
    }
    
    // 获取屏幕分辨率
    let width = unsafe {GetDeviceCaps(hdc, HORZRES)};
    let height = unsafe {GetDeviceCaps(hdc, VERTRES)};

    // 获取缩放比例
    let logical_width = unsafe {GetDeviceCaps(hdc, LOGPIXELSX)};
    let scale_factor = logical_width as f64 / 96.0;

    unsafe {ReleaseDC(hwnd, hdc)};
    
    MonitorInfo {width, height, scale: scale_factor}
}
