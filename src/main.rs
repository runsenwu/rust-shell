use std;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    while (true) {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim_end();

        println!("{input}: command not found");
    }
}
