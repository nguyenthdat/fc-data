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
        .stdout(predicate::str::contains("intraday-by-tick"))
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

#[test]
fn rejects_intraday_ohlc_page_size_10000() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .args(["intraday-ohlc", "--symbol", "SSI", "--page-size", "10000"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("page size"));
}

#[test]
fn rejects_page_size_10000_for_daily_ohlc() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .args([
            "daily-ohlc",
            "--from-date",
            "13/08/2026",
            "--to-date",
            "14/08/2026",
            "--page-size",
            "10000",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("page size"));
}

#[test]
fn accepts_intraday_by_tick_flags() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .args([
            "intraday-by-tick",
            "--symbol",
            "SSI",
            "--from-date",
            "14/08/2026",
            "--to-date",
            "14/08/2026",
        ])
        .env("SSI_FCDATA_CONSUMER_ID", "test-consumer")
        .env("SSI_FCDATA_CONSUMER_SECRET", "test-secret")
        .env("SSI_FCDATA_API_URL", "http://127.0.0.1/")
        .env("SSI_FCDATA_STREAM_URL", "http://127.0.0.1/")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unsupported URL scheme"));
}

#[test]
fn accepts_daily_index_ascending_flag() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .args([
            "daily-index",
            "--index-id",
            "VN30",
            "--from-date",
            "13/08/2026",
            "--to-date",
            "14/08/2026",
            "--ascending",
            "true",
        ])
        .env("SSI_FCDATA_CONSUMER_ID", "test-consumer")
        .env("SSI_FCDATA_CONSUMER_SECRET", "test-secret")
        .env("SSI_FCDATA_API_URL", "http://127.0.0.1/")
        .env("SSI_FCDATA_STREAM_URL", "http://127.0.0.1/")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unsupported URL scheme"));
}

#[test]
fn rejects_intraday_by_tick_without_both_dates() {
    // Given
    let mut command = cargo_bin_cmd!("fc-data");

    // When / Then
    command
        .args([
            "intraday-by-tick",
            "--symbol",
            "SSI",
            "--from-date",
            "14/08/2026",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--to-date"));
}
