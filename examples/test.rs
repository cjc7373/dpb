use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("Usage: test <filename>");
    let contents = fs::read(&path).expect("Failed to read file");
    println!("{}: {} bytes", path, contents.len());
}
