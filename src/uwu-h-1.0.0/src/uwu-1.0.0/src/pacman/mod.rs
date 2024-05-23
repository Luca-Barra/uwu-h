
use std::process::{Command, Stdio};
use ansi_term::Color;

pub fn is_pacman_available() -> bool {
    Command::new("pacman")
        .arg("--version")
        .output()
        .is_ok()
}

pub fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {

    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg(package)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        println!("Package {} installato con successo.", Color::Blue.bold().paint(package));
        Ok(())
    } else {
        Err(format!("Errore durante l'installazione: {}", package).into())
    }
}

pub fn remove_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-R")
        .arg(package)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        println!("Package {} rimosso con successo.", Color::Blue.bold().paint(package));
        Ok(())
    } else {
        Err(format!("Errore durante la rimozione: {}", package).into())
    }
}

