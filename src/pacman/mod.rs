use std::process::{Command, Output};

pub fn is_pacman_available() -> bool {
    Command::new("pacman")
        .arg("--version")
        .output()
        .is_ok()
}

pub fn install_package(package: &str) -> Result<Output, Box<dyn std::error::Error>> {
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg(package)
        .output()?;
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!("Failed to install package: {}", package).into())
    }
}
