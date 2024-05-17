use clap::{App, Arg};
use std::path::PathBuf;

mod database;
mod file_handling;

fn main() {
    let matches = App::new("vimoxide")
        .version("0.1.0")
        .author("Author: ZKAW (https://github.com/ZKAW)")
        .about("A tool to quickly open files with Vim / Nvim based on frequency of access")
        .arg(Arg::new("file")
            .help("The file to open with Vim / Nvim")
            .required(true)
            .index(1))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let config = database::load_config().unwrap_or_else(|err| {
        eprintln!("Failed to load config: {}", err);
        std::process::exit(1);
    });

    let mut db = database::load_database();
    let path = PathBuf::from(file);

    if path.exists() {
        file_handling::open_with_executor(&path, &config.executor);
        database::update_database(&mut db, &path);
        database::save_database(&db).expect("Failed to save the database");
    } else {
        let mut best_match_path = None;
        if let Some(best_match) = database::find_best_match(&db, file) {
            best_match_path = Some(best_match.path.clone());
            file_handling::open_with_executor(&best_match.path, &config.executor);
        }
        if let Some(path) = best_match_path {
            database::update_database(&mut db, &path);
            database::save_database(&db).expect("Failed to save the database");
        } else {
            eprintln!("No matching file found for: {}", file);
        }
    }
}
