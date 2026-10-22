use crate::cmd::device_scan;
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("device")
        .subcommands(vec![device_scan::init()])
        .short("Device commands")
        .build()
}
