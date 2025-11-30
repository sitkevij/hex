//! Format types and formatting logic for hexadecimal display
//!
//! This module provides types and functionality for formatting bytes in various
//! numeric bases (octal, hexadecimal, binary) with optional prefixes.
//!
//! # Examples
//!
//! ```rust
//! use hx::Format;
//!
//! // Format a byte as lowercase hexadecimal with prefix
//! let formatted = Format::LowerHex.format(0x42, true)?;
//! assert_eq!(formatted, "0x42");
//!
//! // Format a byte as binary without prefix
//! let formatted = Format::Binary.format(0xFF, false)?;
//! assert_eq!(formatted, "11111111");
//! # Ok::<(), hx::FormatError>(())
//! ```

use std::error::Error;

/// Custom error type for format operations
///
/// This error is returned when attempting to format a byte using a format variant
/// that is not yet implemented.
///
/// # Examples
///
/// ```rust
/// use hx::{Format, FormatError};
///
/// let result = Format::Pointer.format(0x42, true);
/// assert!(matches!(result, Err(FormatError::Unimplemented(Format::Pointer))));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum FormatError {
    /// Format variant is not implemented
    ///
    /// Contains the `Format` variant that was attempted but is not supported.
    Unimplemented(Format),
}

impl std::fmt::Display for FormatError {
    /// Formats the error as a user-facing error message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hx::{Format, FormatError};
    ///
    /// let error = FormatError::Unimplemented(Format::Pointer);
    /// assert_eq!(format!("{}", error), "format Pointer is not implemented");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unimplemented(format) => {
                write!(f, "format {:?} is not implemented", format)
            }
        }
    }
}

impl Error for FormatError {
    /// Provides a description of the error
    ///
    /// This implementation provides a default description. The actual error
    /// message is provided by the `Display` implementation.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Format variants for hexadecimal display output
///
/// This enum represents the different numeric base formats available for
/// displaying bytes. Each variant corresponds to a standard Rust formatting trait.
///
/// # Format Variants
///
/// | Variant | Character | Description | Status |
/// |---------|-----------|-------------|--------|
/// | `Octal` | `o` | Base 8 (octal) | ✅ Implemented |
/// | `LowerHex` | `x` | Base 16 lowercase | ✅ Implemented |
/// | `UpperHex` | `X` | Base 16 uppercase | ✅ Implemented |
/// | `Binary` | `b` | Base 2 (binary) | ✅ Implemented |
/// | `Pointer` | `p` | Pointer format | ❌ Unimplemented |
/// | `LowerExp` | `e` | Scientific notation lowercase | ❌ Unimplemented |
/// | `UpperExp` | `E` | Scientific notation uppercase | ❌ Unimplemented |
/// | `Unknown` | - | Unknown/unsupported format | ❌ Unimplemented |
///
/// # Examples
///
/// ```rust
/// use hx::Format;
///
/// // Format as lowercase hexadecimal
/// let hex = Format::LowerHex.format(255, true)?;
/// assert_eq!(hex, "0xff");
///
/// // Format as binary
/// let bin = Format::Binary.format(42, false)?;
/// assert_eq!(bin, "00101010");
///
/// // Format as octal
/// let oct = Format::Octal.format(64, true)?;
/// assert_eq!(oct, "0o0100");
/// # Ok::<(), hx::FormatError>(())
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    /// octal format
    Octal,
    /// lower hex format
    LowerHex,
    /// upper hex format
    UpperHex,
    /// pointer format
    Pointer,
    /// binary format
    Binary,
    /// lower exp format
    LowerExp,
    /// upper exp format
    UpperExp,
    /// unknown format
    Unknown,
}

