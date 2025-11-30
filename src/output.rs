//! Output and display functions for hex representation
//!
//! This module provides functions for formatting and displaying hexadecimal
//! data, including offset formatting, byte printing with various formats,
//! color mapping, and ASCII representation.
//!
//! # Examples
//!
//! ```rust
//! use hx::{Format, offset, print_byte, byte_to_color, append_ascii};
//!
//! // Format an offset
//! let offset_str = offset(0x1000);
//! assert_eq!(offset_str, "0x001000");
//!
//! // Print a byte with formatting
//! let mut buffer = Vec::new();
//! print_byte(&mut buffer, 0x42, Format::LowerHex, false, true).unwrap();
//!
//! // Get color for a byte
//! let color = byte_to_color(0xFF);
//! assert_eq!(color, 255);
//!
//! // Append ASCII representation
//! let mut ascii_buf = Vec::new();
//! append_ascii(&mut ascii_buf, b'A', false);
//! assert_eq!(ascii_buf, b"A");
//! ```

use crate::format::Format;
use std::io::{self, Write};

/// Formats a memory offset as a hexadecimal string
///
/// Converts a `u64` offset value into a formatted hexadecimal string with
/// a `0x` prefix and zero-padding to 6 hexadecimal digits (8 characters total).
///
/// # Arguments
///
/// * `b` - The offset value to format (typically a memory address)
///
/// # Returns
///
/// Returns a `String` containing the formatted offset in the format `0xNNNNNN`
/// where N is a hexadecimal digit (6 hex digits total).
///
/// # Examples
///
/// ```rust
/// use hx::offset;
///
/// assert_eq!(offset(0), "0x000000");
/// assert_eq!(offset(0x42), "0x000042");
/// assert_eq!(offset(0x1000), "0x001000");
/// assert_eq!(offset(0xFFFFFFFF), "0xffffffff");
/// ```
pub fn offset(b: u64) -> String {
    format!("{:#08x}", b)
}

/// Writes a formatted offset to a writer
///
/// Formats the offset using [`offset()`] and writes it to the provided writer
/// followed by a colon and space (`: `).
///
/// # Arguments
///
/// * `w` - A mutable reference to a type implementing `Write`
/// * `b` - The offset value to format and write
///
/// # Returns
///
/// Returns `io::Result<()>` indicating success or failure of the write operation.
///
/// # Examples
///
/// ```rust
/// use hx::print_offset;
///
/// let mut buffer = Vec::new();
/// print_offset(&mut buffer, 0x42).unwrap();
/// let output = String::from_utf8(buffer).unwrap();
/// assert_eq!(output, "0x000042: ");
/// ```
pub fn print_offset(w: &mut impl Write, b: u64) -> io::Result<()> {
    write!(w, "{}: ", offset(b))
}

/// Formats and writes a byte to a writer
///
/// Formats a byte according to the specified format (octal, hex, binary, etc.)
/// and writes it to the writer. Optionally adds colorization and numeric base prefixes.
///
/// # Arguments
///
/// * `w` - A mutable reference to a type implementing `Write`
/// * `b` - The byte value (0-255) to format and write
/// * `format` - The format variant to use (`Format::Octal`, `Format::LowerHex`, etc.)
/// * `colorize` - Whether to apply ANSI color codes based on byte value
/// * `prefix` - Whether to include the numeric base prefix (e.g., `0x`, `0o`, `0b`)
///
/// # Returns
///
/// Returns `io::Result<()>` indicating success or failure. Will return an error
/// if the format variant is not implemented.
///
/// # Errors
///
/// Returns `io::Error` with `InvalidInput` kind if the format variant is not supported.
///
/// # Examples
///
/// ```rust
/// use hx::{Format, print_byte};
///
/// let mut buffer = Vec::new();
/// // Print as lowercase hex with prefix, no color
/// print_byte(&mut buffer, 0x42, Format::LowerHex, false, true).unwrap();
/// let output = String::from_utf8(buffer).unwrap();
/// assert!(output.contains("0x42"));
///
/// // Print as binary with prefix
/// let mut buffer2 = Vec::new();
/// print_byte(&mut buffer2, 0b1010, Format::Binary, false, true).unwrap();
/// let output2 = String::from_utf8(buffer2).unwrap();
/// assert!(output2.contains("0b00001010"));
/// ```
pub fn print_byte(
    w: &mut impl Write,
    b: u8,
    format: Format,
    colorize: bool,
    prefix: bool,
) -> io::Result<()> {
    let fmt_string = format
        .format(b, prefix)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    if colorize {
        // note, for color testing: for (( i = 0; i < 256; i++ )); do echo "$(tput setaf $i)This is ($i) $(tput sgr0)"; done
        let color = byte_to_color(b);
        write!(
            w,
            "{} ",
            ansi_term::Style::new()
                .fg(ansi_term::Color::Fixed(color))
                .paint(fmt_string)
        )
    } else {
        write!(w, "{} ", fmt_string)
    }
}

