use std::error::Error;
use std::process::Command;
use ansi_term::Color;
use colored::Colorize;
use crate::ascii_art;

pub(crate) async fn stats_command() {
    let pacman = "pacman";
    let info = collect(pacman).await.unwrap();
    println!();
    println!("Packages totali: {}", Color::Blue.paint(info.total_packages.to_string()));
    println!();
    let total_size_gib = info.total_size as f64 / (1024.0 * 1024.0 * 1024.0);
    println!("Dimensioni totali: {} {}", Color::Blue.paint(total_size_gib.to_string()), Color::Red.paint("GiB".to_string()));
    println!();
    println!("Top 10 packages:");
    for (size, name) in info.max_packages.iter().take(10) {
        let size_gib = *size as f64 / (1024.0 * 1024.0 * 1024.0);
        println!();
        println!("--- {}{} {} {}",
                 Color::Green.paint(name.to_string()),
                 Color::Red.paint(":".to_string()),
                 Color::Blue.paint(size_gib.to_string()),
                 Color::Red.paint("GiB".to_string())
        );
    }

    let ascii_art = ascii_art::uwu();
    let colors = vec!["red", "yellow", "green", "cyan", "blue", "magenta"];
    for (i, line) in ascii_art.lines().enumerate() {
        let color = colors[i % colors.len()];
        println!("{}", line.color(color));
    }
}

struct Info {
    total_packages: usize,
    total_size: i64,
    max_packages: Vec<(i64, String)>,
}

async fn collect(pacman: &str) -> Result<Info, Box<dyn Error>> {
    let output = Command::new(pacman)
        .arg("-Q")
        .arg("-i")
        .arg("-v")
        .output()?;

    let output = String::from_utf8(output.stdout)?;
    let mut total_packages = 0;
    let mut total_size = 0;
    let mut max_packages = Vec::new();
    let mut package_name = String::new();

    for line in output.lines() {
        if line.starts_with("Nome") {
            package_name = line.split(':').last().unwrap().trim().to_string();
            total_packages += 1;
        }
        if line.starts_with("Spazio richiesto") {
            let size_str = match line.split(':').last() {
                Some(size_str) if !size_str.trim().is_empty() => size_str.trim(),
                _ => {
                    eprintln!("Error: Invalid line: {}", line);
                    continue;
                }
            };
            let parts: Vec<&str> = size_str.split_whitespace().collect();
            if parts.len() != 2 {
                eprintln!("Error: Invalid size string: {}", size_str);
                continue;
            }
            let (num_str, suffix) = (parts[0], parts[1]);
            let num_str = num_str.replace(',', ".");
            let num = match num_str.parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error: Invalid size string: {}", size_str);
                    continue;
                }
            };
            let size = match suffix {
                "B" => num.round() as i64,
                "KiB" => (num * 1024.0).round() as i64,
                "MiB" => (num * 1024.0 * 1024.0).round() as i64,
                "GiB" => (num * 1024.0 * 1024.0 * 1024.0).round() as i64,
                _ => {
                    eprintln!("Error: Invalid size string: {}", size_str);
                    continue;
                }
            };
            total_size += size;
            max_packages.push((size, package_name.clone()));
        }
    }

    max_packages.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(Info {
        total_packages,
        total_size,
        max_packages,
    })
}