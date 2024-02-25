// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod config;
pub mod executor;
pub mod external_editor;

use std::fs::{self, remove_file, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use config::config::Config;
use external_editor::ArgParams;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window, WindowEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct TemplayState {
    config: Mutex<Config>,
    temporary_file: Mutex<Option<PathBuf>>,
}

impl TemplayState {
    fn new() -> Self {
        let config = Config::default();
        TemplayState {
            config: Mutex::new(config),
            temporary_file: Mutex::new(None),
        }
    }
}

#[tauri::command]
fn load_config(state: tauri::State<'_, TemplayState>) -> Result<Config, String> {
    let toml_content = fs::read_to_string("config.toml");
    if toml_content.is_err() {
        let message = format!("Error reading config file. {:?}", toml_content.err());
        eprintln!("{}", message);
        return Err(message);
    }

    let config: Config = toml::from_str(&toml_content.unwrap()).unwrap();
    println!("{:?}", config);

    *state.config.lock().unwrap() = config.clone();
    Ok(config)
}

#[tauri::command]
fn save_config(state: tauri::State<'_, TemplayState>, config: Config) {
    let mut file = File::create("config.toml").unwrap();
    let toml_content = toml::to_string(&config).unwrap();
    write!(file, "{}", toml_content).unwrap();
    file.flush().unwrap();

    *state.config.lock().unwrap() = config.clone();
}

fn initialize_tempfile(state: &tauri::State<'_, TemplayState>, text: impl Into<String>) -> PathBuf {
    if state.temporary_file.lock().unwrap().is_some() {
        // truncate temporary file
        let path = state.temporary_file.lock().unwrap().clone().unwrap();
        let mut file = File::create(&path).unwrap();
        file.set_len(0).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        match write!(file, "{}", text.into()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error writing file: {:?}", e);
            }
        }

        return path;
    }

    let mut builder = tempfile::Builder::new();
    let path_builder = builder.prefix("templay.").suffix(".txt");
    let mut tempfile = path_builder.tempfile().unwrap();

    let content = text.into();

    println!("Write: {}", content);
    match write!(tempfile, "{}", content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error writing file: {:?}", e);
        }
    };
    let (_, path) = tempfile.keep().unwrap();

    path
}

#[tauri::command]
fn read_tempfile(state: tauri::State<'_, TemplayState>) -> Result<String, Option<String>> {
    let path = match state.temporary_file.lock().unwrap().clone() {
        Some(p) => p,
        None => {
            eprintln!("temporary file is not set.");
            return Err(None);
        }
    };

    let attr = match fs::metadata(&path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            return Err(None);
        }
    };

    if attr.modified().unwrap() <= attr.created().unwrap() {
        return Err(None);
    }

    let mut buf = String::new();
    {
        let mut file = File::open(&path).unwrap();
        let read_bytes = match file.read_to_string(&mut buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading file: {:?}", e);
                return Err(None);
            }
        };
        println!("Read: ({} bytes) {}", read_bytes, buf);
    }

    Ok(buf)
}

#[tauri::command]
fn open_with_external_editor(state: tauri::State<'_, TemplayState>, text: String) {
    let config = state.config.lock().unwrap().clone();
    let external_editor_config = match config.external_editor {
        Some(e) => e,
        None => {
            eprintln!("external editor is not set.");
            return;
        }
    };

    // write
    let path = initialize_tempfile(&state, &text);
    *state.temporary_file.lock().unwrap() = Some(path.clone());

    // edit
    let editor = external_editor::ExternalEditor::new(
        external_editor_config.name,
        external_editor_config.command,
        external_editor_config.args,
    );
    let mut params = ArgParams::new();
    params.set_file_path(path.to_string_lossy());
    editor.execute(params);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            save_config,
            open_with_external_editor,
            read_tempfile,
        ])
        .setup(|app| {
            let state = TemplayState::new();
            app.manage(state);

            // 開発時は devtools を起動時に表示
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // remove temporary file
                let path = match event
                    .window()
                    .state::<TemplayState>()
                    .temporary_file
                    .lock()
                    .unwrap()
                    .clone()
                {
                    Some(p) => p,
                    None => {
                        eprintln!("temporary file is not set.");
                        return;
                    }
                };
                match remove_file(&path) {
                    Ok(_) => println!("temporary file removed"),
                    Err(e) => eprintln!("Error removing file: {:?}", e),
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
