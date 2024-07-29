#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).is_ok() {
        println!("{}: command not found", input.trim());
        print!("$ ");
        input.clear();
        io::stdout().flush().unwrap();
    }
}
