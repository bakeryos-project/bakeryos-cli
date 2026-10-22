use std::io::{self, Write};

use crate::helpers::{
    system::update_system,
    update::{
        install_bakeryos_update_gui, is_bakeryos_update_gui_installed, try_bakeryos_update_gui,
    },
};
use flag_rs::{Command, CommandBuilder, Flag, FlagType, FlagValue};

fn open_in_gui() -> Result<(), String> {
    println!("Checking package...");

    match is_bakeryos_update_gui_installed() {
        Ok(true) => {
            println!("BakeryOS Update GUI is installed.");
            let _ = try_bakeryos_update_gui();
        }
        Ok(false) => {
            println!("BakeryOS Update GUI is not installed.");
            print!("Do you want to install it now? [y/N] ");

            if let Err(err) = io::stdout().flush() {
                eprintln!("Failed to flush stdout: {err}");
                return Ok(());
            }

            let mut input = String::new();

            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Failed to read input: {err}");
                return Ok(());
            }

            if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                println!("Installing BakeryOS Update GUI Toolbox...");

                match install_bakeryos_update_gui() {
                    Ok(true) => {
                        println!("Installation completed.");
                        let _ = try_bakeryos_update_gui();
                    }
                    Ok(false) => {
                        println!("Installation failed or was cancelled.");
                    }
                    Err(err) => {
                        eprintln!("Installation error: {err}");
                    }
                }
            } else {
                println!("Installation cancelled.");
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }

    Ok(())
}

pub fn init() -> Command {
    CommandBuilder::new("update")
        .short("Check for BakeryOS package updates")
        .flag(
            Flag::new("gui")
                .short('g')
                .usage("Run in BakeryOS Update GUI app")
                .value_type(FlagType::Bool)
                .default(FlagValue::Bool(false)),
        )
        .run(|ctx| {
            let gui = ctx
                .flag("gui")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(false);
            if gui {
                let _ = open_in_gui();
                Ok(())
            } else {
                let result = update_system();
                match result {
                    Ok(_) => {
                        println!("System updated successfully")
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e)
                    }
                }
                Ok(())
            }
        })
        .build()
}
