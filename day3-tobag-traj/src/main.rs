use std::{fs, io};

type Slope = (usize, usize);
type Pos = Slope;

fn main() {
    println!("Advent of Code - Day 3 - Tobaggon Trajectory");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Convert the string representation of the input into a vector of strs
    let trees: Vec<_> = contents.split_ascii_whitespace().collect();
    part_one(&trees);
}

fn part_one(trees: &Vec<&str>) {
    let slope: Slope = (3, 1); // Slope description ; 3 right 1 down
    let mut cur_pos: Pos = (0, 0); // Current position of tobaggons
    let size_of_treeline = trees.get(0).expect("Something went wrong").len();
    // Count of tree encounters
    let mut encounters: usize = match get_at_location(&trees, cur_pos.0, cur_pos.1).unwrap() {
        '#' => 1,
        _ => 0,
    };

    for _ in 1..trees.len() {
        let (x, y): Pos = (cur_pos.0 + slope.0, cur_pos.1 + slope.1);

        if x <= size_of_treeline {
            encounters += match get_at_location(&trees, x, y).unwrap() {
                '#' => 1,
                _ => 0,
            };
        } else {
            let x = (cur_pos.0 + slope.0) % size_of_treeline; // Shadow x here
            encounters += match get_at_location(&trees, x, y).unwrap() {
                '#' => 1,
                _ => 0,
            };
        }
        cur_pos = (x, y);
    }

    println!("Number of tree encounters: {}", encounters);
}

fn get_at_location(trees: &Vec<&str>, x: usize, y: usize) -> Option<char> {
    trees.get(y)?.chars().nth(x)
}
