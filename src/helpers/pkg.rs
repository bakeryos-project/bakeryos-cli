use std::ffi::OsStr;

use crate::utils::command::create_command;

fn install_pkg_via_pacman<T, P>(packages: P) -> Result<(), String>
where
    T: AsRef<OsStr>,
    P: AsRef<[T]>,
{
    let mut cmd = create_command("sudo", ["pacman", "-S", "--noconfirm"]);
    for package in packages.as_ref() {
        cmd.arg(package);
    }
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

fn install_pkg_via_flatpak<T, P>(packages: P) -> Result<(), String>
where
    T: AsRef<OsStr>,
    P: AsRef<[T]>,
{
    let mut cmd = create_command("flatpak", ["install", "flathub", "--assumeyes"]);
    for package in packages.as_ref() {
        cmd.arg(package);
    }
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn install<T, S, P>(packages: P, source: S) -> Result<(), String>
where
    P: AsRef<[T]>,
    T: AsRef<OsStr>,
    S: AsRef<str>,
{
    match source.as_ref() {
        "pacman" => install_pkg_via_pacman(packages),
        "flatpak" => install_pkg_via_flatpak(packages),
        _ => Err("Invalid source".to_string()),
    }
}

fn uninstall_via_pacman<T, P>(packages: P) -> Result<(), String>
where
    P: AsRef<[T]>,
    T: AsRef<OsStr>,
{
    let mut cmd = create_command("sudo", ["pacman", "-R", "--noconfirm"]);
    for package in packages.as_ref() {
        cmd.arg(package);
    }
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

fn uninstall_via_flatpak<T, P>(packages: P) -> Result<(), String>
where
    P: AsRef<[T]>,
    T: AsRef<OsStr>,
{
    let mut cmd = create_command("flatpak", ["uninstall", "--assumeyes"]);
    for package in packages.as_ref() {
        cmd.arg(package);
    }
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn uninstall<T, P, S>(packages: P, source: S) -> Result<(), String>
where
    P: AsRef<[T]>,
    T: AsRef<OsStr>,
    S: AsRef<str>,
{
    match source.as_ref() {
        "pacman" => uninstall_via_pacman(packages),
        "flatpak" => uninstall_via_flatpak(packages),
        _ => Err("Invalid source".to_string()),
    }
}
