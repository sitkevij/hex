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
// #![allow(dead_code)]

//! general hex lib
extern crate ansi_term;
extern crate clap;

use clap::ArgMatches;
use std::f64;
use std::fs;
// use std::fs::File;
use std::io::BufReader;
// use std::io::Read;
// use std::io::{self, Read};
use std::io::{self, BufRead, Read};
use std::env;
use std::error::Error;

/// arg cols
pub const ARG_COL: &str = "cols";
/// arg len
pub const ARG_LEN: &str = "len";
/// arg format
pub const ARG_FMT: &str = "format";
/// arg INPUTFILE
pub const ARG_INP: &str = "INPUTFILE";
/// arg v
// pub const ARG_VRB: &str = "v";
/// arg color
pub const ARG_CLR: &str = "color";
/// arg array
pub const ARG_ARR: &str = "array";
/// arg func
pub const ARG_FNC: &str = "func";
/// arg places
pub const ARG_PLC: &str = "places";

const ARGS: [&str; 8] = [ARG_COL, ARG_LEN, ARG_FMT, ARG_INP, 
    ARG_CLR, ARG_ARR, ARG_FNC, ARG_PLC];

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
/// evaulate for traits implementation
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

/// Line structure for hex output
#[derive(Clone, Debug, Default)]
pub struct Line {
    /// offset
    pub offset: u64,
    /// hex body
    pub hex_body: Vec<u8>,
    /// ascii text
    pub ascii: Vec<char>,
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
pub fn print_offset(b: u64) {
    print!("{}: ", offset(b));
}

/// hex octal, takes u8
pub fn hex_octal(b: u8) -> String {
    format!("{:#06o}", b)
}

/// hex lower hex, takes u8
pub fn hex_lower_hex(b: u8) -> String {
    format!("{:#04x}", b)
}

/// hex upper hex, takes u8
pub fn hex_upper_hex(b: u8) -> String {
    format!("{:#04X}", b)
}

/// hex binary, takes u8
pub fn hex_binary(b: u8) -> String {
    format!("{:#010b}", b)
}

/// print byte to std out
pub fn print_byte(b: u8, format: Format, colorize: bool) {
    let mut color: u8 = b;
    if color < 1 {
        color = 0x16;
    }
    if colorize {
        // note, for color testing: for (( i = 0; i < 256; i++ )); do echo "$(tput setaf $i)This is ($i) $(tput sgr0)"; done
        match format {
            Format::Octal => print!(
                "{} ",
                ansi_term::Style::new()
                    .fg(ansi_term::Color::Fixed(color))
                    .paint(hex_octal(b))
            ),
            Format::LowerHex => print!(
                "{} ",
                ansi_term::Style::new()
                    .fg(ansi_term::Color::Fixed(color))
                    .paint(hex_lower_hex(b))
            ),
            Format::UpperHex => print!(
                "{} ",
                ansi_term::Style::new()
                    .fg(ansi_term::Color::Fixed(color))
                    .paint(hex_upper_hex(b))
            ),
            Format::Binary => print!(
                "{} ",
                ansi_term::Style::new()
                    .fg(ansi_term::Color::Fixed(color))
                    .paint(hex_binary(b))
            ),
            _ => print!("unk_fmt "),
        }
    } else {
        match format {
            Format::Octal => print!("{} ", hex_octal(b)),
            Format::LowerHex => print!("{} ", hex_lower_hex(b)),
            Format::UpperHex => print!("{} ", hex_upper_hex(b)),
            Format::Binary => print!("{} ", hex_binary(b)),
            _ => print!("unk_fmt "),
        }
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
    if let Some(len) = matches.value_of("func") {
        let mut p: usize = 4;
        if let Some(places) = matches.value_of("places") {
            p = places.parse::<usize>().unwrap();
        }
        output_function(len.parse::<u64>().unwrap(), p);
    // } else if let Some(_file) = matches.value_of("INPUTFILE") {
    } else {
        // cases:
        //  $ cat Cargo.toml | target/debug/hx
        //  $ cat Cargo.toml | target/debug/hx -a r
        //  $ target/debug/hx Cargo.toml
        //  $ target/debug/hx Cargo.toml -a r
        // steps:
        //  - match the possible args, to determine if input file in nth 1 arg
        //  (index 2) position.
        let is_stdin = is_stdin(matches.clone());

        // let mut buf: Box<dyn BufRead> = match input {
        //     None => Box::new(BufReader::new(io::stdin())),
        //     // Some("-".to_string()) => Box::new(BufReader::new(io::stdin())),
        //     Some(_file) => Box::new(BufReader::new(fs::File::open(_file).unwrap()))
        // };
        // let input = env::args().nth(1);
        let mut buf: Box<dyn BufRead> = match is_stdin.unwrap() {
            true => Box::new(BufReader::new(io::stdin())),
            false => Box::new(BufReader::new(fs::File::open(matches.value_of(ARG_INP).unwrap()).unwrap()))
        };
        // let f = File::open(file).unwrap();
        // let mut buf_len = fs::metadata(file)?.len();
        // let mut buf_len = 32;
        // let mut buf = BufReader::new(f);
        let mut format_out = Format::LowerHex;
        let mut colorize = true;

        if let Some(columns) = matches.value_of(ARG_COL) {
            column_width = columns.parse::<u64>().unwrap(); //turbofish
        }

        if let Some(_length) = matches.value_of(ARG_LEN) {
            // buf_len = length.parse::<u64>().unwrap();
        }

        if let Some(format) = matches.value_of(ARG_FMT) {
            // o, x, X, p, b, e, E
            match format {
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

        if let Some(color) = matches.value_of(ARG_CLR) {
            let color_v = color.parse::<u8>().unwrap();
            if color_v == 1 {
                colorize = true;
            } else {
                colorize = false;
            }
        }

        // array output mode is mutually exclusive
        if let Some(array) = matches.value_of(ARG_ARR) {
            output_array(array, buf, column_width);
        } else {
            // Transforms this Read instance to an Iterator over its bytes.
            // The returned type implements Iterator where the Item is
            // Result<u8, R::Err>. The yielded item is Ok if a byte was
            // successfully read and Err otherwise for I/O errors. EOF is mapped
            // to returning None from this iterator.
            // (https://doc.rust-lang.org/1.16.0/std/io/trait.Read.html#method.bytes)
            let mut ascii_line: Line = Line::new();
            let mut offset_counter: u64 = 0x0;
            let mut byte_column: u64 = 0x0;
            let page = buf_to_array(&mut buf, column_width).unwrap();

            for line in page.body.iter() {
                print_offset(offset_counter);

                for hex in line.hex_body.iter() {
                    offset_counter += 1;
                    byte_column += 1;
                    print_byte(*hex, format_out, colorize);

                    if *hex > 31 && *hex < 127 {
                        ascii_line.ascii.push(*hex as char);
                    } else {
                        ascii_line.ascii.push('.');
                    }
                }

                if byte_column < column_width {
                    print!("{:<1$}", "", 5 * (column_width - byte_column) as usize);
                }

                byte_column = 0x0;
                let ascii_string: String = ascii_line.ascii.iter().cloned().collect();
                ascii_line = Line::new();
                print!("{}", ascii_string); // print ascii string
                println!();
            }
            if true {
                println!("   bytes: {}", page.bytes);
            }
        }
    }
    Ok(())
}

/// is_stdin
pub fn is_stdin(matches: ArgMatches) -> Result<bool, Box<dyn Error>> {
    let mut is_stdin = false;
    if DBG > 0 { 
        dbg!(env::args().len(), matches.args.len());
        dbg!(env::args().nth(0).unwrap());
    }
    if let Some(nth1) = env::args().nth(1) {
        if DBG > 0 { dbg!(nth1); }
        for arg in ARGS.iter() {
            if let Some(index) = matches.index_of(arg) {
                if DBG > 0 { dbg!("matches.index_of(arg) {0} {1}", arg, index); }
                match index {
                    2 => is_stdin = true,
                    _ => println!("arg index {}", index),
                }
            }
        }
    // } else if matches.args.len() == 0 {
    } else if matches.args.is_empty() {
        is_stdin = true;
    } else if let Some(file) = matches.value_of(ARG_INP) {
        if DBG > 0 { dbg!(file); }
        is_stdin = false;
    }
    if DBG > 0 { dbg!(is_stdin); }
    Ok(is_stdin)
}

/// Output source code array format.
/// # Arguments
///
/// * `array_format` - array format, rust (r), C (c), golang (g).
/// * `buf` - BufRead.
/// * `column_width` - column width.
pub fn output_array(array_format: &str, mut buf: Box<dyn BufRead>, column_width: u64) {
    let page = buf_to_array(&mut buf, column_width).unwrap();
    match array_format {
        "r" => println!("let ARRAY: [u8; {}] = [", page.bytes),
        "c" => println!("unsigned char ARRAY[{}] = {{", page.bytes),
        "g" => println!("a := [{}]byte{{", page.bytes),
        _ => println!("unknown array format"),
    }
    let mut i:u64 = 0x0;
    for line in page.body.iter() {
        print!("    ");
        for hex in line.hex_body.iter() {
            i += 1;
            if i == page.bytes && array_format != "g" {
                print!("{}", hex_lower_hex(*hex));
            } else {
                print!("{}, ", hex_lower_hex(*hex));
            }
        }
        println!();
    }
    match array_format {
        "r" => println!("];"),
        "c" => println!("}};"),
        "g" => println!("}}"),
        _ => println!("unknown array format"),
    }
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
/// * `buf_len` - Buffer length.
/// * `column_width` - column width for output.
pub fn buf_to_array(
    buf: &mut dyn Read,
    // buf_len: u64,
    column_width: u64,
) -> Result<Page, Box<dyn ::std::error::Error>> {
    let mut column_count: u64 = 0x0;
    // let max_array_size: u16 = <u16>::max_value(); // 2^16;
    let mut page: Page = Page::new();
    let mut line: Line = Line::new();
    for b in buf.bytes() {
        let b1: u8 = b.unwrap();
        line.bytes += 1;
        page.bytes += 1;
        line.hex_body.push(b1);
        column_count += 1;

        if column_count >= column_width {
            page.body.push(line);
            line = Line::new();
            column_count = 0;
        }

        // if page.bytes == buf_len || u64::from(max_array_size) == buf_len {
        //     page.body.push(line);
        //     break;
        // }
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
        assert_eq!(hex_octal(b), "0o0006");
        assert_eq!(hex_octal(b), format!("{:#06o}", b));
    }

    /// hex lower hex, takes u8
    #[test]
    fn test_hex_lower_hex() {
        let b: u8 = <u8>::max_value(); // 255
        assert_eq!(hex_lower_hex(b), "0xff");
        assert_eq!(hex_lower_hex(b), format!("{:#04x}", b));
    }

    /// hex upper hex, takes u8
    #[test]
    fn test_hex_upper_hex() {
        let b: u8 = <u8>::max_value();
        assert_eq!(hex_upper_hex(b), "0xFF");
        assert_eq!(hex_upper_hex(b), format!("{:#04X}", b));
    }

    /// hex binary, takes u8
    #[test]
    fn test_hex_binary() {
        let b: u8 = <u8>::max_value();
        assert_eq!(hex_binary(b), "0b11111111");
        assert_eq!(hex_binary(b), format!("{:#010b}", b));
    }
}
