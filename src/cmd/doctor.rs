use crate::{cmd::device_scan, helpers::diagnostic::diagnostic};
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("doctor")
        .subcommands(vec![device_scan::init()])
        .short("Run diagnostics and detect issues with BakeryOS")
        .run(|_| {
            match diagnostic() {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:}", e)
                }
            };
            Ok(())
        })
        .build()
}
