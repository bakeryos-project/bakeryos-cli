use crate::helpers::pkg::{install, is_install};

pub fn setup_desktop() -> Result<(), String> {
    let is_instlled = is_install("bakeryos-desktop-setup")?;
    if !is_instlled {
        install(["bakeryos-desktop-setup"], "pacman")?;
    }
    Ok(())
}
