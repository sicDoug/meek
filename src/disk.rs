use crate::options::Message;

use std::{
    fs,
    fs::OpenOptions,
    path::PathBuf,
    io::Read,
};
use serde::Deserialize;
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

pub fn create_config(config_path: &PathBuf) -> Result<(), String> {
    let default_config = include_str!("default_config.toml");

    fs::write(&config_path, default_config)
        .map_err(|e| format!("Failed to distribute config: {}", e))?;

    Ok(())
}

impl Paths {
    pub fn get() -> Result<Self, String> {

        // get the path for the main dir
        let main_dir_path = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join(".meek");

        // get the path for the config file
        let config_path = main_dir_path
            .join("config.toml");

        // get the PID of the terminal instance
        let pid = process::getppid()
            .ok_or_else(|| "Failed to get parent PID".to_string())?
            .as_raw_nonzero()
            .to_string();

        // get the path for the current log file
        let log_path = main_dir_path
            .join(format!(".meek_log_{}", pid));

        // create the main dir if necessary
        if !main_dir_path.exists() {
            fs::create_dir(&main_dir_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // copy the default config to the main dir if necessary
        if !config_path.exists() {
            create_config(&config_path)?;
        }

        Ok(Self {
            config: config_path,
            log: log_path,
        })
    }
}

pub fn load_config(config_path: &PathBuf) -> Result<Config, String> {

    // read to string
    let toml_str = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config: {}", e))?;

    // map to config struct
    let config: Config = toml::from_str(&toml_str)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    Ok(config)
}

pub fn clear_log(path: &PathBuf) -> Result<(), String> {

    // delete the current log file
    fs::remove_file(&path)
        .map_err(|e| format!("Failed to delete file: {}", e))
}

pub fn load_log(path: &PathBuf) -> Result<Vec<Message>, String> {

    // return empty vec if log does not yet exist
    if !path.exists() {
        return Ok(Vec::new());
    }

    // open file
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(&path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    let mut log_str = String::new();

    // convert to string
    log_file.read_to_string(&mut log_str)
        .map_err(|e| format!("Failed to read log file: {}", e))?;

    // format JSON
    serde_json::from_str(&log_str)
        .map_err(|e| format!("Failed to parse log file: {}", e))
}

pub fn write_log(
    path:     &PathBuf,
    messages: &Vec<Message>
) -> Result<(), String> {

    // format to JSON
    let json = serde_json::to_string(&messages)
        .map_err(|e| format!("Failed to serialize messages: {}", e))?;
    // write to file
    fs::write(&path, json)
        .map_err(|e| format!("Failed to write to log file: {}", e))
}
