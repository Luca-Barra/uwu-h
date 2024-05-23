use std::fs;
use std::process::{Command, Stdio};
use ansi_term::Color;
use dirs::home_dir;
use crate::ascii_art;

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
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Package {} installato con successo", Color::Blue.bold().paint(package));
                Ok(())
            } else {

                let home_directory = home_dir().unwrap();
                let uwu_directory = home_directory.join(".uwu");
                fs::create_dir_all(&uwu_directory)?;

                let output = Command::new("git")
                    .arg("clone")
                    .arg(format!("https://aur.archlinux.org/{}.git", package))
                    .current_dir(&uwu_directory)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()?;

                if !output.status.success() {
                    println!("{}", ascii_art::notuwu());
                    return Err(format!("Error cloning AUR package: {}", package).into());
                }

                let package_name = package.split('/').last().unwrap_or(package);
                let build_dir = uwu_directory.join(package_name);

                let output = Command::new("makepkg")
                    .arg("-si")
                    .current_dir(&build_dir)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()?;

                if output.status.success() {
                    println!("Package {} installato con successo", Color::Blue.bold().paint(package));
                    println!("{}", ascii_art::uwu());

                    Ok(())
                } else {
                    println!("{}", ascii_art::notuwu());
                    Err(format!("Error building/installing AUR package: {}", Color::Red.paint(package)).into())
                }
            }
        },
        Err(_) => {
            println!("{}", ascii_art::notuwu());
            Err(format!("Failed to run command for package: {}", package).into())
        }
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
        let home_directory = home_dir().unwrap();
        let uwu_directory = home_directory.join(".uwu");
        let package_directory = uwu_directory.join(package);

        if package_directory.exists() {
            fs::remove_dir_all(package_directory)?;
        }
        println!("Package {} rimosso con successo.", Color::Blue.bold().paint(package));
        println!("{}", ascii_art::uwu());
        Ok(())
    } else {
        println!("{}", ascii_art::notuwu());
        Err(format!("Errore durante la rimozione: {}", package).into())
    }
}

