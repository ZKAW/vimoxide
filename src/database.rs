use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use serde::Deserialize;

use vimoxide::constants::DATABASE_FILE;
use vimoxide::constants::CONFIG_FILE;


#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub rank: usize,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub executor: String,
}

pub fn load_database() -> HashMap<PathBuf, FileEntry> {
    let mut db = HashMap::new();
    let db_dir = dirs::config_dir().unwrap().join("vimoxide");
    fs::create_dir_all(&db_dir).expect("Failed to create directory for database");

    let db_path = db_dir.join(DATABASE_FILE);
    if !db_path.exists() {
        return db;
    }

    let mut file = fs::File::open(&db_path).expect("Failed to open the database file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read the database file");

    for line in contents.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let path = PathBuf::from(parts[0]);
            if let Ok(rank) = parts[1].parse() {
                db.insert(path.clone(), FileEntry { path, rank });
            }
        }
    }

    db
}

pub fn save_database(db: &HashMap<PathBuf, FileEntry>) -> io::Result<()> {
    let db_dir = dirs::config_dir().unwrap().join("vimoxide");
    let db_path = db_dir.join(DATABASE_FILE);
    let mut file = fs::File::create(db_path)?;

    for entry in db.values() {
        writeln!(file, "{}\t{}", entry.path.display(), entry.rank)?;
    }

    Ok(())
}

pub fn update_database(db: &mut HashMap<PathBuf, FileEntry>, path: &str) {
    let path_buf = PathBuf::from(path);
    let absolute_path = fs::canonicalize(&path_buf).unwrap_or_else(|_| path_buf.clone());

    let entry = db.entry(absolute_path.clone()).or_insert(FileEntry {
        path: absolute_path.clone(),
        rank: 0,
    });
    entry.rank += 1;
}

pub fn find_best_match<'a>(db: &'a HashMap<PathBuf, FileEntry>, query: &'a str) -> Option<String> {
    db.values()
        .filter(|entry| {
            (entry.path.file_stem().map_or(false, |stem| stem.to_string_lossy().contains(query)) ||
            entry.path.file_name().map_or(false, |name| name.to_string_lossy().contains(query))) &&
            entry.path.exists()
        })
        .max_by_key(|entry| entry.rank)
        .map(|entry| entry.path.to_string_lossy().into_owned())
        .or_else(|| Some(query.to_string()))
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Failed to find the config directory")?;
    let config_file_path = config_dir.join("vimoxide").join(CONFIG_FILE);

    // Try to read the configuration file
    let config_file = match std::fs::File::open(&config_file_path) {
        Ok(file) => file,
        Err(_) => {
            return Ok(Config { executor: "vim".to_string() });
        }
    };

    let config: Config = match serde_json::from_reader(config_file) {
        Ok(config) => config,
        Err(_) => {
            return Ok(Config { executor: "vim".to_string() });
        }
    };

    if config.executor != "vim" && config.executor != "nvim" {
        return Ok(Config { executor: "vim".to_string() });
    }

    Ok(config)
}
