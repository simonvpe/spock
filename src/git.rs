use std::process::{Command, ExitStatus};

pub fn check_executable() -> ExitStatus {
    Command::new("git")
        .args(&["--version"])
        .status()
        .expect("Could not find git")
}

pub fn check_git_repository(dir: &str) -> ExitStatus {
    Command::new("git")
        .args(&["-C", dir, "rev-parse", "--git-dir"])
        .status()
        .expect("Not a valid git repository")
}

pub fn init(dir: &str) -> ExitStatus {
    Command::new("git")
        .args(&["-C", dir, "init"])
        .status()
        .expect("Failed to initialize repository")
}
