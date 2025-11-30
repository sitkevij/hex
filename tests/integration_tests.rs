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
