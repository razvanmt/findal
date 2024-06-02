#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use dashmap::DashMap;
use rayon::prelude::*;
use tauri::{command, State};
use showfile::show_path_in_file_manager;

struct AppState {
    index: Arc<DashMap<String, (PathBuf, u64)>>,
}

#[command]
async fn index_files(state: State<'_, AppState>) -> Result<(), String> {
    let drives = vec!["D:\\"]; // Add more drives as needed
    let index = state.index.clone();

    drives.into_par_iter().for_each(|drive| {
        walk_path(Path::new(drive), &index);
    });

    println!("Indexing completed");
    Ok(())
}

#[command]
async fn search_file(state: State<'_, AppState>, file_name: String) -> Result<Option<String>, String> {
    let index = state.index.clone();
    Ok(index.get(&file_name).map(|entry| entry.value().0.display().to_string()))
}

#[command]
async fn get_indexed_files(state: State<'_, AppState>) -> Result<Vec<(String, u64)>, String> {
    let index = state.index.clone();
    let files = index.iter().map(|entry| (entry.key().clone(), entry.value().1)).collect();
    Ok(files)
}

#[command]
async fn open_file_in_explorer(file_path: String) -> Result<(), String> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("File not found".into());
    }
    show_path_in_file_manager(path);

    Ok(())
}

fn walk_path(path: &Path, index: &Arc<DashMap<String, (PathBuf, u64)>>) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let mut files = vec![];
    let mut directories = vec![];

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    directories.push(path);
                } else {
                    files.push(path);
                }
            }
            Err(_) => continue,
        }
    }

    files.into_par_iter().for_each(|path| {
        let metadata = fs::metadata(&path).unwrap();
        let file_size = metadata.len();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        index.insert(file_name.clone(), (path.clone(), file_size));
        println!("Indexed file: {} (size: {} bytes)", file_name, file_size); // Add this line to see indexed files and sizes
    });

    directories.into_par_iter().for_each(|dir| {
        walk_path(&dir, index);
    });
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            index: Arc::new(DashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![index_files, search_file, get_indexed_files, open_file_in_explorer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
