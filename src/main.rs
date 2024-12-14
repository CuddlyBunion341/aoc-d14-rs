use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
