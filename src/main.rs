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
        if input.contains("exit") {
            return;
        }else if input.contains("echo"){
            let echo_input = input.trim_start_matches("echo ");
            print!("{}", echo_input);
            print!("$ ");
            input.clear();
            io::stdout().flush().unwrap();
        }else{
            println!("{}: command not found", input.trim());
            print!("$ ");
            input.clear();
            io::stdout().flush().unwrap();
        }
    };
}
