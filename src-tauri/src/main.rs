#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod error;
mod library_metadata;

use config::{create_default_config, Config, ConfigState};
use error::{Error, FakeError};
use glob::glob;
use library_metadata::LibraryMeta;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::io::BufWriter;
use std::path::Path;
use std::{fs, io, path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Manager, PathResolver, State};

#[tauri::command]
async fn reload_config(app: AppHandle, config: State<'_, ConfigState>) -> Result<(), Error> {
    let mut conf = config.0.lock().unwrap();
    *conf = reload_config_internal(&app.path_resolver())?.clone();
    Ok(())
}

fn reload_config_internal(path_resolver: &PathResolver) -> Result<Config, Error> {
    let mut config_file = PathResolver::app_config_dir(path_resolver).ok_or(FakeError)?;
    config_file.push("config.json");
    let f = fs::File::open(config_file)?;
    let j: Config = serde_json::from_reader(f)?;
    Ok(j)
}

#[tauri::command]
async fn import_file_to_library(
    name: &str,
    config: State<'_, ConfigState>,
) -> Result<String, Error> {
    let p = Path::new(name).file_name().ok_or(FakeError)?;

    let mut hasher = Sha256::new();
    let mut file = fs::File::open(name)?;

    io::copy(&mut file, &mut hasher)?;
    let hash_bytes = hasher.finalize();
    let mut hash_string = String::new();
    for &byte in hash_bytes.as_slice() {
        write!(&mut hash_string, "{:x}", byte).expect("Unable to write");
    }

    let mut file_bin = hash_string.clone();
    file_bin.push_str(".bin");
    let mut file_meta = hash_string.clone();
    file_meta.push_str(".meta");

    let conf = config.0.lock().unwrap();
    let lib_path = conf.library_path.clone().ok_or(FakeError)?;

    let mut data_dir = PathBuf::new();
    data_dir.push(&lib_path[..]);
    let mut hash_path = data_dir.clone();
    hash_path.push(&hash_string[..2]);
    hash_path.push(&hash_string[2..4]);

    let mut data_file = hash_path.clone();
    data_file.push(file_bin);

    let mut meta_file = hash_path.clone();
    meta_file.push(file_meta);

    if Path::exists(&data_file) && Path::exists(&meta_file) {
        return Ok(hash_string);
    }

    let meta = LibraryMeta {
        sha256: hash_string.clone(),
        name: p.to_os_string().into_string().unwrap(),
        categories: vec![],
    };

    fs::create_dir_all(hash_path)?;
    fs::copy(name, data_file)?;

    let meta_writer = fs::File::create(meta_file)?;
    let mut writer = BufWriter::new(meta_writer);
    serde_json::to_writer(&mut writer, &meta)?;

    Ok(hash_string)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            reload_config,
            import_file_to_library
        ])
        .manage(ConfigState(Mutex::new(create_default_config())))
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            let path_resolver = app.path_resolver();
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");
                let conf = reload_config_internal(&path_resolver).expect("Failed to create config");
                match conf.library_path {
                    Some(x) => {
                        println!("Using library path: '{}'", x);
                        create_database(x);
                    }
                    None => {
                        println!("No library path found");
                    }
                }

                std::thread::sleep(std::time::Duration::from_secs(5));

                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_database(lib_path: String) {
    let db_path = Path::new(&lib_path.clone()).join("index.sqlite");
    let glob_path = Path::new(&lib_path.clone()).join("**/*.meta");
    let glob_pattern = glob_path.to_str().expect("Wow");

    let connection = sqlite::open(db_path).unwrap();
    let query = "
        CREATE TABLE IF NOT EXISTS files (
            hash TEXT NOT NULL,
            id INTEGER NOT NULL CONSTRAINT files_id_pk PRIMARY KEY AUTOINCREMENT
        );
        CREATE TABLE IF NOT EXISTS cat (
            file_id INTEGER NOT NULL,
            category INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS category_index ON cat (category);
    ";
    match connection.execute(query) {
        Err(x) => {
            println!("Setup query failed: {}", x);
        }
        _ => {
            println!("Indexing files from hash store...");
            for entry in glob(glob_pattern).expect("Failed to read glob pattern") {
                if let Ok(path) = entry {
                    if let Ok(meta) = read_meta_file(path) {
                        let mut statement = connection
                            .prepare("INSERT OR REPLACE INTO files (hash) VALUES (?)")
                            .unwrap();
                        statement.bind((1, meta.sha256.as_str())).unwrap();
                    }
                }
            }
            println!("Finished initializing indexes...");
        }
    }
}

fn read_meta_file(meta_file: PathBuf) -> Result<LibraryMeta, Error> {
    let f = fs::File::open(meta_file)?;
    let j: LibraryMeta = serde_json::from_reader(f)?;
    Ok(j)
}
