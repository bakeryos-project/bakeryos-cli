use crate::tasks::pkg::uninstall;
use flag_rs::{Command, CommandBuilder, Flag, FlagType, FlagValue};

pub fn init() -> Command {
    CommandBuilder::new("uninstall")
            .short("Unnstall package")
            .long(
                "Uninstalls one or more specified software packages along with their required dependencies. Requires administrator privileges."
            )
            .flag(
                Flag::new("flathub")
                    .usage("Use Flathub as the package source")
                    .value_type(FlagType::Bool)
                    .default(FlagValue::Bool(false))
            )
            .run(|ctx| {
            let arguments = ctx.args();
            if arguments.is_empty() {
                eprintln!("No package specified. Use --help for usage information.");
                return Ok(());
            }

            let mut source_name = "pacman";
            if  ctx.flag_bool_or("flathub", false){
                source_name = "flatpak";
            }

            let result = uninstall(&arguments, source_name);
            match result {
                Ok(_) => {},
                Err(e) => eprintln!("Error: {}", e),
            }

            Ok(())
        })
        .build()
}
