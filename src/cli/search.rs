use crate::aur::search_aur;

use ansi_term::Color;

pub fn search_command(query: &str) {
    match search_aur(query) {
        Ok(response) => {
            for package in response.results {
                println!("{} - {}", Color::Blue.paint(package.Name), Color::Green.paint(package.Version));
                if let Some(desc) = package.Description {
                    println!("{}", Color::Red.paint("-----------------------------------------"));
                    println!("  Description: {}", Color::Yellow.paint(desc));
                    println!("{}", Color::Red.paint("-----------------------------------------"));
                }
            }
        }
        Err(err) => {
            eprintln!("Error searching AUR: {}", err);
        }
    }
}
