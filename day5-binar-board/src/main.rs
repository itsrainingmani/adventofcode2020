use std::fs;

fn main() {
    println!("Day 5: Binary Board");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");
}
