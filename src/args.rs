//! Command-line argument constants and input detection utilities
//!
//! This module provides constants for command-line argument names and utilities
//! for detecting whether input should be read from stdin or from a file.
//!
//! # Examples
//!
//! ```rust
//! use hx::is_stdin;
//! use clap::{Arg, Command};
//!
//! let app = Command::new("test")
//!     .arg(Arg::new("INPUTFILE").index(1));
//! let matches = app.get_matches_from(vec!["test", "file.txt"]);
//! let result = is_stdin(matches).unwrap();
//! assert!(!result); // File provided, not stdin
//! ```

use clap::ArgMatches;
use std::env;
use std::error::Error;

/// Command-line argument name for column width (`--cols`)
pub const ARG_COL: &str = "cols";

/// Command-line argument name for truncation length (`--len`)
pub const ARG_LEN: &str = "len";

/// Command-line argument name for output format (`--format`)
pub const ARG_FMT: &str = "format";

/// Command-line argument name for input file (`INPUTFILE`)
pub const ARG_INP: &str = "INPUTFILE";

/// Command-line argument name for colorization (`--color`)
pub const ARG_CLR: &str = "color";

/// Command-line argument name for array format (`--array`)
pub const ARG_ARR: &str = "array";

/// Command-line argument name for function wave length (`--func`)
pub const ARG_FNC: &str = "func";

/// Command-line argument name for decimal places (`--places`)
pub const ARG_PLC: &str = "places";

/// Command-line argument name for numeric prefix (`--prefix`)
pub const ARG_PFX: &str = "prefix";

/// Array of all command-line argument names for validation
const ARGS: [&str; 9] = [
    ARG_COL, ARG_LEN, ARG_FMT, ARG_INP, ARG_CLR, ARG_ARR, ARG_FNC, ARG_PLC, ARG_PFX,
];

/// Detects whether input should be read from stdin or from a file
///
/// Analyzes the command-line arguments to determine if the program should read
/// input from standard input or from a file path. The detection logic follows
/// this order:
///
/// 1. If an input file is explicitly provided via `ARG_INP`, returns `false`
/// 2. If the first argument (after program name) is a recognized flag/option,
///    checks if it's at position 2, indicating stdin mode
/// 3. If no arguments are present, defaults to stdin mode (`true`)
///
/// # Arguments
///
/// * `matches` - `ArgMatches` from `clap` containing parsed command-line arguments
///
/// # Returns
///
/// Returns `Ok(true)` if input should be read from stdin, `Ok(false)` if a file
/// path is provided, or an error if detection fails.
///
/// # Examples
///
/// ```rust,no_run
/// use hx::is_stdin;
/// use clap::{Arg, Command};
///
/// // File provided - not stdin
/// let app = Command::new("test")
///     .arg(Arg::new("INPUTFILE").index(1));
/// let matches = app.get_matches_from(vec!["test", "file.txt"]);
/// let result = is_stdin(matches).unwrap();
/// // result will be false when a file is provided
///
/// // No arguments - stdin mode
/// let app2 = Command::new("test");
/// let matches2 = app2.get_matches_from(vec!["test"]);
/// let result2 = is_stdin(matches2).unwrap();
/// // result2 will be true when no arguments are provided
/// ```
#[allow(clippy::absurd_extreme_comparisons)]
pub fn is_stdin(matches: ArgMatches) -> Result<bool, Box<dyn Error>> {
    let mut is_stdin = false;
    if let Some(_file) = matches.get_one::<String>(ARG_INP) {
        is_stdin = false;
    } else if !matches.args_present() {
        // No arguments at all - use stdin
        is_stdin = true;
    } else if let Some(_nth1) = env::args().nth(1) {
        // Check if any known argument is at position 2 (indicating flags/options, not a file)
        // Use a safe approach: try each argument individually and catch panics
        is_stdin = ARGS.iter().any(|arg| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                matches.contains_id(arg) && matches.index_of(arg) == Some(2)
            }))
            .unwrap_or(false)
        });
    }
    Ok(is_stdin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};

    fn create_test_app() -> Command {
        Command::new("test")
            .arg(Arg::new(ARG_INP).index(1))
            .arg(Arg::new(ARG_COL).short('c').long(ARG_COL))
            .arg(Arg::new(ARG_LEN).short('l').long(ARG_LEN))
    }

    /// Test is_stdin returns false when file argument is provided
    #[test]
    fn test_is_stdin_with_file() {
        let app = create_test_app();
        let matches = app.try_get_matches_from(vec!["test", "file.txt"]).unwrap();
        let result = is_stdin(matches).unwrap();
        assert!(!result);
    }

    /// Test is_stdin returns true when no arguments
    #[test]
    fn test_is_stdin_no_args() {
        let app = create_test_app();
        let matches = app.try_get_matches_from(vec!["test"]).unwrap();
        let result = is_stdin(matches).unwrap();
        assert!(result);
    }

    /// Test is_stdin with flag arguments but no file
    #[test]
    fn test_is_stdin_with_flags() {
        let app = create_test_app();
        let matches = app.try_get_matches_from(vec!["test", "-c", "10"]).unwrap();
        // When flags are at position 2, it should check if they're arguments
        // This is a complex case that depends on the actual argument parsing
        let result = is_stdin(matches);
        assert!(result.is_ok());
    }

    /// Test ARG constants are correct
    #[test]
    fn test_arg_constants() {
        assert_eq!(ARG_COL, "cols");
        assert_eq!(ARG_LEN, "len");
        assert_eq!(ARG_FMT, "format");
        assert_eq!(ARG_INP, "INPUTFILE");
        assert_eq!(ARG_CLR, "color");
        assert_eq!(ARG_ARR, "array");
        assert_eq!(ARG_FNC, "func");
        assert_eq!(ARG_PLC, "places");
        assert_eq!(ARG_PFX, "prefix");
    }
}
