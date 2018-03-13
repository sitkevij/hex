
extern crate clap;

use clap::ArgMatches;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};
use std::vec::Vec;

pub fn run(matches: ArgMatches) -> Result<(), Box<::std::error::Error>> { 
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
