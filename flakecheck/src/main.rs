use std::process::{Command, Stdio};

// TODO: nix flake check in later versions of nix cause a memory leak
// because nix sucks, whats new? find a way to run it inside a controlled
// environment where its memory usage is restricted, possibly via arguments
fn main() {
    loop {
        let output = Command::new("nix")
            .arg("flake")
            .arg("check")
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let invalid_files: Vec<&str> = stdout
            .lines()
            .filter(|line| line.contains("is not valid"))
            .map(|line| line.split('\'').nth(1).unwrap())
            .collect();

        if invalid_files.is_empty() {
            break; // No invalid files found, exit loop
        }

        for path in &invalid_files {
            println!("Repairing {}", path);
            let _ = Command::new("sudo")
                .arg("nix-store")
                .arg("--repair-path")
                .arg(path)
                .stdout(Stdio::null())
                .output();
        }
    }
}
