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

//! general hex lib
extern crate ansi_term;
extern crate clap;

use clap::ArgMatches;
use no_color::is_no_color;
use std::env;
use std::error::Error;
use std::f64;
use std::fs;
use std::io::BufReader;
use std::io::IsTerminal;
use std::io::{self, BufRead, Read, Write};

/// arg cols
pub const ARG_COL: &str = "cols";
/// arg len
pub const ARG_LEN: &str = "len";
/// arg format
pub const ARG_FMT: &str = "format";
/// arg INPUTFILE
pub const ARG_INP: &str = "INPUTFILE";
/// arg color
pub const ARG_CLR: &str = "color";
/// arg array
pub const ARG_ARR: &str = "array";
/// arg func
pub const ARG_FNC: &str = "func";
/// arg places
pub const ARG_PLC: &str = "places";
/// arg prefix
pub const ARG_PFX: &str = "prefix";

const ARGS: [&str; 9] = [
    ARG_COL, ARG_LEN, ARG_FMT, ARG_INP, ARG_CLR, ARG_ARR, ARG_FNC, ARG_PLC, ARG_PFX,
];

const DBG: u8 = 0x0;

/// nothing ⇒ Display
/// ? ⇒ Debug
/// o ⇒ Octal
/// x ⇒ LowerHex
/// X ⇒ UpperHex
/// p ⇒ Pointer
/// b ⇒ Binary
/// e ⇒ LowerExp
/// E ⇒ UpperExp
/// evaluate for traits implementation
#[derive(Copy, Clone, Debug)]
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
    /// Formats a given u8 according to the base Format
    ///
    /// # Arguments
    ///
    /// * `data` - The byte to be formatted
    /// * `prefix` - whether or not to add a prefix
    fn format(&self, data: u8, prefix: bool) -> String {
        if prefix {
            match &self {
                Self::Octal => format!("{:#06o}", data),
                Self::LowerHex => format!("{:#04x}", data),
                Self::UpperHex => format!("{:#04X}", data),
                Self::Binary => format!("{:#010b}", data),
                _ => panic!("format is not implemented for this Format"),
            }
            .to_string()
        } else {
            match &self {
                Self::Octal => format!("{:04o}", data),
                Self::LowerHex => format!("{:02x}", data),
                Self::UpperHex => format!("{:02X}", data),
                Self::Binary => format!("{:08b}", data),
                _ => panic!("format is not implemented for this Format"),
            }
            .to_string()
        }
    }
}

/// Line structure for hex output
#[derive(Clone, Debug, Default)]
pub struct Line {
    /// offset
    pub offset: u64,
    /// hex body
    pub hex_body: Vec<u8>,
    /// ascii text
    pub ascii: Vec<u8>,
    /// total bytes in Line
    pub bytes: u64,
}
/// Line implementation
impl Line {
    /// Line constructor
    pub fn new() -> Line {
        Line {
            offset: 0x0,
            hex_body: Vec::new(),
            ascii: Vec::new(),
            bytes: 0x0,
        }
    }
}

/// Page structure
#[derive(Clone, Debug, Default)]
pub struct Page {
    /// page offset
    pub offset: u64,
    /// page body
    pub body: Vec<Line>,
    /// total bytes in page
    pub bytes: u64,
}

/// Page implementation
impl Page {
    /// Page constructor
    pub fn new() -> Page {
        Page {
            offset: 0x0,
            body: Vec::new(),
            bytes: 0x0,
        }
    }
}

/// offset column
///
/// # Arguments
///
/// * `b` - offset value.
pub fn offset(b: u64) -> String {
    format!("{:#08x}", b)
}

/// print offset to std out
pub fn print_offset(w: &mut impl Write, b: u64) -> io::Result<()> {
    write!(w, "{}: ", offset(b))
}

