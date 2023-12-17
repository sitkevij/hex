extern crate clap;

use clap::Arg;
use clap::Command;
use std::env;
use std::io::Error;
use std::io::ErrorKind;
use std::process;

/// Central application entry point.
fn main() {
    let desc = &format!(
        "{}\n{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_HOMEPAGE")
    );
    let app = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(desc)
        .arg(
            Arg::new(hx::ARG_COL)
                .action(clap::ArgAction::Set)
                .short('c')
                .long(hx::ARG_COL)
                .value_name("columns")
                .help("Set column length")
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_LEN)
                .action(clap::ArgAction::Set)
                .short('l')
                .long(hx::ARG_LEN)
                .value_name(hx::ARG_LEN)
                .help("Set <len> bytes to read")
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_FMT)
                .action(clap::ArgAction::Set)
                .short('f')
                .long(hx::ARG_FMT)
                .help("Set format of octet: Octal (o), LowerHex (x), UpperHex (X), Binary (b)")
                .value_parser(["o", "x", "X", "b"])
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_INP)
                .help("Pass file path as an argument, or input data may be passed via stdin")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new(hx::ARG_CLR)
                .action(clap::ArgAction::Set)
                .short('t')
                .long(hx::ARG_CLR)
                .help("Set color tint terminal output. 0 to disable, 1 to enable")
                .value_parser(["0", "1"])
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_ARR)
                .action(clap::ArgAction::Set)
                .short('a')
                .long(hx::ARG_ARR)
                .value_name("array_format")
                .help("Set source code format output: rust (r), C (c), golang (g), python (p), kotlin (k), java (j), swift (s), fsharp (f)")
                .value_parser(["r", "c", "g", "p", "k", "j", "s", "f"])
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_FNC)
                .short('u')
                .long(hx::ARG_FNC)
                .value_name("func_length")
                .help("Set function wave length")
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_PLC)
                .short('p')
                .long(hx::ARG_PLC)
                .value_name("func_places")
                .help("Set function wave output decimal places")
                .num_args(1)
        )
        .arg(
            Arg::new(hx::ARG_PFX)
                .action(clap::ArgAction::Set)
                .short('r')
                .long(hx::ARG_PFX)
                .help("Include prefix in output (e.g. 0x/0b/0o). 0 to disable, 1 to enable")
                .value_parser(["0", "1"])
                .num_args(1)
        );

    let matches = app.get_matches();
    match hx::run(matches) {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            let err = &Error::last_os_error();
            let suppress_error = match err.kind() {
                ErrorKind::BrokenPipe => process::exit(0),
                _ => false,
            };
            if !suppress_error {
                eprintln!("error: {}", e);
                process::exit(1);
            }
        }
    }
}
