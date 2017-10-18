use std::process::{Command, ExitStatus, Stdio};
use std::path::Path;

pub fn check_executable() -> ExitStatus {
    Command::new("git")
        .args(&["--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())        
        .status()
        .expect("Could not find git")
}

pub fn check_git_repository(dir: &str) -> ExitStatus {
    Command::new("git")
        .args(&["-C", dir, "rev-parse", "--git-dir"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Not a valid git repository")
}

pub fn init(dir: &str) -> ExitStatus {
    Command::new("git")
        .args(&["-C", dir, "init"])
        .status()
        .expect("Failed to initialize repository")
}

pub fn submodule_add(dir: &str, submodule: &str, suffix: &str) -> ExitStatus {
    let dst = Path::new(dir).join(suffix);
    Command::new("git")
        .args(&["-C", dir, "submodule", "add", submodule, dst.to_str().unwrap()])
        .status()
        .expect("Failed to add submodule")
}
