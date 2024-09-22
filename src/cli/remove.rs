use crate::pacman::remove_package;

pub fn remove_command(package: &str) {
    if remove_package(package).is_ok() {}
}
