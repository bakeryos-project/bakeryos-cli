use flag_rs::{Command, CommandBuilder};

use crate::cmd::{pkg_install, pkg_uninstall};

pub fn init() -> Command {
    CommandBuilder::new("pkg")
        .subcommands(vec![pkg_install::init(), pkg_uninstall::init()])
        .short("Package commands")
        .build()
}
