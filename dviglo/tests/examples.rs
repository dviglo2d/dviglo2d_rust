// Запускает примеры как тесты

use std::process::Command;


#[test]
fn test_example_hello() {
    let status = Command::new("cargo")
        .args(&["run", "--quiet", "--example", "hello"])
        .status()
        .expect("Failed to execute cargo run");

    assert!(status.success(), "Example failed to run!");
}
