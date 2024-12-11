// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{fs, io::Read};
use serde_json::json;
use std::path::Path;

#[tauri::command]
fn get_items() -> String {
    let default_json = json!([{
        "title": "Eat Grass",
        "date": "All The Time",
        "urgency": "High",
        "completed": false,
    }]).to_string();
    if !Path::new("items.json").exists() {
       return default_json
    }
    let mut buff = String::new();
    match fs::File::open("items.json") {
        Ok(mut file) => {
            file.read_to_string(&mut buff).expect("Failed to Read JSON To String");
            buff
        },
        Err(_) => default_json
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
