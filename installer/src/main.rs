use std::env;
use std::fs;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SUDO_USER: String = env::var("SUDO_USER").expect("Failed to get SUDO_USER");
    static ref SUDO_USER_HOME: PathBuf = PathBuf::from(format!("/home/{}", *SUDO_USER));
}

fn check_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn get_shell() -> String {
    // Get the user's shell
    let shell = env::var("SHELL").expect("Failed to get SHELL");

    if shell.contains("bash") {
        "bash".to_string()
    } else if shell.contains("zsh") {
        "zsh".to_string()
    } else if shell.contains("fish") {
        "fish".to_string()
    } else {
        shell
    }
}

fn get_shell_config(shell: &str) -> String {
    let path = match shell {
        "bash" => SUDO_USER_HOME.join(".bashrc"),
        "zsh" => SUDO_USER_HOME.join(".zshrc"),
        "fish" => SUDO_USER_HOME.join(".config").join("fish").join("config.fish"),
        _ => SUDO_USER_HOME.join(".bashrc"),
    };

    path.to_string_lossy().into_owned()
}

fn main() {
    if !check_root() {
        eprintln!("Error: This script must be run as root, retry with sudo");
        std::process::exit(1);
    }

    // Build the release version of the project
    let status = std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .expect("Failed to build the project");

    if !status.success() {
        eprintln!("Error: Failed to build the project");
        std::process::exit(1);
    }

    // Move the vimoxide binary to the bin folder
    let target_dir = env::current_dir().unwrap().join("target").join("release");
    let bin_path = target_dir.join("vimoxide");
    let bin_dest = PathBuf::from("/usr/bin/vimoxide");

    if let Err(err) = fs::copy(&bin_path, &bin_dest) {
        eprintln!("Error: Failed to copy binary: {}", err);
        std::process::exit(1);
    }

    use std::io::{self, Write};

    let mut executor = String::new();
    print!("Do you want to use 'vim' or 'nvim'? [vim]: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut executor) {
        Ok(_) => {
            executor = executor.trim().to_string();
            if executor.is_empty() {
                executor = "vim".to_string();
            }
        },
        Err(_) => {
            eprintln!("Error: Failed to read input");
            std::process::exit(1);
        }
    };

    // Create the configuration file
    let config_dir = SUDO_USER_HOME.join(".config").join("vimoxide");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    let config_file = config_dir.join("conf.json");

    let config_data = serde_json::json!({
        "executor": executor
    });

    fs::write(&config_file, serde_json::to_string_pretty(&config_data).unwrap())
        .expect("Failed to write config file");

    println!("Configuration file created at: {}\n", config_file.display());

    let shell = get_shell();
    let shell_config = get_shell_config(&shell);
    println!("If you want, add this line to your {} file:", shell_config);
    println!("alias v='vimoxide'\n");
    println!("Installation complete!");
}
