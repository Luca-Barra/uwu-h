use crate::pacman::remove_package;

pub fn remove_command(package: &str) {
    match remove_package(package) {
        Ok(..) => {

        }
        Err(..) => {

        }
    }
}