use flag_rs::{Command, CommandBuilder};

use crate::helpers::desktop::setup_desktop;

pub fn init() -> Command {
    CommandBuilder::new("setup")
            .short("Setup desktop")
            .long(
                "Downloads and installs one or more specified software packages along with their required dependencies. Requires administrator privileges."
            )
            .run(|_|{
                let rt = tokio::runtime::Runtime::new()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                rt.block_on(async {
                    if let Err(e) = setup_desktop().await {
                        eprintln!("Error: {}", e);
                    }
                });
                Ok(())
            })
            .build()
}
