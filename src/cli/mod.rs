pub(crate) mod install;
pub(crate) mod remove;
pub(crate) mod search;
pub(crate) mod stats;
pub(crate) mod update;

use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("uwu-h")
        .version("1.2")
        .author("luna")
        .about("AUR Helper written in Rust")
        .subcommand(
            Command::new("search")
                .about("Search for a package")
                .arg(Arg::new("query").help("The search query").required(true)),
        )
        .subcommand(
            Command::new("install").about("Install a package").arg(
                Arg::new("package")
                    .help("The package to install")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("remove").about("Remove a package").arg(
                Arg::new("package")
                    .help("The package to remove")
                    .required(true),
            ),
        )
        .subcommand(Command::new("update").about("Update all packages"))
        .subcommand(Command::new("stats").about("Show statistics about installed packages"))
}
