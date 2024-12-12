use core::panic;
use directories_next::ProjectDirs;
use serde_json::json;
use std::{fs, io::Read};

#[tauri::command]
fn get_items() -> String {
    let default_json = json!([{
        "title": "Eat Grass",
        "date": "All The Time",
        "urgency": "High",
        "completed": false,
    }])
    .to_string();
    let proj_dirs =
        ProjectDirs::from("com", "remindeer", "app").expect("Could Not Find Project Directories");
    let data_dir = proj_dirs.data_dir();
    let file_path = data_dir.join("items.json");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("Could not Create Project Directory");
    }
    if !file_path.exists() {
        match fs::File::create(&file_path) {
            Ok(_) => {
                fs::write(&file_path, &default_json).expect("Could not Write Default Json To Disk");
            }
            Err(_) => panic!("Something has SERIOUSLY gone wrong"),
        };
    }
    let mut buff = String::new();
    match fs::File::open(&file_path) {
        Ok(mut file) => match file.read_to_string(&mut buff) {
            Ok(_) => buff,
            Err(_) => default_json,
        },
        Err(_) => default_json,
    }
}

#[tauri::command]
fn write_items(data: &str) -> Result<i32, &str> {
    let proj_dirs =
        ProjectDirs::from("com", "remindeer", "app").expect("Could Not Find Project Directories");
    let data_dir = proj_dirs.data_dir();
    let file_path = data_dir.join("items.json");
    match fs::write(&file_path, data) {
        Ok(_) => Ok(0),
        Err(_) => Err("Unable to Write To Disk"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_items, write_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
