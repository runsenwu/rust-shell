use std;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        } else if input.starts_with("echo ") {
            println!(&string[5..]);
        } else {
            println!("{input}: command not found");
        }
    }
}
