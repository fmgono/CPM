// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use std::path::Path;
use walkdir::WalkDir;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_all_projects])
    // .invoke_handler(tauri::generate_handler![get_all_projects])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn get_all_projects() {
  let start_dir = "/Users/fmgono/";
  let mut project_names = vec![];

  let walker = WalkDir::new(start_dir)
    .into_iter()
    .filter_entry(|e| e.file_name() != "node_modules");

  for entry in walker.filter_map(|e| e.ok()) {
    if entry.file_name() == "package.json" {
      if let Some(parent) = entry.path().parent() {
        if let Some(project_name) = parent.file_name() {
          let size = get_directory_size(parent);
          project_names.push((project_name.to_string_lossy().to_string(), size))
        } else {
          eprint!("Error: Could not get parent directory name")
        }
      }
    } else {
      eprint!("Error: Could not get parent directory")
    }
  }
}

fn get_directory_size(path: &Path) -> u64 {
  let metadata = fs::metadata(path).unwrap();
  if metadata.is_dir() {
      let mut total_size = 0;
      for entry in fs::read_dir(path).unwrap() {
          let entry = entry.unwrap();
          let file_type = entry.file_type().unwrap();
          if file_type.is_file() {
              total_size += entry.metadata().unwrap().len();
          } else if file_type.is_dir() {
              total_size += get_directory_size(&entry.path());
          }
      }
      total_size
  } else {
      metadata.len()
  }
}
