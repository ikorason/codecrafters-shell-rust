use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        let ret = stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "exit 0" => std::process::exit(0),
            cmd if cmd.starts_with("echo ") => {
                println!("{}", &cmd[5..]);
            }
            cmd if cmd.starts_with("type ") => {
                let command = &cmd[5..];
                match command {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", command),
                    _ => println!("{}: not found", command),
                }
            }
            _ => {
                println!("{}: command not found", input);
            }
        }

        if ret == 0 {
            // EOF reached, exit the loop
            break;
        }
    }
}
