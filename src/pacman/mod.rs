use std::io;
use std::io::Write;
use std::process::{Command, Output, Stdio};
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
        eprintln!("Errore: {}", String::from_utf8_lossy(&output.stderr));
        Err(format!("Errore durante l'installazione: {}", package).into())
    }
}

