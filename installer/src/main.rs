use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref USER_HOME: PathBuf = PathBuf::from(env::var("HOME").expect("Failed to get HOME"));
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
        "bash" => USER_HOME.join(".bashrc"),
        "zsh" => USER_HOME.join(".zshrc"),
        "fish" => USER_HOME
            .join(".config")
            .join("fish")
            .join("config.fish"),
        _ => USER_HOME.join(".bashrc"),
    };

    path.to_string_lossy().into_owned()
}

fn main() {
    // Build the release version of the project
    let status = std::process::Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("Failed to build the project, please install cargo on root user");

    if !status.success() {
        eprintln!("Error: Failed to build the project");
        std::process::exit(1);
    }

    // Move the vimoxide binary to the bin folder
    let target_dir = env::current_dir().unwrap().join("target").join("release");
    let bin_path = target_dir.join("vimoxide");
    let bin_dest = PathBuf::from("/usr/bin/vimoxide");

    let status = Command::new("sudo")
        .args(["cp", bin_path.to_str().unwrap(), bin_dest.to_str().unwrap()])
        .status()
        .expect("Failed to execute sudo cp");

    if !status.success() {
        eprintln!("Error: Failed to copy the binary to /usr/bin");
        std::process::exit(1);
    }

    use std::io::{self, Write};

    let mut executor = String::new();
    print!("Do you want to use 'nvim' or 'vim'? [nvim]: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut executor) {
        Ok(_) => {
            executor = executor.trim().to_string();
            if executor.is_empty() {
                executor = "nvim".to_string();
            }
        }
        Err(_) => {
            eprintln!("Error: Failed to read input");
            std::process::exit(1);
        }
    };

    // Create the configuration file
    let config_dir = USER_HOME.join(".config").join("vimoxide");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    let config_file = config_dir.join("conf.json");

    let config_data = serde_json::json!({
        "executor": executor
    });

    fs::write(
        &config_file,
        serde_json::to_string_pretty(&config_data).unwrap(),
    )
    .expect("Failed to write config file");

    println!("Configuration file created at: {}\n", config_file.display());

    let shell = get_shell();
    let shell_config = get_shell_config(&shell);
    println!("If you want, add this line to your {} file:", shell_config);
    println!("alias v='vimoxide'\n");
    println!("Installation complete!");
}
