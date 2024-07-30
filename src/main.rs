use std::{env, process::Command};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let command :Vec<&str> = vec!["echo", "exit", "type"];
    let  path_env_str = env::var("PATH").expect("Path cannot be empty");
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
        }else if input.starts_with("type "){
            let table_input: Vec<&str> = input.trim().split(" ").collect();
            if command.contains(&table_input[1]){
                println!("{} is a shell builtin", &table_input[1]);
            }else if let Some(path) = path_env_str.split(':').find(|s|{
                std::fs::metadata(format!("{}/{}", &s, &table_input[1].trim())).is_ok()
            }) {
                println!("{} is {path}/{}", &table_input[1].trim(), &table_input[1].trim());
            }else{
                println!("{}: not found", &table_input[1]);
            }
        }else if input.starts_with("pwd"){
            println!("{}", env::current_dir().unwrap().display());
        } else {
            let mut cmd_parts: Vec<&str> = input.trim().split_whitespace().collect();
            let cmd_name = cmd_parts.remove(0);

            match Command::new(cmd_name).args(&cmd_parts).status() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("{}: command not found", cmd_name);
                    }
                },
                Err(_) => {
                    eprintln!("{}: command not found", cmd_name);
                }
            }
        }
        print!("$ ");
        input.clear();
        io::stdout().flush().unwrap();
    };
}
