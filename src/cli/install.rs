use crate::pacman::install_package;

pub fn install_command(package: &str) {
    match install_package(package) {
        Ok(output) => {

        }
        Err(err) => {
            eprintln!("Error installing package: {}", err);
        }
    }
}