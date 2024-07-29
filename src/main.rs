#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let command :Vec<&str> = vec!["echo", "cat", "exit", "type"];
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).is_ok() {
        if input.starts_with("exit") {
            return;
        }else if input.starts_with("echo"){
            let echo_input = input.trim_start_matches("echo ");
            print!("{}", echo_input);
        }else if input.starts_with("type"){
            let table_input: Vec<&str> = input.trim().split(" ").collect();
            if command.contains(&table_input[1]){
                println!("{} is a shell builtin", &table_input[1]);
            }else{
                println!("{}: not found", &table_input[1]);
            }
        }else{
            println!("{}: command not found", input.trim());
        }
        print!("$ ");
        input.clear();
        io::stdout().flush().unwrap();
    };
}
