// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn add_todo(title: &str) -> String {
    // TODOの追加処理
    format!("タスクを追加しました : {}", title)
}

#[tauri::command]
fn remove_todo() -> String {
    // TODOの削除処理
    format!("タスクを削除しました")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_todo, remove_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
