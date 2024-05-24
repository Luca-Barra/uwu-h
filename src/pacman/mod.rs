use std::fs;
use std::process::{Command, Stdio};
use ansi_term::Color;
use dirs::home_dir;
use fs_extra::dir::{copy, CopyOptions};
use crate::ascii_art;
use crate::diff::show_diff;
use dialoguer::Confirm;

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

                let show_diffs = Confirm::new()
                    .with_prompt("Vuoi mostrare le differenze?")
                    .interact()
                    .unwrap_or(false);

                let home_directory = home_dir().unwrap();
                let uwu_directory = home_directory.join(".uwu");
                fs::create_dir_all(&uwu_directory)?;
                let package_name = package.split('/').last().unwrap_or(package);
                let build_dir = uwu_directory.join(package_name);

                if build_dir.exists() {
                    let output = Command::new("git")
                        .arg("pull")
                        .current_dir(&build_dir)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output()?;

                    if !output.status.success() {
                        println!("{}", ascii_art::notuwu());
                        return Err(format!("Error updating AUR package: {}", package).into());
                    }

                    let cache_directory = uwu_directory.join(".cache").join(package_name);
                    fs::create_dir_all(&cache_directory)?;
                    let mut options = CopyOptions::new();
                    options.overwrite = true;

                    ascii_art::uwu();

                    if show_diffs {
                        copy(&build_dir, &cache_directory, &options)?;
                        Ok(show_diff(&cache_directory.to_string_lossy(), &build_dir.to_string_lossy()))
                    } else {
                        copy(&build_dir, &cache_directory, &options)?;
                        println!("Package {} già installato, aggiornato con successo", Color::Blue.bold().paint(package));
                        Ok(())
                    }
                } else {
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
                        let cache_directory = uwu_directory.join(".cache").join(package_name);
                        fs::create_dir_all(&cache_directory)?;
                        let mut options = CopyOptions::new();
                        options.overwrite = true;

                        ascii_art::uwu();

                        if show_diffs {
                            copy(&build_dir, &cache_directory, &options)?;
                            Ok(show_diff(&cache_directory.to_string_lossy(), &build_dir.to_string_lossy()))
                        } else {
                            copy(&build_dir, &cache_directory, &options)?;
                            println!("Package {} già installato, aggiornato con successo", Color::Blue.bold().paint(package));
                            Ok(())
                        }
                    } else {
                        println!("{}", ascii_art::notuwu());
                        Err(format!("Error building/installing AUR package: {}", Color::Red.paint(package)).into())
                    }
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

