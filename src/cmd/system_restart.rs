use flag_rs::{Command, CommandBuilder};

use crate::tasks::system::restart;

pub fn init() -> Command {
    CommandBuilder::new("restart")
        .short("Restart the system")
        .long(
            "Reboots the operating system immediately. Requires administrator or root privileges to execute successfully.",
        )
        .run(|_| {
            let result = restart();
            match result {
                Ok(_) => println!("System restarted successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }

            Ok(())
        })
        .build()
}
