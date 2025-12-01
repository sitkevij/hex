//! Buffer processing and conversion utilities
//!
//! This module provides functions for reading binary data from buffers and
//! converting it into structured page and line formats suitable for hex
//! display. The conversion handles line wrapping based on column width and
//! optional truncation.
//!
//! # Examples
//!
//! ```rust
//! use hx::buf_to_array;
//! use std::io::Cursor;
//!
//! let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
//! let mut buf = Cursor::new(data);
//! let page = buf_to_array(&mut buf, 0, 4).unwrap();
//! assert_eq!(page.bytes, 5);
//! assert_eq!(page.body.len(), 2); // 4 bytes + 1 byte on second line
//! ```
#![allow(clippy::unbuffered_bytes)]
use crate::models::{Line, Page};
use std::error::Error;
use std::io::Read;

/// Converts a readable buffer into a structured `Page` with formatted lines
///
/// Reads bytes from the provided buffer and organizes them into a `Page`
/// structure containing multiple `Line` structures. Each line contains up to
/// `column_width` bytes. The function handles optional truncation and respects
/// the maximum array size limit.
///
/// # Line Wrapping
///
/// Bytes are grouped into lines based on `column_width`. When a line reaches
/// the column width, it's added to the page and a new line is started. The
/// final line (even if incomplete) is always added to the page.
///
/// # Truncation
///
/// If `buf_len` is greater than 0, reading stops when either:
/// - The specified number of bytes (`buf_len`) have been read
/// - The maximum array size (`u16::MAX`) is reached
///
/// # Arguments
///
/// * `buf` - A mutable reference to a type implementing `Read` containing the data to process
/// * `buf_len` - Maximum number of bytes to read (0 = read all available bytes)
/// * `column_width` - Number of bytes per line in the output (determines line wrapping)
///
/// # Returns
///
/// Returns `Ok(Page)` containing the structured data, or an error if reading fails.
///
/// The returned `Page` contains:
/// - `bytes`: Total number of bytes read
/// - `body`: Vector of `Line` structures, each containing up to `column_width` bytes
///
/// # Errors
///
/// This function will return an error if:
/// - Reading from the buffer fails (I/O error)
/// - The buffer is corrupted or unreadable
///
/// # Examples
///
/// ```rust
/// use hx::buf_to_array;
/// use std::io::Cursor;
///
/// // Read all bytes with 8-byte column width
/// let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
/// let mut buf = Cursor::new(data);
/// let page = buf_to_array(&mut buf, 0, 8).unwrap();
/// assert_eq!(page.bytes, 9);
/// assert_eq!(page.body.len(), 2); // 8 bytes + 1 byte
///
/// // Truncate to 5 bytes
/// let mut buf2 = Cursor::new(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
/// let page2 = buf_to_array(&mut buf2, 5, 10).unwrap();
/// assert_eq!(page2.bytes, 5);
///
/// // Empty buffer
/// let mut buf3 = Cursor::new(vec![]);
/// let page3 = buf_to_array(&mut buf3, 0, 10).unwrap();
/// assert_eq!(page3.bytes, 0);
/// assert_eq!(page3.body.len(), 1); // Always has at least one line
/// ```
pub fn buf_to_array(
    buf: &mut dyn Read,
    buf_len: u64,
    column_width: u64,
) -> Result<Page, Box<dyn Error>> {
    let mut column_count: u64 = 0x0;
    let max_array_size: u16 = <u16>::MAX; // 2^16;
    let mut page: Page = Page::new();
    let mut line: Line = Line::new();
    for b in buf.bytes() {
        let b1: u8 = b?;
        line.bytes += 1;
        page.bytes += 1;
        line.hex_body.push(b1);
        column_count += 1;

        if column_count >= column_width {
            page.body.push(line);
            line = Line::new();
            column_count = 0;
        }

        if buf_len > 0 && (page.bytes == buf_len || u64::from(max_array_size) == buf_len) {
            break;
        }
    }
    page.body.push(line);
    Ok(page)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    /// Test `buf_to_array` with empty buffer
    #[test]
    fn test_buf_to_array_empty() {
        let mut buf = io::Cursor::new(vec![]);
        let page = buf_to_array(&mut buf, 0, 10).unwrap();
        assert_eq!(page.bytes, 0);
        assert_eq!(page.body.len(), 1); // Always has at least one line
    }

    /// Test `buf_to_array` with exact column width
    #[test]
    fn test_buf_to_array_exact_column() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut buf = io::Cursor::new(data);
        let page = buf_to_array(&mut buf, 0, 10).unwrap();
        assert_eq!(page.bytes, 10);
        assert_eq!(page.body.len(), 2); // One full line + trailing line
    }

    /// Test `buf_to_array` with truncation
    #[test]
    fn test_buf_to_array_truncation() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut buf = io::Cursor::new(data);
        let page = buf_to_array(&mut buf, 5, 10).unwrap();
        assert_eq!(page.bytes, 5);
    }

    /// Test `buf_to_array` with multiple lines
    #[test]
    fn test_buf_to_array_multiple_lines() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut buf = io::Cursor::new(data);
        let page = buf_to_array(&mut buf, 0, 5).unwrap();
        assert_eq!(page.bytes, 12);
        assert_eq!(page.body.len(), 3); // 5 + 5 + 2
    }

    /// Test buf_to_array with single byte
    #[test]
    fn test_buf_to_array_single_byte() {
        let data = vec![0x42];
        let mut buf = io::Cursor::new(data);
        let page = buf_to_array(&mut buf, 0, 10).unwrap();
        assert_eq!(page.bytes, 1);
        assert_eq!(page.body.len(), 1);
        assert_eq!(page.body[0].hex_body[0], 0x42);
    }

    /// Test buf_to_array with column width of 1
    #[test]
    fn test_buf_to_array_column_width_one() {
        let data = vec![1, 2, 3];
        let mut buf = io::Cursor::new(data);
        let page = buf_to_array(&mut buf, 0, 1).unwrap();
        assert_eq!(page.bytes, 3);
        assert_eq!(page.body.len(), 4); // 3 lines + trailing empty line
    }

    /// Test buf_to_array preserves data integrity
    #[test]
    fn test_buf_to_array_data_integrity() {
        let data: Vec<u8> = (0..16).collect();
        let mut buf = io::Cursor::new(data.clone());
        let page = buf_to_array(&mut buf, 0, 8).unwrap();

        let mut reconstructed: Vec<u8> = Vec::new();
        for line in page.body.iter() {
            reconstructed.extend(&line.hex_body);
        }

        assert_eq!(reconstructed, data);
    }
}
