//! Data structures for hex representation
//!
//! This module provides the core data structures used to represent hexadecimal
//! output in a structured format. The `Line` struct represents a single line
//! of hex output, while the `Page` struct represents a collection of lines
//! forming a complete page of hex data.
//!
//! # Examples
//!
//! ```rust
//! use hx::{Line, Page};
//!
//! // Create a new line
//! let mut line = Line::new();
//! line.hex_body.push(0x42);
//! line.ascii.push(b'B');
//! line.bytes = 1;
//!
//! // Create a page with lines
//! let mut page = Page::new();
//! page.body.push(line);
//! page.bytes = 1;
//! ```

/// Line structure for hex output
///
/// A `Line` represents a single line of hexadecimal output, containing:
/// - The offset (memory address) for this line
/// - The hexadecimal representation of bytes
/// - The ASCII representation of bytes
/// - The total byte count for this line
///
/// # Examples
///
/// ```rust
/// use hx::Line;
///
/// let mut line = Line::new();
/// line.hex_body.push(0x48);
/// line.hex_body.push(0x65);
/// line.hex_body.push(0x6C);
/// line.ascii.push(b'H');
/// line.ascii.push(b'e');
/// line.ascii.push(b'l');
/// line.bytes = 3;
/// line.offset = 0x1000;
/// ```
#[derive(Clone, Debug, Default)]
pub struct Line {
    /// Memory offset for this line (displayed as hexadecimal address)
    pub offset: u64,
    /// Hexadecimal representation of bytes in this line
    pub hex_body: Vec<u8>,
    /// ASCII representation of bytes in this line (non-printable chars shown as '.')
    pub ascii: Vec<u8>,
    /// Total number of bytes represented in this line
    pub bytes: u64,
}

/// Line implementation
impl Line {
    /// Creates a new empty `Line` with default values
    ///
    /// All fields are initialized to zero or empty vectors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hx::Line;
    ///
    /// let line = Line::new();
    /// assert_eq!(line.offset, 0);
    /// assert_eq!(line.hex_body.len(), 0);
    /// assert_eq!(line.ascii.len(), 0);
    /// assert_eq!(line.bytes, 0);
    /// ```
    pub fn new() -> Line {
        Line {
            offset: 0x0,
            hex_body: Vec::new(),
            ascii: Vec::new(),
            bytes: 0x0,
        }
    }
}

/// Page structure for hex output
///
/// A `Page` represents a collection of `Line` structures, forming a complete
/// page of hexadecimal output. This structure is used to organize and process
/// multiple lines of hex data together.
///
/// # Examples
///
/// ```rust
/// use hx::{Line, Page};
///
/// let mut page = Page::new();
/// let mut line1 = Line::new();
/// line1.hex_body.push(0x01);
/// line1.bytes = 1;
///
/// let mut line2 = Line::new();
/// line2.hex_body.push(0x02);
/// line2.bytes = 1;
///
/// page.body.push(line1);
/// page.body.push(line2);
/// page.bytes = 2;
/// ```
#[derive(Clone, Debug, Default)]
pub struct Page {
    /// Starting memory offset for this page
    pub offset: u64,
    /// Collection of `Line` structures forming the page content
    pub body: Vec<Line>,
    /// Total number of bytes represented across all lines in this page
    pub bytes: u64,
}

/// Page implementation
impl Page {
    /// Creates a new empty `Page` with default values
    ///
    /// All fields are initialized to zero or empty vectors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hx::Page;
    ///
    /// let page = Page::new();
    /// assert_eq!(page.offset, 0);
    /// assert_eq!(page.body.len(), 0);
    /// assert_eq!(page.bytes, 0);
    /// ```
    pub fn new() -> Page {
        Page {
            offset: 0x0,
            body: Vec::new(),
            bytes: 0x0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Line struct initialization and field access
    #[test]
    fn test_line_struct() {
        let mut ascii_line: Line = Line::new();
        ascii_line.ascii.push(b'.');
        assert_eq!(ascii_line.ascii[0], b'.');
        assert_eq!(ascii_line.offset, 0x0);
    }

    /// Test Line default values
    #[test]
    fn test_line_default() {
        let line = Line::new();
        assert_eq!(line.offset, 0x0);
        assert_eq!(line.hex_body.len(), 0);
        assert_eq!(line.ascii.len(), 0);
        assert_eq!(line.bytes, 0x0);
    }

    /// Test Line with data
    #[test]
    fn test_line_with_data() {
        let mut line = Line::new();
        line.hex_body.push(0x42);
        line.hex_body.push(0x43);
        line.ascii.push(b'B');
        line.ascii.push(b'C');
        line.bytes = 2;
        line.offset = 0x100;

        assert_eq!(line.hex_body.len(), 2);
        assert_eq!(line.hex_body[0], 0x42);
        assert_eq!(line.hex_body[1], 0x43);
        assert_eq!(line.ascii.len(), 2);
        assert_eq!(line.ascii[0], b'B');
        assert_eq!(line.ascii[1], b'C');
        assert_eq!(line.bytes, 2);
        assert_eq!(line.offset, 0x100);
    }

    /// Test Page default values
    #[test]
    fn test_page_default() {
        let page = Page::new();
        assert_eq!(page.offset, 0x0);
        assert_eq!(page.body.len(), 0);
        assert_eq!(page.bytes, 0x0);
    }

    /// Test Page with lines
    #[test]
    fn test_page_with_lines() {
        let mut page = Page::new();
        let mut line1 = Line::new();
        line1.hex_body.push(0x01);
        line1.bytes = 1;
        let mut line2 = Line::new();
        line2.hex_body.push(0x02);
        line2.bytes = 1;

        page.body.push(line1);
        page.body.push(line2);
        page.bytes = 2;
        page.offset = 0x100;

        assert_eq!(page.body.len(), 2);
        assert_eq!(page.bytes, 2);
        assert_eq!(page.offset, 0x100);
    }
}