impl Format {
    /// Formats a given byte according to the format variant
    ///
    /// This method converts a `u8` value into a string representation based on
    /// the numeric base specified by the `Format` variant. The output can optionally
    /// include a prefix (e.g., `0x` for hex, `0o` for octal, `0b` for binary).
    ///
    /// # Arguments
    ///
    /// * `data` - The byte value (0-255) to be formatted
    /// * `prefix` - Whether to include the numeric base prefix (`true`) or not (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the formatted representation, or
    /// `Err(FormatError::Unimplemented)` if the format variant is not supported.
    ///
    /// # Errors
    ///
    /// Returns `FormatError::Unimplemented` if the format variant is not yet
    /// implemented. Currently, only `Octal`, `LowerHex`, `UpperHex`, and `Binary`
    /// are supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hx::Format;
    ///
    /// // Format with prefix
    /// let result = Format::LowerHex.format(0x42, true)?;
    /// assert_eq!(result, "0x42");
    ///
    /// // Format without prefix
    /// let result = Format::LowerHex.format(0x42, false)?;
    /// assert_eq!(result, "42");
    ///
    /// // Binary format
    /// let result = Format::Binary.format(0b1010, true)?;
    /// assert_eq!(result, "0b00001010");
    ///
    /// // Octal format
    /// let result = Format::Octal.format(64, true)?;
    /// assert_eq!(result, "0o0100");
    /// # Ok::<(), hx::FormatError>(())
    /// ```
    ///
    /// # Format Details
    ///
    /// | Format | With Prefix | Without Prefix | Example (255) |
    /// |--------|-------------|----------------|---------------|
    /// | `Octal` | `0o000377` | `0377` | `0o000377` / `0377` |
    /// | `LowerHex` | `0xff` | `ff` | `0xff` / `ff` |
    /// | `UpperHex` | `0xFF` | `FF` | `0xFF` / `FF` |
    /// | `Binary` | `0b11111111` | `11111111` | `0b11111111` / `11111111` |
    pub fn format(&self, data: u8, prefix: bool) -> Result<String, FormatError> {
        let formatted = if prefix {
            match self {
                Self::Octal => format!("{:#06o}", data),
                Self::LowerHex => format!("{:#04x}", data),
                Self::UpperHex => format!("{:#04X}", data),
                Self::Binary => format!("{:#010b}", data),
                _ => return Err(FormatError::Unimplemented(*self)),
            }
        } else {
            match self {
                Self::Octal => format!("{:04o}", data),
                Self::LowerHex => format!("{:02x}", data),
                Self::UpperHex => format!("{:02X}", data),
                Self::Binary => format!("{:08b}", data),
                _ => return Err(FormatError::Unimplemented(*self)),
            }
        };
        Ok(formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    /// Test octal format output with and without prefix
    #[test]
    fn test_hex_octal() {
        let b: u8 = 0x6;

        //with prefix
        assert_eq!(Format::Octal.format(b, true).unwrap(), "0o0006");
        assert_eq!(
            Format::Octal.format(b, true).unwrap(),
            format!("{:#06o}", b)
        );

        //without prefix
        assert_eq!(Format::Octal.format(b, false).unwrap(), "0006");
        assert_eq!(
            Format::Octal.format(b, false).unwrap(),
            format!("{:04o}", b)
        );
    }

    /// Test lower hex format output with and without prefix
    #[test]
    fn test_hex_lower_hex() {
        let b: u8 = u8::MAX; // 255

        //with prefix
        assert_eq!(Format::LowerHex.format(b, true).unwrap(), "0xff");
        assert_eq!(
            Format::LowerHex.format(b, true).unwrap(),
            format!("{:#04x}", b)
        );

        //without prefix
        assert_eq!(Format::LowerHex.format(b, false).unwrap(), "ff");
        assert_eq!(
            Format::LowerHex.format(b, false).unwrap(),
            format!("{:02x}", b)
        );
    }

    /// Test upper hex format output with and without prefix
    #[test]
    fn test_hex_upper_hex() {
        let b: u8 = u8::MAX;

        //with prefix
        assert_eq!(Format::UpperHex.format(b, true).unwrap(), "0xFF");
        assert_eq!(
            Format::UpperHex.format(b, true).unwrap(),
            format!("{:#04X}", b)
        );

        // without prefix
        assert_eq!(Format::UpperHex.format(b, false).unwrap(), "FF");
        assert_eq!(
            Format::UpperHex.format(b, false).unwrap(),
            format!("{:02X}", b)
        );
    }

    /// Test binary format output with and without prefix
    #[test]
    fn test_hex_binary() {
        let b: u8 = u8::MAX;

        // with prefix
        assert_eq!(Format::Binary.format(b, true).unwrap(), "0b11111111");
        assert_eq!(
            Format::Binary.format(b, true).unwrap(),
            format!("{:#010b}", b)
        );

        // without prefix
        assert_eq!(Format::Binary.format(b, false).unwrap(), "11111111");
        assert_eq!(
            Format::Binary.format(b, false).unwrap(),
            format!("{:08b}", b)
        );
    }

    /// Test `FormatError` Display implementation
    #[test]
    fn test_format_error_display() {
        let error = FormatError::Unimplemented(Format::Pointer);
        let error_string = format!("{}", error);
        assert!(error_string.contains("not implemented"));
        assert!(error_string.contains("Pointer"));
    }

    /// Test `FormatError` is a proper Error type
    #[test]
    fn test_format_error_is_error() {
        let error = FormatError::Unimplemented(Format::Unknown);
        // This should compile if `FormatError` implements `std::error::Error`
        let _boxed: Box<dyn Error> = Box::new(error);
    }

    /// Test `Format::format` returns error for `Pointer` format
    #[test]
    fn test_format_error_pointer_with_prefix() {
        let result = Format::Pointer.format(0x42, true);
        assert!(result.is_err());
        match result {
            Err(FormatError::Unimplemented(Format::Pointer)) => {
                // Expected error
            }
            _ => panic!("Expected FormatError::Unimplemented(Format::Pointer)"),
        }
    }

    /// Test `Format::format` returns error for `Pointer` format without prefix
    #[test]
    fn test_format_error_pointer_without_prefix() {
        let result = Format::Pointer.format(0x42, false);
        assert!(result.is_err());
        match result {
            Err(FormatError::Unimplemented(Format::Pointer)) => {
                // Expected error
            }
            _ => panic!("Expected FormatError::Unimplemented(Format::Pointer)"),
        }
    }

    /// Test `Format::format` returns error for `LowerExp` format
    #[test]
    fn test_format_error_lower_exp() {
        let result = Format::LowerExp.format(0xFF, true);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FormatError::Unimplemented(Format::LowerExp))
        ));
    }

    /// Test `Format::format` returns error for `UpperExp` format
    #[test]
    fn test_format_error_upper_exp() {
        let result = Format::UpperExp.format(0xFF, false);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FormatError::Unimplemented(Format::UpperExp))
        ));
    }

    /// Test `Format::format` returns error for `Unknown` format
    #[test]
    fn test_format_error_unknown() {
        let result = Format::Unknown.format(0x00, true);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FormatError::Unimplemented(Format::Unknown))
        ));
    }

    /// Test all unimplemented formats return errors
    #[test]
    fn test_all_unimplemented_formats() {
        let unimplemented = vec![
            Format::Pointer,
            Format::LowerExp,
            Format::UpperExp,
            Format::Unknown,
        ];

        for format in unimplemented {
            let result_with_prefix = format.format(0x42, true);
            let result_without_prefix = format.format(0x42, false);

            assert!(
                result_with_prefix.is_err(),
                "Format {:?} should return error with prefix",
                format
            );
            assert!(
                result_without_prefix.is_err(),
                "Format {:?} should return error without prefix",
                format
            );
        }
    }

    /// Test all implemented formats return Ok
    #[test]
    fn test_all_implemented_formats() {
        let implemented = vec![
            Format::Octal,
            Format::LowerHex,
            Format::UpperHex,
            Format::Binary,
        ];

        for format in implemented {
            let result_with_prefix = format.format(0x42, true);
            let result_without_prefix = format.format(0x42, false);

            assert!(
                result_with_prefix.is_ok(),
                "Format {:?} should return Ok with prefix",
                format
            );
            assert!(
                result_without_prefix.is_ok(),
                "Format {:?} should return Ok without prefix",
                format
            );
        }
    }

    /// Test format with edge case values
    #[test]
    fn test_format_edge_cases() {
        // Test zero
        assert_eq!(Format::LowerHex.format(0, true).unwrap(), "0x00");
        assert_eq!(Format::LowerHex.format(0, false).unwrap(), "00");
        assert_eq!(Format::Octal.format(0, true).unwrap(), "0o0000");
        assert_eq!(Format::Octal.format(0, false).unwrap(), "0000");
        assert_eq!(Format::Binary.format(0, true).unwrap(), "0b00000000");
        assert_eq!(Format::Binary.format(0, false).unwrap(), "00000000");

        // Test maximum value
        assert_eq!(Format::LowerHex.format(u8::MAX, true).unwrap(), "0xff");
        assert_eq!(Format::LowerHex.format(u8::MAX, false).unwrap(), "ff");
        assert_eq!(Format::UpperHex.format(u8::MAX, true).unwrap(), "0xFF");
        assert_eq!(Format::UpperHex.format(u8::MAX, false).unwrap(), "FF");
    }
}
