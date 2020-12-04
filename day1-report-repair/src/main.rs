use std::{collections::HashMap, fs, time::Instant};

fn main() {
    println!("Day 1 - Report Repair");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Convert the string representation of the input into a vector of u32
    let expense_data: Vec<_> = contents
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    partone(&expense_data);

    let now = Instant::now();
    parttwo(&expense_data);
    println!("{}s", now.elapsed().as_secs_f32());
}

fn partone(exp_data: &Vec<u32>) {
    let mut map_of2020: HashMap<u32, u32> = HashMap::new();

    for i in exp_data.iter() {
        map_of2020.insert(*i, 2020 - *i);
    }

    for i in exp_data.iter() {
        let key = 2020 - *i;
        if map_of2020.contains_key(&key) {
            println!("The pair has been found: {} {}", *i, key);
            println!("Their product is: {}", *i * key);
            break;
        }
    }
}

fn parttwo(exp_data: &Vec<u32>) {
    let mut pairs: HashMap<u32, u32> = HashMap::new();

    for i in exp_data.iter() {
        for j in exp_data.iter() {
            if *i != *j {
                pairs.insert(*i + *j, *i * *j);
            }
        }
    }

    pairs
        .iter()
        .filter(|(k, _v)| **k < 2020)
        .filter(|(k, _v)| exp_data.contains(&(2020 - **k)))
        .take(1)
        .for_each(|(k, v)| println!("The product is {}", v * (2020 - *k)));
}
