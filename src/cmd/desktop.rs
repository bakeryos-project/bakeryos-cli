use crate::cmd::desktop_setup;
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("desktop")
        .subcommands(vec![desktop_setup::init()])
        .short("Desktop commands")
        .build()
}
