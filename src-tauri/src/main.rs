#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod error;
mod library_metadata;

use config::{
    create_default_config, CombinedConfig, Config, ConfigState, DynamicEvent, DynamicEventList,
    LibraryCategoryList, StudentList,
};
use error::{Error, FakeError};
use glob::glob;
use library_metadata::LibraryMeta;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io, path::PathBuf, sync::Mutex};
use tauri::api::dialog;
use tauri::{AppHandle, Manager, PathResolver, State};

pub struct DatabaseState(pub Mutex<Arc<Pool<SqliteConnectionManager>>>);

impl DatabaseState {
    fn insert_file(&self, sha256: String) -> Result<(), Error> {
        let pool = self.0.lock().unwrap().clone();
        let connection = pool.get()?;
        connection.execute(
            "INSERT OR REPLACE INTO files (hash, in_store) VALUES (?,1);",
            [sha256.as_str()],
        )?;
        Ok(())
    }
}

#[tauri::command]
async fn reload_config_files(app: AppHandle, config: State<'_, ConfigState>) -> Result<(), Error> {
    let mut conf = config.0.lock().unwrap();
    *conf = read_all_configs(&app.path_resolver())?;
    Ok(())
}

fn read_all_configs(path_resolver: &PathResolver) -> Result<CombinedConfig, Error> {
    Ok(config::CombinedConfig(
        read_json_config_file::<Config>(path_resolver, "config.json")?.clone(),
        read_json_config_file::<DynamicEventList>(path_resolver, "dynamic-events.json")?.clone(),
        read_json_config_file::<LibraryCategoryList>(path_resolver, "library-categories.json")?
            .clone(),
        read_json_config_file::<StudentList>(path_resolver, "students.json")?.clone(),
    ))
}

fn read_json_config_file<T: for<'de> Deserialize<'de>>(
    path_resolver: &PathResolver,
    name: &str,
) -> Result<T, Error> {
    let mut config_file = PathResolver::app_config_dir(path_resolver).ok_or(FakeError {
        message: String::from("Cannot get app config directory"),
    })?;
    config_file.push(name);
    let f = fs::File::open(config_file)?;
    let j: T = serde_json::from_reader::<File, T>(f)?;
    Ok(j)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            reload_config_files,
            import_file_to_library
        ])
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            let path_resolver = app.path_resolver();
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");
                let mut has_error: Option<String> = None;

                let conf: Config = read_json_config_file(&path_resolver, "config.json")
                    .expect("Failed to create config");
                app_handle.manage(ConfigState(Mutex::new(conf.clone())));
                if let Some(x) = conf.library_path {
                    println!("Using library path: '{}'", x);
                    match create_hash_storage_database(x) {
                        Ok(x) => {
                            app_handle.manage(x);
                        }
                        Err(x) => {
                            has_error = Some(format!("Error indexing data store: {}", x));
                        }
                    };
                } else {
                    has_error = Some(String::from("No library path found"));
                }

                std::thread::sleep(std::time::Duration::from_secs(5));

                splashscreen_window.close().unwrap();
                match has_error {
                    Some(x) => {
                        dialog::message(Some(&splashscreen_window), "Tutor Resource Management", x);
                        main_window.close().unwrap();
                    }
                    None => main_window.show().unwrap(),
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn import_file_to_library(
    name: &str,
    config: State<'_, ConfigState>,
) -> Result<String, Error> {
    let p = Path::new(name).file_name().ok_or(FakeError {
        message: String::from("Cannot get filename"),
    })?;

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
    let lib_path = conf.library_path.clone().ok_or(FakeError {
        message: String::from("Cannot get library path"),
    })?;

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

fn create_information_database(config_path: String) -> Result<(), Error> {
    Ok(())
}

fn create_hash_storage_database(lib_path: String) -> Result<DatabaseState, Error> {
    let db_path = Path::new(&lib_path.clone()).join("index.sqlite");
    let glob_path = Path::new(&lib_path.clone()).join("**/*.meta");
    let glob_pattern = glob_path.to_str().expect("Wow");

    let sqlite_connection_manager = SqliteConnectionManager::file(db_path);
    let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager)?;
    let pool_arc = Arc::new(sqlite_pool);

    let pool = pool_arc.clone();
    let connection = pool.get()?;

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS files (
            hash TEXT NOT NULL,
            id INTEGER NOT NULL CONSTRAINT files_id_pk PRIMARY KEY AUTOINCREMENT,
            in_store BOOLEAN NOT NULL
        );
        CREATE TABLE IF NOT EXISTS cat (
            file_id INTEGER NOT NULL,
            category INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS files_hash_index ON files (hash);
        CREATE INDEX IF NOT EXISTS category_index ON cat (category);
        ",
        [],
    )?;

    let state = DatabaseState(Mutex::new(pool_arc));

    println!("Indexing files from hash store...");
    connection.execute("UPDATE files SET in_store = 0;", [])?;
    for entry in glob(glob_pattern).unwrap().filter_map(Result::ok) {
        println!("Check path: {}", entry.to_str().unwrap());
        match read_meta_file(entry) {
            Ok(meta) => {
                state.insert_file(meta.sha256).unwrap();
            }
            Err(x) => println!("Read Meta File: {}", x),
        }
    }
    println!("Finished initializing indexes...");
    Ok(state)
}

fn read_meta_file(meta_file: PathBuf) -> Result<LibraryMeta, Error> {
    let f = fs::File::open(meta_file)?;
    let j: LibraryMeta = serde_json::from_reader(f)?;
    Ok(j)
}
