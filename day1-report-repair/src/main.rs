use std::{collections::HashMap, fs};

fn main() {
    println!("Day 1 - Report Repair");
    let input_filename = String::from("input.txt");

    println!("In file {}", input_filename);

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Convert the string representation of the input into a vector of u32
    let expense_data: Vec<_> = contents
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut map_of2020: HashMap<u32, u32> = HashMap::new();

    for i in expense_data.iter() {
        map_of2020.insert(*i, 2020 - *i);
    }

    for i in expense_data.iter() {
        let key = 2020 - *i;
        if map_of2020.contains_key(&key) {
            println!("The pair has been found: {} {}", *i, key);
            println!("Their product is: {}", *i * key);
            break;
        }
    }
}
