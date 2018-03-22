extern crate clap;
mod lib;
use clap::{App, Arg};
use std::process;

/// Central application entry point.
fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
       .version(env!("CARGO_PKG_VERSION"))
       .about(env!("CARGO_PKG_DESCRIPTION")) // CARGO_PKG_HOMEPAGE
       .author(env!("CARGO_PKG_AUTHORS"))
       .arg(Arg::with_name("cols")
                    .short("c")
                    .long("cols")
                    .value_name("columns")
                    .help("Sets the column length")
                    .takes_value(true))
        .arg(Arg::with_name("len")
                    .short("l")
                    .long("len")
                    .value_name("len")
                    .help("Stop after <len> bytes")
                    .takes_value(true))
        .arg(Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .help("Format of octet, possible values: o, x, X, p, b, e, E")
                    .takes_value(true))
       .arg(Arg::with_name("INPUTFILE")
                    .help("Pass file path as an argument for hex dump")
                    .required(true)
                    .index(1))
        .arg(Arg::with_name("v")
                    .short("v")
                    .multiple(true)
                    .help("Sets verbosity level"))
        .arg(Arg::with_name("array")
                    .short("a")
                    .long("array")
                    .value_name("array_format")
                    .help("Output array in source code format: -a<value> accepts values `r` for rust, `c` for C, `g` for golang.")
                    .takes_value(true))
       .get_matches();

    match lib::run(matches) {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("error = \"{}\"", e);
            process::exit(1);
        }
    }
}