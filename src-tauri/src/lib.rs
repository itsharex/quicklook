use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

mod preview;
mod tray;

#[path = "./command.rs"]
mod command;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
            
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
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
                        .max_file_size(50000)
                        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
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
        .invoke_handler(tauri::generate_handler![
            command::show_open_with_dialog,
            command::archive,
            command::document
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