/// Maps a byte value to an ANSI terminal color code
///
/// Converts a byte value (0-255) to a terminal color code for use with ANSI
/// color sequences. Byte value 0 is mapped to color code 0x16 (22) to ensure
/// visibility, while all other values map directly to their byte value.
///
/// # Arguments
///
/// * `b` - The byte value to map to a color code
///
/// # Returns
///
/// Returns a `u8` color code suitable for use with ANSI terminal color sequences.
///
/// # Examples
///
/// ```rust
/// use hx::byte_to_color;
///
/// // Zero maps to minimum visible color
/// assert_eq!(byte_to_color(0), 0x16);
///
/// // Other values map directly
/// assert_eq!(byte_to_color(1), 1);
/// assert_eq!(byte_to_color(42), 42);
/// assert_eq!(byte_to_color(255), 255);
/// ```
pub fn byte_to_color(b: u8) -> u8 {
    let mut color: u8 = b;
    if color < 1 {
        color = 0x16;
    }
    color
}

/// Appends the ASCII representation of a byte to a buffer
///
/// Converts a byte to its ASCII character representation and appends it to the
/// target buffer. Printable ASCII characters (32-126) are represented as their
/// actual characters, while non-printable characters are represented as a
/// period (`.`). Optionally applies ANSI color codes based on the byte value.
///
/// # Arguments
///
/// * `target` - A mutable reference to a `Vec<u8>` buffer to append to
/// * `b` - The byte value to convert to ASCII representation
/// * `colorize` - Whether to apply ANSI color codes to the output
///
/// # ASCII Character Mapping
///
/// | Byte Range | Representation |
/// |------------|----------------|
/// | 0-31       | `.` (non-printable) |
/// | 32-126     | Actual character (space through `~`) |
/// | 127+       | `.` (non-printable) |
///
/// # Examples
///
/// ```rust
/// use hx::append_ascii;
///
/// // Printable character
/// let mut buffer = Vec::new();
/// append_ascii(&mut buffer, b'A', false);
/// assert_eq!(buffer, b"A");
///
/// // Non-printable character
/// let mut buffer2 = Vec::new();
/// append_ascii(&mut buffer2, 0x00, false);
/// assert_eq!(buffer2, b".");
///
/// // Space character
/// let mut buffer3 = Vec::new();
/// append_ascii(&mut buffer3, 32, false);
/// assert_eq!(buffer3, b" ");
/// ```
pub fn append_ascii(target: &mut Vec<u8>, b: u8, colorize: bool) {
    let char = match b > 31 && b < 127 {
        true => b as char,
        false => '.',
    };

    if colorize {
        let string = ansi_term::Style::new()
            .fg(ansi_term::Color::Fixed(byte_to_color(b)))
            .paint(char.to_string());
        target.extend(format!("{}", string).as_bytes());
    } else {
        target.extend(format!("{}", char).as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::Format;
    use std::io;

    /// Test offset formatting
    ///
    /// See: <https://users.rust-lang.org/t/how-to-test-output-to-stdout/4877/6>
    #[test]
    fn test_offset() {
        let b: u64 = 0x6;
        assert_eq!(offset(b), "0x000006");
        assert_eq!(offset(b), format!("{:#08x}", b));
    }

    /// Test offset with zero
    #[test]
    fn test_offset_zero() {
        assert_eq!(offset(0), "0x000000");
    }

    /// Test offset with maximum value
    #[test]
    fn test_offset_max() {
        assert_eq!(offset(0xFFFFFFFF), "0xffffffff");
    }

    /// Test print_offset writes correct format
    #[test]
    fn test_print_offset() {
        let mut buffer = Vec::new();
        print_offset(&mut buffer, 0x42).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "0x000042: ");
    }

    /// Test `print_byte` handles format errors correctly
    #[test]
    fn test_print_byte_with_invalid_format() {
        let mut buffer = Vec::new();
        let result = print_byte(&mut buffer, 0x42, Format::Pointer, false, true);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }

    /// Test `print_byte` succeeds with valid format
    #[test]
    fn test_print_byte_with_valid_format() {
        let mut buffer = Vec::new();
        let result = print_byte(&mut buffer, 0x42, Format::LowerHex, false, true);

        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("0x42"));
    }

    /// Test print_byte with different formats
    #[test]
    fn test_print_byte_formats() {
        let mut buffer = Vec::new();
        print_byte(&mut buffer, 0xFF, Format::LowerHex, false, true).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("0xff"));

        let mut buffer2 = Vec::new();
        print_byte(&mut buffer2, 0xFF, Format::UpperHex, false, true).unwrap();
        let output2 = String::from_utf8(buffer2).unwrap();
        assert!(output2.contains("0xFF"));

        let mut buffer3 = Vec::new();
        print_byte(&mut buffer3, 0xFF, Format::Octal, false, true).unwrap();
        let output3 = String::from_utf8(buffer3).unwrap();
        assert!(output3.contains("0o0377"));

        let mut buffer4 = Vec::new();
        print_byte(&mut buffer4, 0xFF, Format::Binary, false, true).unwrap();
        let output4 = String::from_utf8(buffer4).unwrap();
        assert!(output4.contains("0b11111111"));
    }

    /// Test print_byte with and without prefix
    #[test]
    fn test_print_byte_prefix() {
        let mut buffer = Vec::new();
        print_byte(&mut buffer, 0x42, Format::LowerHex, false, true).unwrap();
        let with_prefix = String::from_utf8(buffer).unwrap();
        assert!(with_prefix.contains("0x42"));

        let mut buffer2 = Vec::new();
        print_byte(&mut buffer2, 0x42, Format::LowerHex, false, false).unwrap();
        let without_prefix = String::from_utf8(buffer2).unwrap();
        assert!(without_prefix.contains("42"));
        assert!(!without_prefix.contains("0x"));
    }

    /// Test `byte_to_color` with zero returns minimum color
    #[test]
    fn test_byte_to_color_zero() {
        assert_eq!(byte_to_color(0), 0x16);
    }

    /// Test `byte_to_color` with non-zero returns same value
    #[test]
    fn test_byte_to_color_non_zero() {
        assert_eq!(byte_to_color(1), 1);
        assert_eq!(byte_to_color(42), 42);
        assert_eq!(byte_to_color(255), 255);
    }

    /// Test `byte_to_color` edge cases
    #[test]
    fn test_byte_to_color_edge_cases() {
        assert_eq!(byte_to_color(0), 0x16);
        assert_eq!(byte_to_color(1), 1);
        assert_eq!(byte_to_color(255), 255);
    }

    /// Test `append_ascii` with printable character
    #[test]
    fn test_append_ascii_printable() {
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, b'A', false);
        assert_eq!(buffer, b"A");
    }

    /// Test `append_ascii` with non-printable character
    #[test]
    fn test_append_ascii_non_printable() {
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, 0x00, false);
        assert_eq!(buffer, b".");

        let mut buffer2 = Vec::new();
        append_ascii(&mut buffer2, 0x1F, false);
        assert_eq!(buffer2, b".");

        let mut buffer3 = Vec::new();
        append_ascii(&mut buffer3, 0x7F, false);
        assert_eq!(buffer3, b".");
    }

    /// Test `append_ascii` with boundary values
    #[test]
    fn test_append_ascii_boundaries() {
        // Just below printable range (31)
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, 31, false);
        assert_eq!(buffer, b".");

        // Start of printable range (32 = space)
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, 32, false);
        assert_eq!(buffer, b" ");

        // End of printable range (126 = '~')
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, 126, false);
        assert_eq!(buffer, b"~");

        // Just after printable range (127 = DEL)
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, 127, false);
        assert_eq!(buffer, b".");
    }

    /// Test append_ascii with colorization disabled
    #[test]
    fn test_append_ascii_no_color() {
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, b'X', false);
        assert_eq!(buffer, b"X");
    }

    /// Test append_ascii with multiple characters
    #[test]
    fn test_append_ascii_multiple() {
        let mut buffer = Vec::new();
        append_ascii(&mut buffer, b'H', false);
        append_ascii(&mut buffer, b'e', false);
        append_ascii(&mut buffer, b'l', false);
        append_ascii(&mut buffer, b'l', false);
        append_ascii(&mut buffer, b'o', false);
        assert_eq!(buffer, b"Hello");
    }
}
