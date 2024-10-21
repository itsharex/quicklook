use tauri::command;

#[path = "./utils/mod.rs"]
mod utils;
use utils::preview;

#[command]
pub fn preview_file(path: String) -> Result<preview::File, String> {
    preview::preview_file(path)
}