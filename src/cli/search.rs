use crate::aur::search_aur;

use ansi_term::Color;

pub fn search_command(query: &str) {
    match search_aur(query) {
        Ok(response) => {
            for package in response.results {
                println!("{} - {}", Color::Blue.bold().paint(package.Name), Color::Green.paint(package.Version));
                if let Some(desc) = package.Description {
                    println!();
                    println!("{}", Color::Red.paint("-----------------------------------------"));
                    println!();
                    println!("  Description: {}", Color::Yellow.italic().paint(desc));
                    println!();
                    println!("{}", Color::Red.paint("******************************************"));
                    println!();
                }
            }
        }
        Err(err) => {
            eprintln!("Error searching AUR: {}", err);
        }
    }
}
