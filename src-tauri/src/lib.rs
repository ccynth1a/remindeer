// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{fs, io::Read};
use serde_json::json;
use std::path::Path;
use thiserror::Error;

#[tauri::command]
fn get_items() -> String {
    let default_json = json!([{
        "title": "Eat Grass",
        "date": "All The Time",
        "urgency": "High",
        "completed": false,
    }]).to_string();
    if !Path::new("items.json").exists() {
        match fs::File::create("items.json") {
            Ok(_) => {
                fs::write("items.json", &default_json).expect("Failed on Write");
            }
            Err(_) => panic!("FUCK")
        }
    }
    let mut buff = String::new();
    match fs::File::open("items.json") {
        Ok(mut file) => {
            match file.read_to_string(&mut buff) {
                Ok(_) => buff,
                Err(_) => default_json
            }
        },
        Err(_) => default_json
    }
}

#[tauri::command]
fn write_items(data: &str) -> Result<i32, &str> {
    // DEER FILE IS GUARANTEED TO EXIST
    match fs::write("items.json", data) {
        Ok(_) => Ok(0),
        Err(_) => Err("Unable to Write To Disk")
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
        .invoke_handler(tauri::generate_handler![greet, get_items, write_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
