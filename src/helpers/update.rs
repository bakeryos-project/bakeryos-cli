use std::{env, process::Command};

pub fn is_bakeryos_update_gui_installed() -> Result<bool, String> {
    let Some(path) = env::var_os("PATH") else {
        return Ok(false);
    };

    for dir in env::split_paths(&path) {
        let binary = dir.join("bakeryos-update");

        if binary.is_file() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn install_bakeryos_update_gui() -> Result<bool, String> {
    let status = Command::new("sudo")
        .args(["pacman", "-S", "--needed", "bakeryos-update"])
        .status()
        .map_err(|e| format!("Failed to run pacman: {e}"))?;

    Ok(status.success())
}

pub fn try_bakeryos_update_gui() -> Result<bool, String> {
    let status = Command::new("bakeryos-update")
        .status()
        .map_err(|e| format!("Failed to run pacman: {e}"))?;

    Ok(status.success())
}
