use super::*;
/// @see (https://users.rust-lang.org/t/how-to-test-output-to-stdout/4877/6)
#[test]
fn test_offset() {
    let b: u64 = 0x6;
    assert_eq!(offset(b), "0x000006");
    assert_eq!(offset(b), format!("{:#08x}", b));
}

/// hex octal, takes u8
#[test]
pub fn test_hex_octal() {
    let b: u8 = 0x6;

    //with prefix
    assert_eq!(Format::Octal.format(b, true), "0o0006");
    assert_eq!(Format::Octal.format(b, true), format!("{:#06o}", b));

    //without prefix
    assert_eq!(Format::Octal.format(b, false), "0006");
    assert_eq!(Format::Octal.format(b, false), format!("{:04o}", b));
}

/// hex lower hex, takes u8
#[test]
fn test_hex_lower_hex() {
    let b: u8 = <u8>::max_value(); // 255

    //with prefix
    assert_eq!(Format::LowerHex.format(b, true), "0xff");
    assert_eq!(Format::LowerHex.format(b, true), format!("{:#04x}", b));

    //without prefix
    assert_eq!(Format::LowerHex.format(b, false), "ff");
    assert_eq!(Format::LowerHex.format(b, false), format!("{:02x}", b));
}

/// hex upper hex, takes u8
#[test]
fn test_hex_upper_hex() {
    let b: u8 = <u8>::max_value();

    //with prefix
    assert_eq!(Format::UpperHex.format(b, true), "0xFF");
    assert_eq!(Format::UpperHex.format(b, true), format!("{:#04X}", b));

    // without prefix
    assert_eq!(Format::UpperHex.format(b, false), "FF");
    assert_eq!(Format::UpperHex.format(b, false), format!("{:02X}", b));
}

/// hex binary, takes u8
#[test]
fn test_hex_binary() {
    let b: u8 = <u8>::max_value();

    // with prefix
    assert_eq!(Format::Binary.format(b, true), "0b11111111");
    assert_eq!(Format::Binary.format(b, true), format!("{:#010b}", b));

    // without prefix
    assert_eq!(Format::Binary.format(b, false), "11111111");
    assert_eq!(Format::Binary.format(b, false), format!("{:08b}", b));
}

#[test]
fn test_line_struct() {
    let mut ascii_line: Line = Line::new();
    ascii_line.ascii.push(b'.');
    assert_eq!(ascii_line.ascii[0], b'.');
    assert_eq!(ascii_line.offset, 0x0);
}

use assert_cmd::Command;

/// target/debug/hx -ar tests/files/tiny.txt
/// assert may have unexpected results depending on terminal:
///     .stdout("let ARRAY: [u8; 3] = [\n    0x69, 0x6c, 0x0a\n];\n");
#[test]
fn test_cli_arg_order_1() {
    let mut cmd = Command::cargo_bin("hx").unwrap();
    let assert = cmd.arg("-ar").arg("tests/files/tiny.txt").assert();
    assert.success().code(0);
}

/// target/debug/hx tests/files/tiny.txt -ar
/// assert may have unexpected results depending on terminal:
///     .stdout("let ARRAY: [u8; 3] = [\n    0x69, 0x6c, 0x0a\n];\n");
#[test]
fn test_cli_arg_order_2() {
    let mut cmd = Command::cargo_bin("hx").unwrap();
    let assert = cmd.arg("tests/files/tiny.txt").arg("-ar").assert();
    assert.success().code(0);
}

/// target/debug/hx --len tests/files/tiny.txt
///     error: invalid digit found in string
#[test]
fn test_cli_missing_param_value() {
    let mut cmd = Command::cargo_bin("hx").unwrap();
    let assert = cmd.arg("--len").arg("tests/files/tiny.txt").assert();
    assert.failure().code(1);
}

#[test]
fn test_cli_input_missing_file() {
    let mut cmd = Command::cargo_bin("hx").unwrap();
    let assert = cmd.arg("missing-file").assert();
    assert.failure().code(1);
}

#[test]
fn test_cli_input_directory() {
    let mut cmd = Command::cargo_bin("hx").unwrap();
    let assert = cmd.arg("src").assert();
    assert.failure().code(1);
}

#[test]
fn test_cli_input_stdin() {
    let mut cmd = Command::cargo_bin("hx").unwrap();
    let assert = cmd.arg("-t0").write_stdin("012").assert();
    assert
        .success()
        .code(0)
        .stdout("0x000000: 0x30 0x31 0x32                                    012\n   bytes: 3\n");
}
