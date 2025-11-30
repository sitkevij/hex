#![deny(
    dead_code,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

//! # hx - Overview
//!
//! A modern hex viewer library that displays file contents in hexadecimal format
//! with ASCII representation. This library provides the core functionality for
//! the `hx` command-line tool.
//!
//! ## Features
//!
//! - **Multiple input sources**: Read from files or stdin
//! - **Flexible output formats**: Support for octal, hexadecimal (lower/upper),
//!   binary, pointer, and exponential formats
//! - **Array output modes**: Generate code arrays in various programming languages
//!   (Rust, C, Go, Python, etc.)
//! - **Colorization**: ANSI color codes for better readability (automatically
//!   disabled for non-terminal outputs)
//! - **Configurable display**: Customizable column width, byte truncation, and
//!   formatting options
//!
//! ## Core Concepts
//!
//! The library processes binary data and displays it in a traditional hex editor
//! format:
//!
//! - **Offset**: Memory address or byte position (displayed in hexadecimal)
//! - **Hex values**: Bytes represented as hexadecimal values (grouped by column width)
//! - **ASCII representation**: Printable characters corresponding to each byte
//!
//! ## Usage
//!
//! The main entry point is the [`run`] function, which processes command-line
//! arguments and executes the hex viewing operation.
//!
//! ## Modules
//!
//! The library is organized into several internal modules:
//!
//! - **args**: Command-line argument parsing and validation
//! - **array_output**: Generate programming language array formats
//! - **buffer**: Buffer reading and conversion utilities
//! - **format**: Output format definitions and handling
//! - **function_output**: Mathematical function wave generation
//! - **models**: Data structures for hex display (Line, Page)
//! - **output**: Low-level output formatting and printing functions

extern crate ansi_term;
extern crate clap;

// Module declarations
mod args;
mod array_output;
mod buffer;
mod format;
mod function_output;
mod models;
mod output;

// Re-exports for public API compatibility
pub use args::{
    ARG_ARR, ARG_CLR, ARG_COL, ARG_FMT, ARG_FNC, ARG_INP, ARG_LEN, ARG_PFX, ARG_PLC, is_stdin,
};
pub use array_output::output_array;
pub use buffer::buf_to_array;
pub use format::{Format, FormatError};
pub use function_output::output_function;
pub use models::{Line, Page};
pub use output::{append_ascii, byte_to_color, offset, print_byte, print_offset};

use clap::ArgMatches;
use no_color::is_no_color;
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::io::IsTerminal;
use std::io::{self, BufRead, Write};

