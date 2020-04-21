extern crate clap;
mod lib;
use clap::{App, Arg};
use std::env;
use std::process;
use lib::{ARG_COL, ARG_LEN, ARG_FMT, ARG_INP, ARG_CLR, ARG_ARR, ARG_FNC, ARG_PLC};

/// Central application entry point.
fn main() {
    let desc: &str = &format!(
        "{}\n{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_HOMEPAGE")
    );
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(desc)
        .arg(
            Arg::with_name(ARG_COL)
                .short("c")
                .long(ARG_COL)
                .value_name("columns")
                .help("Set column length")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_LEN)
                .short("l")
                .long(ARG_LEN)
                .value_name(ARG_LEN)
                .help("Set <len> bytes to read")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_FMT)
                .short("f")
                .long(ARG_FMT)
                .help("Set format of octet: Octal (o), LowerHex (x), UpperHex (X), Binary (b)")
                .possible_values(&["o", "x", "X", "b"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_INP)
                .help("Pass file path as an argument for hex dump")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name(ARG_CLR)
                .short("t")
                .long(ARG_CLR)
                .help("Set color tint terminal output. 0 to disable, 1 to enable")
                // .default_value("1")
                .possible_values(&["0", "1"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_ARR)
                .short("a")
                .long(ARG_ARR)
                .value_name("array_format")
                .help("Set source code format output: rust (r), C (c), golang (g)")
                .possible_values(&["r", "c", "g"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_FNC)
                .short("u")
                .long(ARG_FNC)
                .value_name("func_length")
                .help("Set function wave length")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_PLC)
                .short("p")
                .long(ARG_PLC)
                .value_name("func_places")
                .help("Set function wave output decimal places")
                .takes_value(true),
        );

    // disable 1 arg check for stdin
    // let args: Vec<_> = env::args().collect();
    // if args.len() == 1 {
    //     app.clone().print_help().unwrap();
    //     println!();
    //     println!();
    //     process::exit(0);
    // }

    let matches = app.get_matches();
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
