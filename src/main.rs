extern crate clap;
mod lib;
use clap::{App, Arg};
use std::env;
use std::process;

/// Central application entry point.
fn main() {
    let desc: &str = &format!(
        "{}\n{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_HOMEPAGE")
    )
    .to_string();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(desc)
        .arg(
            Arg::with_name("cols")
                .short("c")
                .long("cols")
                .value_name("columns")
                .help("Set column length")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("len")
                .short("l")
                .long("len")
                .value_name("len")
                .help("Set <len> bytes to read")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Set format of octet: Octal (o), LowerHex (x), UpperHex (X), Binary (b)")
                .possible_values(&["o", "x", "X", "b"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUTFILE")
                .help("Pass file path as an argument for hex dump")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets verbosity level"),
        )
        .arg(
            Arg::with_name("color")
                .short("t")
                .long("color")
                .help("Set color tint terminal output. 0 to disable, 1 to enable")
                .default_value("1")
                .possible_values(&["0", "1"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("array")
                .short("a")
                .long("array")
                .value_name("array_format")
                .help("Set source code format output: rust (r), C (c), golang (g)")
                .possible_values(&["r", "c", "g"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("func")
                .short("u")
                .long("func")
                .value_name("func_length")
                .help("Set function wave length")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("places")
                .short("p")
                .long("places")
                .value_name("func_places")
                .help("Set function wave output decimal places")
                .takes_value(true),
        );

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        app.clone().print_help().unwrap();
        println!();
        println!();
        process::exit(0);
    }

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
