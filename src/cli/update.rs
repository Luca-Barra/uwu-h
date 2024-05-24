use std::process::{Command, Stdio};
use colored::*;
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
                println!("{}", "Aggiornamento completato".blue());
                let ascii_art = ascii_art::uwu();
                let colors = vec!["red", "yellow", "green", "cyan", "blue", "magenta"];
                for (i, line) in ascii_art.lines().enumerate() {
                    let color = colors[i % colors.len()];
                    println!("{}", line.color(color));
                }
            }
            else {
                eprintln!("Error updating system");
                ascii_art::notuwu();
            }
        },
        Err(e) => {
            eprintln!("Error updating system: {}", e);
        }
    }
}