use std::io::BufReader;

//use clap::builder::styling::Color;
use cli_utils::{colors, format_stdin};

    

fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit):");
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin.lock());

        input = format_stdin(&mut reader);
        let buffer = colors::red(&mut input);
        println!("You entered: {}", buffer); 
        
    }
    println!("Goodbye!")
}