/// Main entry point for processing hex viewer operations.
///
/// This function processes command-line arguments and executes the appropriate
/// hex viewing operation. It supports two main modes:
///
/// 1. **Function wave generation**: When the `func` argument is provided,
///    generates and outputs a mathematical sine wave function.
/// 2. **Hex viewing**: Processes input data (from file or stdin) and displays
///    it in hexadecimal format with ASCII representation.
///
/// ## Display Format
///
/// In most hex editor applications, the data of the computer file is
/// represented as hexadecimal values grouped in 4 groups of 4 bytes (or
/// two groups of 8 bytes), followed by one group of 16 printable ASCII
/// characters which correspond to each pair of hex values (each byte).
/// Non-printable ASCII characters (e.g., Bell) and characters that would take
/// more than one character space (e.g., tab) are typically represented by a
/// dot (".") in the following ASCII field.
///
/// ## Input Sources
///
/// The function can read from:
/// - **File**: When a file path is provided as a positional argument
/// - **Stdin**: When no file path is provided and data is piped in
///
/// ## Output Modes
///
/// - **Standard hex view**: Traditional hex dump format with offset, hex bytes, and ASCII
/// - **Array output**: Generate code arrays in various programming languages
///   (specified via the `array` argument: "r" for Rust, "c" for C, etc.)
///
/// ## Format Options
///
/// Supported output formats (via `format` argument):
/// - `"o"`: Octal
/// - `"x"`: Lowercase hexadecimal (default)
/// - `"X"`: Uppercase hexadecimal
/// - `"p"`: Pointer format
/// - `"b"`: Binary
/// - `"e"`: Lowercase exponential
/// - `"E"`: Uppercase exponential
///
/// ## Colorization
///
/// Color output is automatically disabled when:
/// - The `NO_COLOR` environment variable is set
/// - Output is not a terminal (e.g., when piping to another command)
///
/// Color can be explicitly controlled via the `color` argument (`0` = off, `1` = on).
///
/// ## Arguments
///
/// * `matches` - Argument matches from command line parsing (clap `ArgMatches`)
///
/// ## Returns
///
/// Returns `Ok(())` on success, or `Err(Box<dyn Error>)` if:
/// - File cannot be opened or read
/// - Invalid argument values are provided (e.g., non-numeric column width)
/// - I/O errors occur during reading or writing
///
/// ## Examples
///
/// ### Reading from a file
///
/// ```rust,no_run
/// use clap::Command;
/// use hx::run;
///
/// let matches = Command::new("hx")
///     .arg(clap::Arg::new("input").index(1))
///     .get_matches_from(vec!["hx", "Cargo.toml"]);
///
/// run(matches)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ### Reading from stdin
///
/// ```rust,no_run
/// use clap::Command;
/// use hx::run;
///
/// // In practice, this would be called with stdin data piped in:
/// // $ echo "hello" | hx
/// let matches = Command::new("hx").get_matches_from(vec!["hx"]);
/// run(matches)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ### Custom column width and format
///
/// ```rust,no_run
/// use clap::Command;
/// use hx::run;
///
/// let matches = Command::new("hx")
///     .arg(clap::Arg::new("input").index(1))
///     .arg(clap::Arg::new("cols").short('c').long("cols"))
///     .arg(clap::Arg::new("format").short('f').long("format"))
///     .get_matches_from(vec!["hx", "file.bin", "-c", "16", "-f", "X"]);
///
/// run(matches)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ### Array output mode (Rust format)
///
/// ```rust,no_run
/// use clap::Command;
/// use hx::run;
///
/// let matches = Command::new("hx")
///     .arg(clap::Arg::new("input").index(1))
///     .arg(clap::Arg::new("array").short('a').long("array"))
///     .get_matches_from(vec!["hx", "data.bin", "-a", "r"]);
///
/// run(matches)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut column_width: u64 = 10;
    let mut truncate_len: u64 = 0x0;
    if let Some(len) = matches.get_one::<String>("func") {
        let mut p: usize = 4;
        if let Some(places) = matches.get_one::<String>("places") {
            p = match places.parse::<usize>() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("-p, --places <integer> expected. {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }
        let length = len.parse::<u64>().map_err(|e| {
            eprintln!("-u, --func <integer> expected. {:?}", e);
            e
        })?;
        output_function(length, p);
    } else {
        // cases:
        //  $ cat Cargo.toml | target/debug/hx
        //  $ cat Cargo.toml | target/debug/hx -a r
        //  $ target/debug/hx Cargo.toml
        //  $ target/debug/hx Cargo.toml -a r
        let is_stdin = is_stdin(matches.clone())?;
        let mut buf: Box<dyn BufRead> = if is_stdin {
            Box::new(BufReader::new(io::stdin()))
        } else {
            let input_file = matches.get_one::<String>(ARG_INP).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Input file not specified")
            })?;
            Box::new(BufReader::new(fs::File::open(input_file)?))
        };
        let mut format_out = Format::LowerHex;
        let mut colorize = true;
        let mut prefix = true;

        if let Some(columns) = matches.get_one::<String>(ARG_COL) {
            column_width = match columns.parse::<u64>() {
                Ok(column_width) => column_width,
                Err(e) => {
                    eprintln!("-c, --cols <integer> expected. {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }

        if let Some(length) = matches.get_one::<String>(ARG_LEN) {
            truncate_len = match length.parse::<u64>() {
                Ok(truncate_len) => truncate_len,
                Err(e) => {
                    eprintln!("-l, --len <integer> expected. {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }

        if let Some(format) = matches.get_one::<String>(ARG_FMT) {
            // o, x, X, p, b, e, E
            match format.as_str() {
                "o" => format_out = Format::Octal,
                "x" => format_out = Format::LowerHex,
                "X" => format_out = Format::UpperHex,
                "p" => format_out = Format::Pointer,
                "b" => format_out = Format::Binary,
                "e" => format_out = Format::LowerExp,
                "E" => format_out = Format::UpperExp,
                _ => format_out = Format::Unknown,
            }
        }

        // check no_color here
        // override via ARG_CLR below
        if is_no_color() {
            colorize = false;
        }

        // prevent term color codes being sent to stdout
        // test: cat Cargo.toml | target/debug/hx | more
        // override via ARG_CLR below
        if !io::stdout().is_terminal() {
            colorize = false;
        }

        if let Some(color) = matches.get_one::<String>(ARG_CLR) {
            colorize = color.parse::<u8>().map_err(|e| {
                eprintln!("-t, --color <0|1> expected. {:?}", e);
                e
            })? == 1;
        }

        if let Some(prefix_flag) = matches.get_one::<String>(ARG_PFX) {
            prefix = prefix_flag.parse::<u8>().map_err(|e| {
                eprintln!("-r, --prefix <0|1> expected. {:?}", e);
                e
            })? == 1;
        }

        // array output mode is mutually exclusive
        if let Some(array) = matches.get_one::<String>(ARG_ARR) {
            output_array(array, buf, truncate_len, column_width)?;
        } else {
            // Transforms this Read instance to an Iterator over its bytes.
            // The returned type implements Iterator where the Item is
            // Result<u8, R::Err>. The yielded item is Ok if a byte was
            // successfully read and Err otherwise for I/O errors. EOF is
            // mapped to returning None from this iterator.
            // (https://doc.rust-lang.org/1.16.0/std/io/trait.Read.html#method.bytes)
            let mut ascii_line: Line = Line::new();
            let mut offset_counter: u64 = 0x0;
            let mut byte_column: u64 = 0x0;
            let page = buf_to_array(&mut buf, truncate_len, column_width)?;

            let stdout = io::stdout();
            let mut locked = stdout.lock();

            for line in page.body.iter() {
                print_offset(&mut locked, offset_counter)?;

                for hex in line.hex_body.iter() {
                    offset_counter += 1;
                    byte_column += 1;
                    print_byte(&mut locked, *hex, format_out, colorize, prefix)?;
                    append_ascii(&mut ascii_line.ascii, *hex, colorize);
                }

                if byte_column < column_width {
                    write!(
                        locked,
                        "{:<1$}",
                        "",
                        5 * (column_width - byte_column) as usize
                    )?;
                }

                locked.write_all(ascii_line.ascii.as_slice())?;
                writeln!(locked)?;

                byte_column = 0x0;
                ascii_line = Line::new();
            }
            if true {
                writeln!(locked, "   bytes: {}", page.bytes)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    /// Test CLI argument order: flags before file path
    ///
    /// Verifies that the `-ar` flags can be placed before the file path argument.
    /// This tests the flexibility of argument parsing order.
    ///
    /// Expected behavior: Array output mode with Rust format should work
    /// regardless of whether flags come before or after the file path.
    ///
    /// Note: Assertions may have unexpected results depending on terminal
    /// configuration and color settings.
    #[test]
    fn test_cli_arg_order_1() {
        let mut cmd = Command::cargo_bin("hx").unwrap();
        let assert = cmd.arg("-ar").arg("tests/files/tiny.txt").assert();
        assert.success().code(0);
    }

    /// Test CLI argument order: flags after file path
    ///
    /// Verifies that the `-ar` flags can be placed after the file path argument.
    /// This tests the flexibility of argument parsing order.
    ///
    /// Expected behavior: Array output mode with Rust format should work
    /// regardless of whether flags come before or after the file path.
    ///
    /// Note: Assertions may have unexpected results depending on terminal
    /// configuration and color settings.
    #[test]
    fn test_cli_arg_order_2() {
        let mut cmd = Command::cargo_bin("hx").unwrap();
        let assert = cmd.arg("tests/files/tiny.txt").arg("-ar").assert();
        assert.success().code(0);
    }

    /// Test CLI error handling when parameter value is missing
    ///
    /// Verifies that the application correctly handles cases where a flag
    /// requiring a value (like `--len`) is provided but the value is missing
    /// or invalid.
    ///
    /// Expected behavior: The application should fail with exit code 1 and
    /// display an appropriate error message indicating that an integer value
    /// was expected.
    #[test]
    fn test_cli_missing_param_value() {
        let mut cmd = Command::cargo_bin("hx").unwrap();
        let assert = cmd.arg("--len").arg("tests/files/tiny.txt").assert();
        assert.failure().code(1);
    }

    /// Test CLI error handling when input file doesn't exist
    ///
    /// Verifies that the application correctly handles cases where the
    /// specified input file cannot be found or accessed.
    ///
    /// Expected behavior: The application should fail with exit code 1 and
    /// display an appropriate error message indicating the file was not found.
    #[test]
    fn test_cli_input_missing_file() {
        let mut cmd = Command::cargo_bin("hx").unwrap();
        let assert = cmd.arg("missing-file").assert();
        assert.failure().code(1);
    }

    /// Test CLI error handling when input is a directory instead of a file
    ///
    /// Verifies that the application correctly rejects directory paths and
    /// only accepts regular files as input.
    ///
    /// Expected behavior: The application should fail with exit code 1 and
    /// display an appropriate error message indicating that a file was expected,
    /// not a directory.
    #[test]
    fn test_cli_input_directory() {
        let mut cmd = Command::cargo_bin("hx").unwrap();
        let assert = cmd.arg("src").assert();
        assert.failure().code(1);
    }

    /// Test CLI behavior with stdin input
    ///
    /// Verifies that the application correctly processes input from stdin
    /// when no file path is provided. This test uses the `-t0` flag to disable
    /// colorization for consistent output comparison.
    ///
    /// Expected behavior: The application should successfully read from stdin,
    /// process the input bytes, and output the hex representation with ASCII
    /// characters. The output should include the byte count at the end.
    ///
    /// # Example Output
    ///
    /// For input "012", the expected output is:
    /// ```text
    /// 0x000000: 0x30 0x31 0x32                                    012
    ///    bytes: 3
    /// ```
    #[test]
    fn test_cli_input_stdin() {
        let mut cmd = Command::cargo_bin("hx").unwrap();
        let assert = cmd.arg("-t0").write_stdin("012").assert();
        assert.success().code(0).stdout(
            "0x000000: 0x30 0x31 0x32                                    012\n   bytes: 3\n",
        );
    }
}
