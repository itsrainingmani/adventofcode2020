use std::fs;

fn main() {
    println!("Advent of Code 2020: Day 4: Passport Processing");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Split each passport into a vector of key:value pairs
    let passports_vec: Vec<Vec<&str>> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_ascii_whitespace().collect())
        .collect();

    println!("{:#?}", passports_vec);
}
