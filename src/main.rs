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
       .arg(Arg::with_name("FILE")
                    .help("Pass file path as an argument for inspection")
                    .required(true)
                    .index(1))
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