use tauri::{command, AppHandle, Manager};
use windows::Win32::Foundation::HWND;

#[path ="helper/mod.rs"]
mod helper;
use helper::{archives, docs, monitor, win};

#[command]
pub fn show_open_with_dialog(app: AppHandle, path: &str) {
    if let Some(preview_window) = app.get_webview_window("preview") {
        let hwnd = preview_window.hwnd().map_or(HWND::default(), |hwnd| hwnd);
        let _ = win::show_open_with_dialog(path, hwnd);
    }
}

#[command]
pub fn archive(path: &str, mode: &str) -> Result<Vec<archives::Extract>, String> {
    match mode {
        "zip" => archives::Extract::zip(path).map_err(|e| e.to_string()),
        _ => Err("Not Support".to_string()),
    }
}

#[command]
pub fn document(path: &str, mode: &str) -> Result<docs::Docs, String> {
    match mode {
        "csv" => docs::Docs::csv(path).map_err(|e| e.to_string()),
        "xlsx" | "xls" | "xlsm" | "xlsb" | "xla" | "xlam" | "ods" => {
            docs::Docs::excel(path).map_err(|e| e.to_string())
        },
        "docx" => docs::Docs::docx(path).map_err(|e| e.to_string()),
        _ => Err("Not Support".to_string()),
    }
}

#[command]
pub fn get_monitor_info()-> monitor::MonitorInfo {
    monitor::get_monitor_info()
}

#[command]
pub fn get_default_program_name(path: &str)-> Result<String,String> {
    win::get_default_program_name(path)
}

// use windows::{
//     core::s,
//     Win32::{
//         Foundation::RECT,
//         UI::WindowsAndMessaging::{self},
//     },
// };
// #[tauri::command]
// pub fn set_into_taskbar(app: AppHandle, label: String) {
//     let w = app.get_webview_window(&label).unwrap();
//     let h = w.hwnd().unwrap();
//     set_taskbar(h);
// }

// fn set_taskbar(h: HWND) {
//     unsafe {
//         let shell_tray_wnd = WindowsAndMessaging::FindWindowA(s!("Shell_TrayWnd"), None).unwrap();
//         let tray =
//             WindowsAndMessaging::FindWindowExA(shell_tray_wnd, None, s!("TrayNotifyWnd"), None)
//                 .unwrap();
//         let rect = &mut RECT {
//             left: 0,
//             right: 0,
//             top: 0,
//             bottom: 0,
//         } as *mut RECT;
//         let _ = WindowsAndMessaging::GetWindowRect(tray, rect);
//         let r = *rect;
//         let _ = WindowsAndMessaging::SetParent(h, shell_tray_wnd);
//         let _ = WindowsAndMessaging::MoveWindow(h, r.left - 100, 0, 100, 60, false);
//     }
// }