use flag_rs::{Command, CommandBuilder};

use crate::tasks::system::system_status;

pub fn init() -> Command {
    CommandBuilder::new("status")
        .short("Show system status")
        .long(
            "Reboots the operating system immediately. Requires administrator or root privileges to execute successfully.",
        )
        .run(|_| {
            let result = system_status();
            match result {
                Ok(_) => {},
                Err(e) => eprintln!("Error: {}", e),
            }

            Ok(())
        })
        .build()
}
