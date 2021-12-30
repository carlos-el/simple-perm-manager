use std::process::Command;

// Runs test and collects its coverage
fn main() {
    // Run test and coverage extraction with tarpaulin crates package
    assert!(Command::new("cargo")
        .arg("tarpaulin")
        .arg("-o")
        .arg("Html")
        .arg("--ignore-tests")
        .status()
        .expect("task coverage failed")
        .success());
}
