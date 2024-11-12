use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

pub async fn update(app: &AppHandle) -> tauri_plugin_updater::Result<()> {
    let checked = app.updater()?.check().await?;

    match checked {
        Some(update) => {
            println!("checked: {:?}", update.version);
            let mut downloaded = 0;

            // alternatively we could also call update.download() and update.install() separately
            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        println!("downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                        println!("download finished");
                    },
                )
                .await?;

            println!("update installed");
            app.restart();
        }
        None => {
            app.dialog()
                .message("没有可用更新").kind(MessageDialogKind::Warning)
                .title("更新")
                .blocking_show();
        }
    }

    Ok(())
}
