// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod executor;
pub mod external_editor;

use std::env::temp_dir;
use std::fs::{self, remove_file, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use external_editor::ArgParams;
use serde::{Deserialize, Serialize};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigTemplate {
    name: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    version: u32,
    templates: Vec<ConfigTemplate>,
}

#[tauri::command]
fn load_config() -> Result<Config, String> {
    let toml_content = fs::read_to_string("config.toml");
    if toml_content.is_err() {
        let message = format!("Error reading config file. {:?}", toml_content.err());
        eprintln!("{}", message);
        return Err(message);
    }

    let config: Config = toml::from_str(&toml_content.unwrap()).unwrap();
    println!("{:?}", config);
    Ok(config)
}

fn initialize_tempfile(text: impl Into<String>) -> PathBuf {
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
fn open_by_external_editor(text: String) -> String {
    // write
    let path = initialize_tempfile(&text);

    // edit
    let editor = external_editor::ExternalEditor::new(
        "Vim",
        // "F:\\software\\vim\\vim90\\gvim.exe",
        "notepad",
        // "F:\\software\\Notepad++\\notepad++.exe",
        "{FilePath}",
    );
    let mut params = ArgParams::new();
    params.set_file_path(path.to_string_lossy());
    editor.execute(params);

    // read
    let mut buf = String::new();
    {
        let mut file = File::open(&path).unwrap();
        let read_bytes = match file.read_to_string(&mut buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading file: {:?}", e);
                return text;
            }
        };
        println!("Read: ({} bytes) {}", read_bytes, buf);

        match remove_file(&path) {
            Ok(_) => println!("temporary file removed"),
            Err(e) => eprintln!("Error removing file: {:?}", e),
        }
    }

    buf
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            open_by_external_editor,
        ])
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
