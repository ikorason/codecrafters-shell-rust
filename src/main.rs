use std::{
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

fn find_in_path(cmd: &str) -> Option<PathBuf> {
    match std::env::var("PATH") {
        Ok(path_str) => {
            for dir in path_str.split(":") {
                let mut path = PathBuf::from(dir);
                path.push(cmd);

                if path.exists() && path.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                    return Some(path);
                }
            }

            None
        }
        Err(_e) => todo!(),
    }
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "echo" | "exit" | "type" | "pwd" | "cd")
}

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
            "exit 0" | "exit" => std::process::exit(0),
            cmd if cmd.starts_with("echo ") => {
                let command = &cmd[5..];
                println!("{}", command);
            }
            cmd if cmd.starts_with("type ") => {
                let cmd = &cmd[5..];
                if is_builtin(cmd) {
                    println!("{} is a shell builtin", cmd)
                } else if let Some(path) = find_in_path(cmd) {
                    println!("{} is {}", cmd, path.display())
                } else {
                    println!("{}: not found", cmd)
                }
            }
            "pwd" => match std::env::current_dir() {
                Ok(current_dir) => {
                    println!("{}", current_dir.display());
                }
                Err(_e) => todo!(),
            },
            cmd if cmd.starts_with("cd ") => {
                let path = &cmd[3..];

                let target = if path == "~" {
                    std::env::var("HOME").unwrap_or_default()
                } else {
                    path.to_string()
                };

                let write_dir = std::env::set_current_dir(target);

                if let Err(_e) = write_dir {
                    println!("cd: {path}: No such file or directory");
                }
            }
            _ => {
                let parts: Vec<&str> = input.split(' ').collect();
                let command_name = parts[0];
                if Command::new(command_name)
                    .args(&parts[1..])
                    .status()
                    .is_err()
                {
                    println!("{}: command not found", command_name);
                }
            }
        }

        if ret == 0 {
            // EOF reached, exit the loop
            break;
        }
    }
}
