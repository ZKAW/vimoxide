use clap::{Arg, Command};
use std::env;
use std::path::PathBuf;

mod file_handling;
mod history;
mod utils;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SUDO_USER: String = env::var("SUDO_USER").expect("Failed to get SUDO_USER");
    static ref SUDO_USER_HOME: PathBuf = PathBuf::from(format!("/home/{}", *SUDO_USER));
}

fn main() {
    let matches = Command::new("vimoxide")
        .version("0.1.0")
        .author("Author: ZKAW (https://github.com/ZKAW)")
        .about("A tool to quickly open files with Vim / Nvim based on frequency of access")
        .arg(
            Arg::new("file")
                .help("The file to open with Vim / Nvim")
                .required(false)
                .index(1),
        )
        .get_matches();

    let config = utils::load_config_file().expect("Failed to load the configuration file");

    if let Some(file) = matches.get_one::<String>("file") {
        let mut db = history::load_history();
        let best_match_path = history::find_best_match(&db, file);
        if let Some(ref path) = best_match_path {
            file_handling::open_with_executor(path, &config.executor);
            history::update_history(&mut db, path);
            history::save_history(&db).expect("Failed to save the history");
        }
    } else {
        file_handling::open_with_executor("", &config.executor); // No arg provided
    }
}
