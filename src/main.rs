use clap::{Command, Arg};
use hx::{ARG_ARR, ARG_CLR, ARG_COL, ARG_FMT, ARG_FNC, ARG_INP, ARG_LEN, ARG_PLC};
use std::env;
use std::io::Error;
use std::io::ErrorKind;
use std::process;

/// Central application entry point.
fn main() {
    let desc: &str = &format!(
        "{}\n{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_HOMEPAGE")
    );
    let app = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(desc)
        .arg(
            Arg::new(ARG_COL)
                .short('c')
                .long(ARG_COL)
                .value_name("columns")
                .help("Set column length")
                .takes_value(true),
        )
        .arg(
            Arg::new(ARG_LEN)
                .short('l')
                .long(ARG_LEN)
                .value_name(ARG_LEN)
                .help("Set <len> bytes to read")
                .takes_value(true),
        )
        .arg(
            Arg::new(ARG_FMT)
                .short('f')
                .long(ARG_FMT)
                .help("Set format of octet: Octal (o), LowerHex (x), UpperHex (X), Binary (b)")
                .value_parser(["o", "x", "X", "b"])
                .takes_value(true),
        )
        .arg(
            Arg::new(ARG_INP)
                .help("Pass file path as an argument for hex dump")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new(ARG_CLR)
                .short('t')
                .long(ARG_CLR)
                .help("Set color tint terminal output. 0 to disable, 1 to enable")
                .value_parser(["0", "1"])
                .takes_value(true),
        )
        .arg(
            Arg::new(ARG_ARR)
                .short('a')
                .long(ARG_ARR)
                .value_name("array_format")
                .help("Set source code format output: rust (r), C (c), golang (g), python (p), kotlin (k), java (j), swift (s)")
                .value_parser(["r", "c", "g", "p", "k", "j", "s"])
                .takes_value(true),
        )
        .arg(
            Arg::new(ARG_FNC)
                .short('u')
                .long(ARG_FNC)
                .value_name("func_length")
                .help("Set function wave length")
                .takes_value(true),
        )
        .arg(
            Arg::new(ARG_PLC)
                .short('p')
                .long(ARG_PLC)
                .value_name("func_places")
                .help("Set function wave output decimal places")
                .takes_value(true),
        );

    let matches = app.get_matches();
    match hx::run(matches) {
        Ok(_) => {
            process::exit(0);
        }
        Err(_) => {
            let err = &Error::last_os_error();
            let suppress_error = match err.kind() {
                ErrorKind::BrokenPipe => process::exit(0),
                _ => false,
            };
            if !suppress_error {
                eprintln!("error: {}", err);
                process::exit(1);
            }
        }
    }
}
