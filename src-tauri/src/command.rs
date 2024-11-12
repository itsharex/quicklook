use tauri::command;

#[command]
pub fn my_custom_command() {
    println!("Hello from my custom command!");
}
