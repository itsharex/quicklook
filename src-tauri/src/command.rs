use tauri::{command, AppHandle, Manager};
use windows::Win32::Foundation::HWND;

mod helper;

#[command]
pub fn show_open_with_dialog(app: AppHandle, path: &str) {
    if let Some(preview_window) = app.get_webview_window("preview") {
        let hwnd = preview_window.hwnd().map_or(HWND::default(), |hwnd| {
            hwnd
        });
        let _ = helper::show_open_with_dialog(path, hwnd);
    }
    
}
