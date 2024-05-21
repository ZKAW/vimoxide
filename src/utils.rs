use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

use vimoxide::constants::CONFIG_FILE;
use vimoxide::constants::HISTORY_FILE;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub executor: String,
}

// Will get the config file even if script is running as root
pub fn get_config_dir() -> PathBuf {
    // Use SUDO_USER
    let user = env::var("SUDO_USER")
        .unwrap_or_else(|_| env::var("USER").expect("Failed to get the current user"));

    let mut config_dir = PathBuf::from("/home").join(user).join(".config");
    if !config_dir.exists() {
        config_dir = dirs::config_dir().expect("Failed to get the config directory");
    }

    config_dir = config_dir.join("vimoxide");

    // If config_dir does not exist, create it
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create the configuration directory");
    }

    config_dir
}

pub fn get_config_file() -> PathBuf {
    get_config_dir().join(CONFIG_FILE)
}

pub fn get_history_file() -> PathBuf {
    get_config_dir().join(HISTORY_FILE)
}

pub fn load_config_file() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file_path = get_config_file();

    // Try to read the configuration file
    let config_file = match std::fs::File::open(config_file_path) {
        Ok(file) => file,
        Err(_) => {
            return Ok(Config {
                executor: "vim".to_string(),
            });
        }
    };

    let config: Config = match serde_json::from_reader(config_file) {
        Ok(config) => config,
        Err(_) => {
            return Ok(Config {
                executor: "vim".to_string(),
            });
        }
    };

    if config.executor != "vim" && config.executor != "nvim" {
        return Ok(Config {
            executor: "vim".to_string(),
        });
    }

    Ok(config)
}
