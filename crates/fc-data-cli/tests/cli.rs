#![doc = "End-to-end command-line parsing tests."]

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn shows_supported_commands_when_help_is_requested() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("securities"))
        .stdout(predicate::str::contains("stream"));
}

#[test]
fn rejects_an_unsupported_page_size_before_contacting_ssi() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .args(["securities", "--page-size", "25"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("page size"));
}
