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
        .stdout(predicate::str::contains("backtest"))
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

#[test]
fn rejects_page_size_500_for_securities_before_loading_configuration() {
    let mut command = cargo_bin_cmd!("fc-data");

    command
        .args(["securities", "--page-size", "500"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("page size"));
}

#[test]
fn rejects_bond_for_securities_before_loading_configuration() {
    let mut command = cargo_bin_cmd!("fc-data");

    command
        .args(["securities", "--market", "bond"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("possible values"));
}

#[test]
fn rejects_upcom_for_index_list_before_loading_configuration() {
    let mut command = cargo_bin_cmd!("fc-data");

    command
        .args(["index-list", "--exchange", "upcom"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("possible values"));
}

#[test]
fn accepts_intraday_without_date_flags() {
    let mut command = cargo_bin_cmd!("fc-data");

    command
        .args(["intraday-ohlc", "--symbol", "SSI"])
        .env("SSI_FCDATA_CONSUMER_ID", "test-consumer")
        .env("SSI_FCDATA_CONSUMER_SECRET", "test-secret")
        .env("SSI_FCDATA_API_URL", "http://127.0.0.1/")
        .env("SSI_FCDATA_STREAM_URL", "http://127.0.0.1/")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unsupported URL scheme"));
}
