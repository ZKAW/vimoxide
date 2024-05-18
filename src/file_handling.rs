use std::process::Command as ProcessCommand;

pub fn open_with_nvim(path: &str) {
    ProcessCommand::new("nvim")
        .arg(path)
        .status()
        .expect("Failed to open file with neovim");
}

pub fn open_with_vim(path: &str) {
    ProcessCommand::new("vim")
        .arg(path)
        .status()
        .expect("Failed to open file with vim");
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
