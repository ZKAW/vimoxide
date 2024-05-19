use std::process::Command as ProcessCommand;

pub fn open_with_nvim(path: &str) {
    let mut command = ProcessCommand::new("nvim");

    if !path.is_empty() {
        command.arg(path);
    }

    command.status().expect("Failed to open file with neovim");
}

pub fn open_with_vim(path: &str) {
    let mut command = ProcessCommand::new("vim");

    if !path.is_empty() {
        command.arg(path);
    }

    command.status().expect("Failed to open file with vim");
}

pub fn open_with_executor(path: &str, executor: &str) {
    match executor {
        "vim" => open_with_vim(path),
        "nvim" => open_with_nvim(path),
        _ => {
            eprintln!("Unknown executor specified in config, defaulting to vim");
            open_with_vim(path);
        }
    }
}
