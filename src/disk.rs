use crate::options::Message;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Read;
use serde::{ Deserialize };
use rustix::process;

#[derive(Deserialize)]
pub struct Config {
    pub model:       String,
    pub temperature: f32,
    pub color:       String,
}

pub struct Paths {
    pub config: PathBuf,
    pub log:    PathBuf,
}

impl Paths {
    pub fn get() -> Self {
        let pid = process::getppid()
            .unwrap()
            .as_raw_nonzero()
            .to_string();

        let main_dir_path = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(".meek");

        let config_path = main_dir_path
            .join("config.toml");

        let log_path = main_dir_path
            .join(format!(".meek_log_{}", pid));

        if !main_dir_path.exists() {
            fs::create_dir(&main_dir_path).unwrap();
        }
        if !config_path.exists() {
            let default_config = include_str!("default_config.toml");
            fs::write(&config_path, default_config).unwrap();
        }
        return Self {
            config: config_path,
            log: log_path,
        }
    }
}

pub fn load_config(config_path: &PathBuf) -> Config {
   let toml_str = fs::read_to_string(&config_path)
       .expect("Failed to read config");
   let config: Config = toml::from_str(&toml_str)
       .expect("Failed to parse config");
   return config;
}

pub fn clear_log(path: &PathBuf) {
    match fs::remove_file(&path) {
        Ok(_)  => println!("\nChat history file deleted.\n"),
        Err(_) => println!("\nNo file to delete.\n"),
    };
}

pub fn load_log(path: &PathBuf) -> Vec<Message> {

    // open file
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(&path)
        .unwrap();

    let mut log_str = String::new();

    // convert to string
    log_file.read_to_string(&mut log_str)
        .unwrap();

    // format to JSON
    let messages = match serde_json::from_str(&log_str) {
        Ok(value) => value,
        Err(_)    => Vec::new(),
    };

    return messages
}

pub fn write_log(path: &PathBuf, messages: &Vec<Message>) {
    
    if let Ok(json) = serde_json::to_string(&messages) {
        fs::write(&path, json)
            .expect("Failed writing to log file");
    }
}
