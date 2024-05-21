use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use vimoxide::constants::HISTORY_FILE;

use crate::utils;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub rank: usize,
}

pub fn load_history() -> HashMap<PathBuf, FileEntry> {
    let mut db = HashMap::new();

    let db_path = utils::get_history_file();
    if !db_path.exists() {
        return db;
    }

    let mut file = fs::File::open(&db_path).expect("Failed to open the history file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the history file");

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

pub fn save_history(db: &HashMap<PathBuf, FileEntry>) -> io::Result<()> {
    let db_dir = dirs::config_dir().unwrap().join("vimoxide");
    let db_path = db_dir.join(HISTORY_FILE);
    let mut file = fs::File::create(db_path)?;

    for entry in db.values() {
        writeln!(file, "{}\t{}", entry.path.display(), entry.rank)?;
    }

    Ok(())
}

pub fn update_history(db: &mut HashMap<PathBuf, FileEntry>, path: &str) {
    let path_buf = PathBuf::from(path);
    let absolute_path = fs::canonicalize(&path_buf).unwrap_or_else(|_| path_buf.clone());

    if absolute_path.exists() {
        let entry = db.entry(absolute_path.clone()).or_insert(FileEntry {
            path: absolute_path.clone(),
            rank: 0,
        });
        entry.rank += 1;
    }
}

/*
Match will follow this specific priority order:
1. Path exists
2. Exact match
3. Substring match
4. Query (as a fallback)
*/
pub fn find_best_match<'a>(db: &'a HashMap<PathBuf, FileEntry>, query: &'a str) -> Option<String> {
    // Check if the query is a path
    let query_path = PathBuf::from(query);
    if query_path.exists() {
        return Some(query.to_string());
    }

    // Check if the query is an exact match
    if let Some(entry) = db.values().find(|entry| {
        entry
            .path
            .file_stem()
            .map_or(false, |stem| stem.to_string_lossy() == query)
    }) {
        return Some(entry.path.to_string_lossy().into_owned());
    }

    // Check if the query is a substring of a path
    db.values()
        .filter(|entry| {
            (entry
                .path
                .file_stem()
                .map_or(false, |stem| stem.to_string_lossy().contains(query))
                || entry
                    .path
                    .file_name()
                    .map_or(false, |name| name.to_string_lossy().contains(query)))
                && entry.path.exists()
        })
        .max_by_key(|entry| entry.rank)
        .map(|entry| entry.path.to_string_lossy().into_owned())
        .or_else(|| Some(query.to_string()))
}
