//! Integration tests for the hx binary
//!
//! These tests verify that the binary can be executed and processes input correctly.

extern crate assert_cmd;
extern crate hx;

use assert_cmd::Command;
use std::env;

/// Test text used for stdin tests (matches CI TEST_TEXT environment variable)
const TEST_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[test]
fn test_binary_execution_with_stdin() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin(TEST_TEXT);
    cmd.assert().success();
}

#[test]
fn test_binary_execution_with_empty_stdin() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin("");
    cmd.assert().success();
}

#[test]
fn test_binary_execution_with_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("tests/files/tiny.txt");
    cmd.assert().success();
}

#[test]
fn test_binary_execution_with_format_flag() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-f").arg("x");
    cmd.write_stdin(TEST_TEXT);
    cmd.assert().success();
}

#[test]
fn test_binary_execution_with_array_format() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-a").arg("r");
    cmd.write_stdin(TEST_TEXT);
    cmd.assert().success();
}

#[test]
fn test_binary_execution_with_column_width() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-c").arg("4");
    cmd.write_stdin(TEST_TEXT);
    cmd.assert().success();
}

#[test]
fn test_binary_execution_with_file_and_flags() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("tests/files/tiny.txt").arg("-a").arg("r");
    cmd.assert().success();
}

#[test]
fn test_binary_execution_flags_before_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-a").arg("r").arg("tests/files/tiny.txt");
    cmd.assert().success();
}

/// Combined `-ar` before the file path (short-flag bundling).
#[test]
fn test_cli_arg_order_1() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("-ar").arg("tests/files/tiny.txt").assert();
    assert.success().code(0);
}

/// Combined `-ar` after the file path (short-flag bundling).
#[test]
fn test_cli_arg_order_2() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("tests/files/tiny.txt").arg("-ar").assert();
    assert.success().code(0);
}

/// `--len` without an integer value should fail.
#[test]
fn test_cli_missing_param_value() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--len").arg("tests/files/tiny.txt").assert();
    assert.failure().code(1);
}

/// Missing input file should fail.
#[test]
fn test_cli_input_missing_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("missing-file").assert();
    assert.failure().code(1);
}

/// Directory path instead of a file should fail.
#[test]
fn test_cli_input_directory() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("src").assert();
    assert.failure().code(1);
}

/// Stdin with color disabled (`-t0`) should match exact hex output.
#[test]
fn test_cli_input_stdin() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("-t0").write_stdin("012").assert();
    assert.success().code(0).stdout(
        "0x000000: 0x30 0x31 0x32                                    012\n   bytes: 3\n",
    );
}