/// print byte to std out
pub fn print_byte(
    w: &mut impl Write,
    b: u8,
    format: Format,
    colorize: bool,
    prefix: bool,
) -> io::Result<()> {
    let fmt_string = format.format(b, prefix);
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

/// get the color for a specific byte
pub fn byte_to_color(b: u8) -> u8 {
    let mut color: u8 = b;
    if color < 1 {
        color = 0x16;
    }
    color
}

/// append char representation of a byte to a buffer
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

/// In most hex editor applications, the data of the computer file is
/// represented as hexadecimal values grouped in 4 groups of 4 bytes (or
/// two groups of 8 bytes), followed by one group of 16 printable ASCII
/// characters which correspond to each pair of hex values (each byte).
/// Non-printable ASCII characters (e.g., Bell) and characters that would take
/// more than one character space (e.g., tab) are typically represented by a
/// dot (".") in the following ASCII field.
///
/// # Arguments
///
/// * `matches` - Argument matches from command line.
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
        output_function(len.parse::<u64>().unwrap(), p);
    } else {
        // cases:
        //  $ cat Cargo.toml | target/debug/hx
        //  $ cat Cargo.toml | target/debug/hx -a r
        //  $ target/debug/hx Cargo.toml
        //  $ target/debug/hx Cargo.toml -a r
        let is_stdin = is_stdin(matches.clone());
        let mut buf: Box<dyn BufRead> = if is_stdin.unwrap() {
            Box::new(BufReader::new(io::stdin()))
        } else {
            Box::new(BufReader::new(fs::File::open(
                matches.get_one::<String>(ARG_INP).unwrap(),
            )?))
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
            colorize = color.parse::<u8>().unwrap() == 1;
        }

        if let Some(prefix_flag) = matches.get_one::<String>(ARG_PFX) {
            prefix = prefix_flag.parse::<u8>().unwrap() == 1;
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

/// Detect stdin, file path and/or parameters.
/// # Arguments
///
/// * `matches` - argument matches.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn is_stdin(matches: ArgMatches) -> Result<bool, Box<dyn Error>> {
    let mut is_stdin = false;
    if let Some(file) = matches.get_one::<String>(ARG_INP) {
        if DBG > 0 {
            dbg!(file);
        }
        is_stdin = false;
    } else if let Some(nth1) = env::args().nth(1) {
        if DBG > 0 {
            dbg!(nth1);
        }
        is_stdin = ARGS.iter().any(|arg| matches.index_of(arg) == Some(2));
    } else if !matches.args_present() {
        is_stdin = true;
    }
    if DBG > 0 {
        dbg!(is_stdin);
    }
    Ok(is_stdin)
}

/// Output source code array format.
/// # Arguments
///
/// * `array_format` - array format, rust (r), C (c), golang (g).
/// * `buf` - BufRead.
/// * `truncate_len` - truncate to length.
/// * `column_width` - column width.
pub fn output_array(
    array_format: &str,
    mut buf: Box<dyn BufRead>,
    truncate_len: u64,
    column_width: u64,
) -> io::Result<()> {
    let stdout = io::stdout();
    let mut locked = stdout.lock();

    let page = buf_to_array(&mut buf, truncate_len, column_width).unwrap();
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
            if i == page.bytes && array_format != "g" {
                if array_format != "f" {
                    write!(locked, "{}", Format::LowerHex.format(*hex, true))?;
                } else {
                    write!(locked, "{}uy", Format::LowerHex.format(*hex, true))?;
                }
            } else if array_format != "f" {
                write!(locked, "{}, ", Format::LowerHex.format(*hex, true))?;
            } else {
                write!(locked, "{}uy; ", Format::LowerHex.format(*hex, true))?;
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
}

/// Function wave out.
/// # Arguments
///
/// * `len` - Wave length.
/// * `places` - Number of decimal places for function wave floats.
pub fn output_function(len: u64, places: usize) {
    for y in 0..len {
        let y_float: f64 = y as f64;
        let len_float: f64 = len as f64;
        let x: f64 = (((y_float / len_float) * f64::consts::PI) / 2.0).sin();
        let formatted_number = format!("{:.*}", places, x);
        print!("{}", formatted_number);
        print!(",");
        if (y % 10) == 9 {
            println!();
        }
    }
    println!();
}

/// Buffer to array.
///
/// # Arguments
///
/// * `buf` - Buffer to be read.
/// * `buf_len` - force buffer length.
/// * `column_width` - column width for output.
pub fn buf_to_array(
    buf: &mut dyn Read,
    buf_len: u64,
    column_width: u64,
) -> Result<Page, Box<dyn ::std::error::Error>> {
    let mut column_count: u64 = 0x0;
    let max_array_size: u16 = <u16>::max_value(); // 2^16;
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
        assert.success().code(0).stdout(
            "0x000000: 0x30 0x31 0x32                                    012\n   bytes: 3\n",
        );
    }
}
