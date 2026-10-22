use std::io::{self, Write};

use crate::helpers::dev_toolbox::{
    install_dev_toolbox, is_dev_toolbox_installed, try_open_dev_toolbox,
};
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("open")
        .short("Open BakeryOS Dev Toolbox")
        .run(|_ctx| {
            println!("Checking package...");

            match is_dev_toolbox_installed() {
                Ok(true) => {
                    println!("BakeryOS Dev Toolbox is installed.");
                    let _ = try_open_dev_toolbox();
                }
                Ok(false) => {
                    println!("BakeryOS Dev Toolbox is not installed.");
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
                        println!("Installing BakeryOS Dev Toolbox...");

                        match install_dev_toolbox() {
                            Ok(true) => {
                                println!("Installation completed.");
                                let _ = try_open_dev_toolbox();
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
                    eprintln!("Error checking Dev Toolbox: {err}");
                }
            }

            Ok(())
        })
        .build()
}
