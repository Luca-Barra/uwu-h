use std::process::{Command, Stdio};
use ansi_term::Color;
use crate::ascii_art;

pub(crate) async fn update_command() {
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-Syu")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("{}", Color::Blue.paint("Aggiornamento completato"));
                println!("{}", Color::Green.paint(ascii_art::uwu()));
            } else {
                eprintln!("Error updating system");
                ascii_art::notuwu();
            }
        },
        Err(e) => {
            eprintln!("Error updating system: {}", e);
        }
    }
}