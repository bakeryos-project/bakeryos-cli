use crate::helpers::dev_toolbox::install_dev_toolbox;
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("install")
        .short("Install BakeryOS Dev Toolbox")
        .run(|_ctx| {
            println!("Installing BakeryOS Dev Toolbox...");

            match install_dev_toolbox() {
                Ok(true) => {
                    println!("Installation completed.");
                }
                Ok(false) => {
                    println!("Installation failed or was cancelled.");
                }
                Err(err) => {
                    eprintln!("Installation error: {err}");
                }
            }

            Ok(())
        })
        .build()
}
