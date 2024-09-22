use crate::ascii_art;
use crate::aur::search_aur;
use crate::diff::show_diff;
use ansi_term::Color;
use colored::Colorize;
use dialoguer::Confirm;
use dirs::home_dir;
use fs_extra::dir::{copy, CopyOptions};
use std::fs;
use std::process::{Command, Stdio};

pub fn is_pacman_available() -> bool {
    Command::new("pacman").arg("--version").output().is_ok()
}

pub async fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
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
                println!(
                    "Package {} installato con successo",
                    Color::Blue.bold().paint(package)
                );
                Ok(())
            } else {
                let aur_response = search_aur(package).await?;
                if aur_response.results.is_empty() {
                    println!("{}", Color::Red.paint(ascii_art::notuwu()));
                    return Err(format!("Package {} non trovato", package).into());
                }

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
                        Ok(show_diff(
                            &cache_directory.to_string_lossy(),
                            &build_dir.to_string_lossy(),
                        ))
                    } else {
                        copy(&build_dir, &cache_directory, &options)?;
                        println!(
                            "Package {} già installato, aggiornato con successo",
                            Color::Blue.bold().paint(package)
                        );
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
                        println!("{}", Color::Red.paint(ascii_art::notuwu()));
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
                        println!(
                            "Package {} installato con successo",
                            Color::Blue.bold().paint(package)
                        );

                        let ascii_art = ascii_art::uwu();
                        let colors = ["red", "yellow", "green", "cyan", "blue", "magenta"];
                        for (i, line) in ascii_art.lines().enumerate() {
                            let color = colors[i % colors.len()];
                            println!("{}", line.color(color));
                        }

                        let cache_directory = uwu_directory.join(".cache").join(package_name);
                        fs::create_dir_all(&cache_directory)?;
                        let mut options = CopyOptions::new();
                        options.overwrite = true;

                        if show_diffs {
                            copy(&build_dir, &cache_directory, &options)?;
                            Ok(show_diff(
                                &cache_directory.to_string_lossy(),
                                &build_dir.to_string_lossy(),
                            ))
                        } else {
                            copy(&build_dir, &cache_directory, &options)?;
                            println!(
                                "Package {} già installato, aggiornato con successo",
                                Color::Blue.bold().paint(package)
                            );
                            Ok(())
                        }
                    } else {
                        println!("{}", Color::Red.paint(ascii_art::notuwu()));
                        Err(format!(
                            "Error building/installing AUR package: {}",
                            Color::Red.paint(package)
                        )
                        .into())
                    }
                }
            }
        }
        Err(_) => {
            println!("{}", Color::Red.paint(ascii_art::notuwu()));
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
        println!(
            "Package {} rimosso con successo.",
            Color::Blue.bold().paint(package)
        );

        let ascii_art = ascii_art::uwu();
        let colors = ["red", "yellow", "green", "cyan", "blue", "magenta"];
        for (i, line) in ascii_art.lines().enumerate() {
            let color = colors[i % colors.len()];
            println!("{}", line.color(color));
        }

        Ok(())
    } else {
        println!("{}", Color::Red.paint(ascii_art::notuwu()));
        Err(format!("Errore durante la rimozione: {}", package).into())
    }
}
