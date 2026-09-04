use flag_rs::{Command, CommandBuilder};

use crate::tasks::system::shutdown;

pub fn init() -> Command {
    CommandBuilder::new("shutdown")
        .short("Shutdown the system")
        .long(
            "Shuts down the operating system immediately. Requires administrator or root privileges to execute successfully.",
        )
        .run(|_| {
            let result = shutdown();
            match result {
                Ok(_) => println!("System shutdown successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }

            Ok(())
        })
        .build()
}
