use std::process::{Command, exit};

pub fn show_diff(dir1: &str, dir2: &str) {
    let output = Command::new("sudo")
        .arg("diff")
        .arg("-c")
        .arg("-r")
        .arg(dir1)
        .arg(dir2)
        .output()
        .expect("Failed to execute diff command");

    let diff = String::from_utf8_lossy(&output.stdout);
    if !diff.is_empty() {
        println!("{}", diff);
    } else if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: Could not compute diff: {}", error_message);
        exit(1);
    }
}