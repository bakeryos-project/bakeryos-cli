use flag_rs::{Command, CommandBuilder};

use crate::tasks::system::update_system;

pub fn init() -> Command {
    CommandBuilder::new("update")
        .short("Check and update all system packages")
        .long("Scans for available updates across all installed system packages and repositories, then automatically downloads and applies them. Requires administrator privileges.")
        .run(|_| {
            let result = update_system();
            match result {
                Ok(_) => println!("System updated successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }

            Ok(())
        })
        .build()
}
