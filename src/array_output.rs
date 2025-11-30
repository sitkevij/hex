//! Array format output generation for various programming languages
//!
//! This module provides functionality to convert binary data into source code
//! array formats for various programming languages. Supported formats include
//! Rust, C, Go, Python, Kotlin, Java, Swift, and F#.
//!
//! # Examples
//!
//! ```rust
//! use hx::output_array;
//! use std::io::Cursor;
//!
//! let data = vec![0x42, 0x43, 0x44];
//! let buf: Box<dyn std::io::BufRead> = Box::new(Cursor::new(data));
//! // Outputs Rust array format to stdout
//! output_array("r", buf, 0, 10).unwrap();
//! ```

use crate::buffer::buf_to_array;
use crate::format::Format;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// Converts binary data to source code array format for various programming languages
///
/// Reads bytes from the provided buffer and outputs them as a formatted array
/// declaration in the specified programming language. The output is written to
/// standard output with proper formatting and indentation.
///
/// # Supported Formats
///
/// | Code | Language | Example Output |
/// |------|----------|----------------|
/// | `r`  | Rust     | `let ARRAY: [u8; N] = [ ... ];` |
/// | `c`  | C        | `unsigned char ARRAY[N] = { ... };` |
/// | `g`  | Go       | `a := [N]byte{ ... }` |
/// | `p`  | Python   | `a = [ ... ]` |
/// | `k`  | Kotlin   | `val a = byteArrayOf( ... )` |
/// | `j`  | Java     | `byte[] a = new byte[]{ ... };` |
/// | `s`  | Swift    | `let a: [UInt8] = [ ... ]` |
/// | `f`  | F#       | `let a = [\| ... \|]` |
///
/// # Arguments
///
/// * `array_format` - Single character code specifying the output format (`r`, `c`, `g`, `p`, `k`, `j`, `s`, `f`)
/// * `buf` - A boxed `BufRead` trait object containing the input data
/// * `truncate_len` - Maximum number of bytes to process (0 = no limit)
/// * `column_width` - Number of bytes per line in the output
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if:
/// - Reading from the buffer fails
/// - Writing to stdout fails
/// - Format conversion fails
///
/// # Errors
///
/// This function will return an error if:
/// - The buffer cannot be read
/// - Format conversion fails (invalid format code)
/// - Writing to stdout fails
///
/// # Examples
///
/// ```rust,no_run
/// use hx::output_array;
/// use std::io::Cursor;
/// use std::fs::File;
/// use std::io::BufReader;
///
/// // Convert file to Rust array format
/// let file = File::open("data.bin").unwrap();
/// let buf: Box<dyn std::io::BufRead> = Box::new(BufReader::new(file));
/// output_array("r", buf, 0, 16).unwrap();
///
/// // Convert in-memory data to C array format
/// let data = vec![0x01, 0x02, 0x03];
/// let buf: Box<dyn std::io::BufRead> = Box::new(Cursor::new(data));
/// output_array("c", buf, 0, 8).unwrap();
/// ```
pub fn output_array(
    array_format: &str,
    mut buf: Box<dyn BufRead>,
    truncate_len: u64,
    column_width: u64,
) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut locked = stdout.lock();

    let page = buf_to_array(&mut buf, truncate_len, column_width)?;
    match array_format {
        "r" => writeln!(locked, "let ARRAY: [u8; {}] = [", page.bytes)?,
        "c" => writeln!(locked, "unsigned char ARRAY[{}] = {{", page.bytes)?,
        "g" => writeln!(locked, "a := [{}]byte{{", page.bytes)?,
        "p" => writeln!(locked, "a = [")?,
        "k" => writeln!(locked, "val a = byteArrayOf(")?,
        "j" => writeln!(locked, "byte[] a = new byte[]{{")?,
        "s" => writeln!(locked, "let a: [UInt8] = [")?,
        "f" => writeln!(locked, "let a = [|")?,
        _ => writeln!(locked, "unknown array format")?,
    }
    let mut i: u64 = 0x0;
    for line in page.body.iter() {
        write!(locked, "    ")?;
        for hex in line.hex_body.iter() {
            i += 1;
            let hex_str = Format::LowerHex
                .format(*hex, true)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            if i == page.bytes && array_format != "g" {
                if array_format != "f" {
                    write!(locked, "{}", hex_str)?;
                } else {
                    write!(locked, "{}uy", hex_str)?;
                }
            } else if array_format != "f" {
                write!(locked, "{}, ", hex_str)?;
            } else {
                write!(locked, "{}uy; ", hex_str)?;
            }
        }
        writeln!(locked)?;
    }

    writeln!(
        locked,
        "{}",
        match array_format {
            "r" => "];",
            "c" | "j" => "};",
            "g" => "}",
            "p" => "]",
            "k" => ")",
            "s" => "]",
            "f" => "|]",
            _ => "unknown array format",
        }
    )
    .map_err(|e| -> Box<dyn Error> { Box::new(e) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_test_buffer(data: Vec<u8>) -> Box<dyn BufRead> {
        Box::new(Cursor::new(data))
    }

    /// Test output_array with Rust format
    #[test]
    fn test_output_array_rust() {
        let data = vec![0x42, 0x43, 0x44];
        let buf = create_test_buffer(data);
        let result = output_array("r", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with C format
    #[test]
    fn test_output_array_c() {
        let data = vec![0x01, 0x02, 0x03];
        let buf = create_test_buffer(data);
        let result = output_array("c", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with Go format
    #[test]
    fn test_output_array_go() {
        let data = vec![0x10, 0x20, 0x30];
        let buf = create_test_buffer(data);
        let result = output_array("g", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with Python format
    #[test]
    fn test_output_array_python() {
        let data = vec![0xAA, 0xBB];
        let buf = create_test_buffer(data);
        let result = output_array("p", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with Kotlin format
    #[test]
    fn test_output_array_kotlin() {
        let data = vec![0xFF];
        let buf = create_test_buffer(data);
        let result = output_array("k", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with Java format
    #[test]
    fn test_output_array_java() {
        let data = vec![0x11, 0x22, 0x33];
        let buf = create_test_buffer(data);
        let result = output_array("j", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with Swift format
    #[test]
    fn test_output_array_swift() {
        let data = vec![0x99];
        let buf = create_test_buffer(data);
        let result = output_array("s", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with F# format
    #[test]
    fn test_output_array_fsharp() {
        let data = vec![0x55, 0x66];
        let buf = create_test_buffer(data);
        let result = output_array("f", buf, 0, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with unknown format
    #[test]
    fn test_output_array_unknown_format() {
        let data = vec![0x42];
        let buf = create_test_buffer(data);
        let result = output_array("x", buf, 0, 10);
        assert!(result.is_ok()); // Should still succeed but output "unknown array format"
    }

    /// Test output_array with truncation
    #[test]
    fn test_output_array_truncation() {
        let data: Vec<u8> = (0..20).collect();
        let buf = create_test_buffer(data);
        let result = output_array("r", buf, 10, 10);
        assert!(result.is_ok());
    }

    /// Test output_array with empty buffer
    #[test]
    fn test_output_array_empty() {
        let data = vec![];
        let buf = create_test_buffer(data);
        let result = output_array("r", buf, 0, 10);
        assert!(result.is_ok());
    }
}
