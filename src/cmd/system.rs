use crate::cmd::{system_restart, system_shutdown, system_status, system_update};
use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("system")
        .short("System commands")
        .subcommands(vec![
            system_update::init(),
            system_shutdown::init(),
            system_restart::init(),
            system_status::init(),
        ])
        .build()
}
