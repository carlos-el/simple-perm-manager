use std::process::Command;

// Run audit (security advisor)
fn main() {
    // Generate Cargo.lock file (used by cargo-audit to check vulnerabilities)
    assert!(Command::new("cargo")
        .arg("generate-lockfile")
        .status()
        .expect("task check failed")
        .success());

    // Run cargo audit
    assert!(Command::new("cargo")
        .arg("audit")
        .status()
        .expect("task check failed")
        .success());
}
