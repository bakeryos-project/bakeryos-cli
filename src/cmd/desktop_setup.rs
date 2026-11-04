use flag_rs::{Command, CommandBuilder};

use crate::{helpers::desktop::setup_desktop, utils::style::green};

pub fn init() -> Command {
    CommandBuilder::new("setup")
            .short("Setup desktop")
            .long(
                "Downloads and installs one or more specified software packages along with their required dependencies. Requires administrator privileges."
            )
            .run(|_|{
                if let Err(e) = setup_desktop() {
                    eprintln!("Error: {}", e);
                }else{
                     print!("{} \n", green("Success"));
                }

                Ok(())
            })
            .build()
}
