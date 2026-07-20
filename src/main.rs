#[allow(unused_imports)]
use std::io::{self, Write};
use std::{self, collections::HashSet};

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

        // I think all of these could be stored within a type of some kind as a switch case
        // to match against
        if input == "exit" {
            break;
        } else if input.starts_with("echo ") {
            println!("{}", &input[5..]);
        } else if input.starts_with("type ") {
            let command = &input[5..];
            if commands.contains(command) {
                println!("{command} is a shell builtin");
            } else {
                println!("{command}: not found")
            }
        } else {
            println!("{input}: command not found");
        }
    }
}
