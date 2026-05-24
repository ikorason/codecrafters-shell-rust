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

fn parse_args(input: &str) -> Vec<String> {
    let mut ret = vec![];

    let mut current = String::new();

    let mut in_quotes = false;

    for c in input.chars() {
        if c == '\'' {
            in_quotes = !in_quotes;
            continue;
        }

        if c.is_whitespace() && !in_quotes {
            if !current.is_empty() {
                ret.push(current.clone());
            }

            current.clear();
            continue;
        }

        current.push(c);
    }

    if !current.is_empty() {
        ret.push(current);
    }

    ret
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
                let args = &input[5..];
                let str = parse_args(args);
                println!("{}", str.join(" "));
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
                let parts = parse_args(input);
                if Command::new(&parts[0]).args(&parts[1..]).status().is_err() {
                    println!("{}: command not found", &parts[0]);
                }
            }
        }

        if ret == 0 {
            // EOF reached, exit the loop
            break;
        }
    }
}
