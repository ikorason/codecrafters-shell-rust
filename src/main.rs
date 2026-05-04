use std::{
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

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
                    _ => match std::env::var("PATH") {
                        Ok(path_str) => {
                            let mut found = false;

                            for dir in path_str.split(":") {
                                let mut path = PathBuf::from(dir);
                                path.push(command);

                                if path.exists()
                                    && path.metadata().unwrap().permissions().mode() & 0o111 != 0
                                {
                                    println!("{} is {}", command, path.display());
                                    found = true;
                                }

                                if found {
                                    break;
                                }
                            }

                            if !found {
                                println!("{}: not found", command);
                            }
                        }
                        Err(e) => todo!(),
                    },
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
