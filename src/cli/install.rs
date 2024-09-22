use crate::pacman::install_package;

pub async fn install_command(package: &str) {
    if (install_package(package).await).is_ok() {}
}
