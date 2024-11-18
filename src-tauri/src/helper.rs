use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging;

use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, WebviewWindow};

pub fn get_window_class_name(hwnd: HWND) -> String {
    let mut class_name = [0u16; 256];
    let len = unsafe { WindowsAndMessaging::GetClassNameW(hwnd, &mut class_name) };
    String::from_utf16_lossy(&class_name[0..len as usize])
}

pub fn create_preview_window(app: &AppHandle) -> Result<WebviewWindow, tauri::Error> {
    
    let result = WebviewWindowBuilder::new(app, "preview", WebviewUrl::App("/preview".into()))
        .build();

    if let Ok(ref preview) = result {
        let preview_clone = preview.clone();
        let _ = preview.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                println!("preview window closed");
                let _ = preview_clone.clone().close();
            }
            
        });
        let _ = preview.center();
        let _ = preview.set_decorations(false);
        let _ = preview.set_skip_taskbar(true);
        let _ = preview.show();
        let _ = preview.set_focus();
    }

    
    result
}
