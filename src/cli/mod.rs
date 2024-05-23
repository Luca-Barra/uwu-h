pub(crate) mod install;
pub(crate) mod search;

use clap::{Arg, Command};

pub fn build_cli() -> Command<'static,> {
    Command::new("uwu")
        .version("1.0")
        .author("luna")
        .about("AUR Helper written in Rust")
        .subcommand(
            Command::new("search")
                .about("Search for a package")
                .arg(
                    Arg::new("query")
                        .help("The search query")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(
                    Arg::new("package")
                        .help("The package to install")
                        .required(true),
                ),
        )
}
