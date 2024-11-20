use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl, webview::WebviewWindow, Error as TError};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging;

pub fn get_window_class_name(hwnd: HWND) -> String {
    let mut class_name = [0u16; 256];
    let len = unsafe { WindowsAndMessaging::GetClassNameW(hwnd, &mut class_name) };
    String::from_utf16_lossy(&class_name[0..len as usize])
}

pub fn get_webview_window(app: &AppHandle, label: &str, url: &str) -> Result<WebviewWindow, TError> {
    match app.get_webview_window(label) {
        Some(window) => Ok(window),
        None => WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into())).center().build(),
    }
}