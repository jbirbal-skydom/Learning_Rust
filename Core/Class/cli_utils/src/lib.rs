
//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line".



use std::io::{BufRead, BufReader};

use colors::red;

// we can make a public modules
pub mod colors;
pub mod config;

// or private module for local use
mod colors_pvt;


/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use cli_utils::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    pvt_function();
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

pub fn format_stdin<R: BufRead>(mut input: &mut R) -> String {
    let output = _read_stdin(&mut input);
    output.to_string()

}
// we can not use stdin for testing we need to test the main function.
// so we will make a function for just reading stdin and one to do the logic. 

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}



/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use cli_utils::read_stdin;
/// let input = read_stdin();
/// ```
fn pvt_function(){
        // Use the set_unique_color function
        colors_pvt::set_unique_color(100, 200, 50);
    
        // Print colored text
        println!("Hello, World!");
    
        // Reset the color to default
        colors_pvt::reset_color();
}

#[cfg(test)]
mod tests {
   mod test_stdin;
   pub mod test_colors;

    use super::_read_stdin;
    use std::io::Cursor;

   #[test]
   fn test_read_input() {
       let input = "Hello, world!\n";
       let expected_output = "Hello, world!";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }

   #[test]
   fn test_read_input_empty() {
       let input = "";
       let expected_output = "";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }

}