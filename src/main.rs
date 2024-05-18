use clap::{Arg, Command};
use serde::Deserialize;

use vimoxide::constants::CONFIG_FILE;

mod file_handling;
mod history;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub executor: String,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Failed to find the config directory")?;
    let config_file_path = config_dir.join("vimoxide").join(CONFIG_FILE);

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

fn main() {
    let matches = Command::new("vimoxide")
        .version("0.1.0")
        .author("Author: ZKAW (https://github.com/ZKAW)")
        .about("A tool to quickly open files with Vim / Nvim based on frequency of access")
        .arg(
            Arg::new("file")
                .help("The file to open with Vim / Nvim")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();
    let config = load_config().unwrap_or_else(|err| {
        eprintln!("Failed to load config: {}", err);
        std::process::exit(1);
    });

    let mut db = history::load_history();

    let best_match_path = history::find_best_match(&db, file);
    if let Some(ref path) = best_match_path {
        file_handling::open_with_executor(path, &config.executor);
        history::update_history(&mut db, path);
        history::save_history(&db).expect("Failed to save the history");
    }
}
