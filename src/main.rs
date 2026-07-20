#[allow(unused_imports)]
use std::io::{self, Write};
use std;

fn main() {
    print!("$ ");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim_end();

    print!("{input}: command not found");

    return;
}
