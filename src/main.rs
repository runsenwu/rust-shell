use is_executable::IsExecutable;
use std::fs::{self, Permissions};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{self, collections::HashSet};
use std::{env, path, string};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let commands = HashSet::from(["echo", "exit", "type"]);

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
            if commands.contains(command) {
                println!("{command} is a shell builtin");
            } else {
                let test = env::var("PATH").unwrap_or_default();

                if test.is_empty() {
                    println!("{command}: not found")
                } else {
                    let paths = env::split_paths(&test);

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
                                println!(
                                    "{command} is {}",
                                    &path_with_file.to_str().unwrap_or_default()
                                );

                                found = true;

                                break;
                            }
                        }
                    }

                    if (!found) {
                        println!("{command}: not found");
                    }
                }
            }
        } else {
            println!("{input}: command not found");
        }
    }
}
