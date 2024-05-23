mod cli;
mod aur;
mod pacman;
mod cache;

use cli::build_cli;

#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();

    if !pacman::is_pacman_available() {
        eprintln!("pacman is not available on this system.");
        return;
    }

    match matches.subcommand() {
        Some(("search", sub_m)) => {
            let query = sub_m.get_one::<String>("query").unwrap();
            cli::search::search_command(query);
        },
        Some(("install", sub_m)) => {
            let package = sub_m.get_one::<String>("package").unwrap();
            cli::install::install_command(package).await;
        },
        Some(("remove", sub_m)) => {
            let package = sub_m.get_one::<String>("package").unwrap();
            cli::remove::remove_command(package);
        },
        _ => {
            println!("Unknown command");
        }
    }
}
