
use crate::pacman::install_package;

pub async fn install_command(package: &str) {
    match install_package(package).await {
        Ok(..) => {

        }
        Err(..) => {

        }
    }
}