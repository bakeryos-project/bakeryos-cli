use flag_rs::{Command, CommandBuilder};

pub fn init() -> Command {
    CommandBuilder::new("bakeryos")
        .short("A CLI for BakeryOS")
        .subcommand(system::init())
        .subcommand(pkg::init())
        .build()
}

mod pkg;
mod pkg_install;
mod pkg_uninstall;
mod system;
mod system_restart;
mod system_shutdown;
mod system_status;
mod system_update;
