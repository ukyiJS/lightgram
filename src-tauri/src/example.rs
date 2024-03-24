#[tauri::command]
pub fn example_data() -> String {
    format!("example data")
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
