use is_executable::IsExecutable;
use std::fs::{self, Permissions};
use std::io::ErrorKind::NotFound;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use std::{self, collections::HashSet};
use std::{env, path, println, string};

static COMMANDS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| HashSet::from(["echo", "exit", "type"]));

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let dirs: Vec<&str> = vec![];

        // I think all of these could be stored within a type of some kind as a switch case
        // to match against
        if input == "exit" {
            break;
        } else if input.starts_with("echo ") {
            println!("{}", &input[5..]);
        } else if input.starts_with("type ") {
            // all of these things can go into functions, instead of all living in main
            let command = &input[5..].trim();
            let (found, result) = is_executable(command);

            if found {
                println!("{result}");
            } else {
                command_not_found(command);
            }
        } else {
            let mut splits = input.split_whitespace();

            let throw: Vec<&str> = splits.clone().collect();

            let command = splits.next().unwrap();

            let (found, result) = is_executable(command);

            if found {
                match Command::new(command).args(splits).status() {
                    Ok(_) => {}
                    Err(_) => println!("Execution failed"),
                }
            } else {
                command_not_found(command);
            }
        }
    }
}

fn command_not_found(command: &str) {
    println!("{command}: not found");
}

fn is_executable(command: &str) -> (bool, String) {
    if COMMANDS.contains(command) {
        return (true, format!("{command} is a shell builtin"));
    } else {
        let test = env::var("PATH").unwrap_or_default();

        if test.is_empty() {
            return (false, String::new());
        } else {
            let paths = env::split_paths(&test);
            let mut ret_string = String::new();

            let mut found = false;

            // perm checking
            for path in paths {
                let temp_path = path.to_str().unwrap_or_default();
                let path_str = &format!("{}/{command}", temp_path);
                let path_with_file = Path::new(path_str);

                if path_with_file.exists() {
                    let permission = match fs::metadata(&path_with_file) {
                        Ok(mode) => mode.permissions().mode(),
                        Err(_) => continue,
                    };

                    // if executable
                    if permission & 0o111 != 0 {
                        ret_string = format!(
                            "{command} is {}",
                            path_with_file.to_str().unwrap_or_default()
                        );
                        found = true;

                        break;
                    }
                }
            }

            return (found, ret_string);
        }
    }
}
