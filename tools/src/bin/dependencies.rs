use std::process::Command;

// Install dependencies. If anything fails it should stop
fn main() {
    // Install clippy and rustfmt using rustup
    assert!(Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg("clippy")
        .arg("rustfmt")
        .status()
        .expect("task (install) dependencies failed in 'rustup' command")
        .success());

    // Install tarpaulin (coverage tool) using cargo (Make sure libssl-dev is installed before)
    assert!(Command::new("cargo")
        .arg("install")
        .arg("cargo-tarpaulin")
        .status()
        .expect("task (install) dependencies failed in 'cargo' command")
        .success());
}
