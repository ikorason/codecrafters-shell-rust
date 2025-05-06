use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        let ret = stdin.read_line(&mut input).unwrap();
        println!("{}: command not found", input.trim());

        if ret == 0 {
            // EOF reached, exit the loop
            break;
        }
    }
}
