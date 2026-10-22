use std::env;
use std::process::Command;

pub fn is_dev_toolbox_installed() -> Result<bool, String> {
    let Some(path) = env::var_os("PATH") else {
        return Ok(false);
    };

    for dir in env::split_paths(&path) {
        let binary = dir.join("bakeryos-dev-toolbox");

        if binary.is_file() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn install_dev_toolbox() -> Result<bool, String> {
    let status = Command::new("sudo")
        .args(["pacman", "-S", "--needed", "bakeryos-dev-toolbox"])
        .status()
        .map_err(|e| format!("Failed to run pacman: {e}"))?;

    Ok(status.success())
}

pub fn try_open_dev_toolbox() -> Result<bool, String> {
    let status = Command::new("bakeryos-dev-toolbox")
        .status()
        .map_err(|e| format!("Failed to run pacman: {e}"))?;

    Ok(status.success())
}
