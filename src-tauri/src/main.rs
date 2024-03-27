// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize}; // Add this line



fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, list_folders, list_projects])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn list_folders() -> Vec<String> {
    if let Some(home_dir) = dirs::home_dir() {
        if let Ok(entries) = fs::read_dir(home_dir) {
            let folders = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    if let Ok(metadata) = entry.metadata() {
                      if metadata.is_dir() {
                        println!("metadata {:?}", metadata);
                        if let Some(dir) = entry.file_name().to_str() {
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_string_lossy();
                            if !file_name_str.contains(".") {
                              return Some(file_name_str.to_string());
                            }
                          }
                        }
                    }
                    None
                })
                .collect::<Vec<String>>();
            return folders;
        }
    }
    Vec::new()
}

#[derive(Debug, Serialize, Deserialize)]
struct FolderInfo {
  name: String,
  size: String,
  modified_at : u64,
  created_at: u64
}

fn convert_size(size: u64) -> String {
  const KB: f64 = 1024.0;
  const MB: f64 = KB * 1024.0;
  const GB: f64 = MB * 1024.0;

  if size < KB as u64 {
    format!("{} B", size)
  } else if size < MB as u64 {
    format!("{:.1} KB", size as f64 / KB)
  } else if size < GB as u64 {
    format!("{:.1} MB", size as f64 / MB)
  } else {
    format!("{:.1} GB", size as f64 / GB)

  }
}

#[tauri::command]
fn list_projects() -> Vec<FolderInfo> {
    if let Some(home_dir) = dirs::home_dir() {
        if let Ok(entries) = fs::read_dir(home_dir) {
            let folders = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                  let metadata = entry.metadata().ok()?;
                  if metadata.is_dir() {
                    let modified_at = metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs();
                    let created_at = metadata.created().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs();
                    let size = convert_size(metadata.len());
                    let filename = entry.file_name();
                    let name_str = filename.to_string_lossy();
                    if !name_str.contains(".") {
                      Some(FolderInfo {
                        name: name_str.to_string(),
                        size,
                        modified_at,
                        created_at
                      })
                    } else {
                      None
                    }
                  } else {
                    None
                  }
                })
                .collect::<Vec<FolderInfo>>();
            return folders;
        }
    }
    Vec::new()
}
