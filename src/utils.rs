// utils.rs

//! Utils Module
//!
//! This module provides generic utility functions used throughout the program.
//! Includes functions for file management, string formatting, etc.
//!
//! # Functions
//!
//! ```
//! use utils::print_helper;
//! use utils::another_function;
//!
//! print_helper(Some("Message"));
//! ```

/// Prints a helpful guide for the lost souls.
///
/// It's like a friendly local giving you directions.
pub fn print_helper(support_message: Option<String>) {
    let message = match support_message {
        Some(msg) => msg,
        None => "".to_string(),
    };
    print!("{}", message);
    let help = r#"Usage:  cdc [OPTIONS] COMMAND
Your best Code directory cleaner

Options:
  -t          Set the target directory
  -O          Set the output filename
  -e          Exlude directories
  -h          Print the helps
    "#;
    println!("{}", help);
}
