use std::fs;

const PATH: &str = "asset/day1.input.txt";


fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string(PATH).expect("Should have been able to read a file");
    println!("{}", contents);
}
