use std::io::{self, Write};

use crate::{
    helpers::dev_toolbox::{install_dev_toolbox, is_dev_toolbox_installed},
    utils::style::{bold, cyan, fail_badge, info_badge, pass_badge, warn_badge, yellow},
};
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("check")
        .short("Check BakeryOS Dev Toolbox status")
        .run(|_ctx| {
            println!("{} Checking BakeryOS Dev Toolbox status...", info_badge());

            match is_dev_toolbox_installed() {
                Ok(true) => {
                    println!("{} BakeryOS Dev Toolbox is installed.", pass_badge());
                }
                Ok(false) => {
                    println!("{} BakeryOS Dev Toolbox is not installed.", warn_badge());
                    print!("{} Do you want to install it now? [y/N]: ", bold(yellow("?")));

                    if let Err(err) = io::stdout().flush() {
                        eprintln!("{} Failed to flush stdout: {err}", fail_badge());
                        return Ok(());
                    }

                    let mut input = String::new();

                    if let Err(err) = io::stdin().read_line(&mut input) {
                        eprintln!("{} Failed to read input: {err}", fail_badge());
                        return Ok(());
                    }

                    if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                        println!("{} Installing BakeryOS Dev Toolbox...", info_badge());

                        match install_dev_toolbox() {
                            Ok(true) => {
                                println!("{} Installation completed successfully.", pass_badge());
                            }
                            Ok(false) => {
                                println!("{} Installation failed or was cancelled.", fail_badge());
                            }
                            Err(err) => {
                                eprintln!("{} Installation error: {err}", fail_badge());
                            }
                        }
                    } else {
                        println!("{} Installation cancelled.", cyan("[INFO]"));
                    }
                }
                Err(err) => {
                    eprintln!("{} Error checking Dev Toolbox: {err}", fail_badge());
                }
            }

            Ok(())
        })
        .build()
}
