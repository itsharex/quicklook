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


#[command]
pub fn archive(path: &str, mode: &str) -> Result<Vec<helper::Extract>, String> {
  match mode {
    "zip" => helper::Extract::zip(path).map_err(|e| e.to_string()),
    _ => {
      Err("Not Support".to_string())
    }
      
  }
}

#[command]
pub fn document(path: &str, mode: &str) -> Result<Vec<helper::DSheet>, String> {
    match mode {
        "csv" => helper::Document::csv(path).map_err(|e| e.to_string()),
        "xlsx" | "xls" => helper::Document::excel(path).map_err(|e| e.to_string()),
        _ => {
            Err("Not Support".to_string())
        }
    }
}
