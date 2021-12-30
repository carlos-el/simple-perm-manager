use std::process::Command;

// Syntax checks
fn main() {
    // Format code using rustfmt
    assert!(Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("task check failed")
        .success());

    // Find improvable syntax with clippy
    assert!(Command::new("cargo")
        .arg("clippy")
        .status()
        .expect("task check failed")
        .success());
}
