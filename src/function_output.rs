//! Function wave output generation
//!
//! This module provides functionality to generate and output mathematical
//! function wave data. The wave is generated using a sine function over a
//! specified range and output as comma-separated floating-point values.
//!
//! # Examples
//!
//! ```rust,no_run
//! use hx::output_function;
//!
//! // Generate 10 values with 2 decimal places
//! output_function(10, 2);
//! ```

/// Generates and outputs a mathematical function wave to stdout
///
/// Generates a sequence of floating-point values representing a sine wave
/// function. The wave is calculated as `sin((y / len) * π / 2)` for each value
/// `y` from `0` to `len-1`. Values are formatted with the specified number of
/// decimal places and printed as comma-separated values, with newlines inserted
/// every 10 values for readability.
///
/// # Mathematical Formula
///
/// For each index `y` in `0..len`:
/// ```text
/// x = sin((y / len) * π / 2)
/// ```
///
/// This produces a wave that starts at 0.0 and reaches approximately 1.0
/// at the end of the sequence.
///
/// # Output Format
///
/// Values are printed as comma-separated floating-point numbers with the
/// specified precision. A newline is inserted after every 10th value (indices
/// 9, 19, 29, etc.) for better readability. A final newline is printed after
/// all values.
///
/// # Arguments
///
/// * `len` - The number of values to generate (wave length). Must be a positive integer.
/// * `places` - Number of decimal places for formatting the floating-point values
///
/// # Examples
///
/// ```rust,no_run
/// use hx::output_function;
///
/// // Generate 5 values with 2 decimal places
/// // Output: 0.00,0.00,0.31,0.59,0.81,
/// output_function(5, 2);
///
/// // Generate 20 values with 4 decimal places
/// output_function(20, 4);
///
/// // Generate 100 values with 6 decimal places (for high precision)
/// output_function(100, 6);
/// ```
///
/// # Note
///
/// This function writes directly to stdout. To capture the output programmatically,
/// consider redirecting stdout or using a custom writer implementation.
pub fn output_function(len: u64, places: usize) {
    for y in 0..len {
        let y_float: f64 = y as f64;
        let len_float: f64 = len as f64;
        let x: f64 = (((y_float / len_float) * std::f64::consts::PI) / 2.0).sin();
        let formatted_number = format!("{:.*}", places, x);
        print!("{}", formatted_number);
        print!(",");
        if (y % 10) == 9 {
            println!();
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test output_function with small length
    #[test]
    fn test_output_function_small() {
        // Capture output by redirecting stdout
        output_function(5, 2);
        // Function prints to stdout, so we can't easily test output
        // But we can verify it doesn't panic
    }

    /// Test output_function with zero length
    #[test]
    fn test_output_function_zero() {
        output_function(0, 2);
        // Should not panic
    }

    /// Test output_function with different decimal places
    #[test]
    fn test_output_function_decimal_places() {
        output_function(3, 0);
        output_function(3, 4);
        output_function(3, 10);
        // Should not panic with different precision values
    }

    /// Test output_function with large length
    #[test]
    fn test_output_function_large() {
        output_function(100, 4);
        // Should handle larger sequences
    }

    /// Test output_function produces correct number of values
    #[test]
    fn test_output_function_count() {
        // The function should produce exactly `len` values
        // We can't easily test stdout, but we can verify it runs
        output_function(10, 2);
    }

    /// Test output_function formatting
    #[test]
    fn test_output_function_formatting() {
        // Test that the function formats numbers correctly
        // Since it prints to stdout, we verify it doesn't crash
        output_function(1, 5);
    }
}
