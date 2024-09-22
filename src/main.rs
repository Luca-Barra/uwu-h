mod ascii_art;
mod aur;
mod cache;
mod cli;
mod diff;
mod pacman;

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
            cli::search::search_command(query).await;
        }
        Some(("install", sub_m)) => {
            let package = sub_m.get_one::<String>("package").unwrap();
            cli::install::install_command(package).await;
        }
        Some(("remove", sub_m)) => {
            let package = sub_m.get_one::<String>("package").unwrap();
            cli::remove::remove_command(package);
        }
        Some(("update", _)) => {
            cli::update::update_command().await;
        }
        Some(("stats", _)) => {
            cli::stats::stats_command().await;
        }
        _ => {
            println!("Unknown command");
        }
    }
}
