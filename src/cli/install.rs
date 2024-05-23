use crate::pacman::install_package;

pub fn install_command(package: &str) {
    match install_package(package) {
        Ok(output) => {
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        }
        Err(err) => {
            eprintln!("Error installing package: {}", err);
        }
    }
}