use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

mod preview;
mod tray;

#[path = "./command.rs"]
mod command;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // 自动启动
            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();
            // 创建托盘
            tray::create_tray(app)?;
            // 初始化预览文件
            let app_handle = app.handle().clone();
            preview::init_preview_file(app_handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command::preview_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
