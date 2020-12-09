use std::fs;

type Slope = (usize, usize);
type Pos = Slope;

fn main() {
    println!("Advent of Code - Day 3 - Tobaggon Trajectory");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Convert the string representation of the input into a vector of strs
    let trees: Vec<_> = contents.split_ascii_whitespace().collect();
    let slope: Slope = (3, 1);
    part_one(&trees, slope);

    let slopes: [Slope; 5] = [(3, 1), (5, 1), (1, 1), (7, 1), (1, 2)];
    let mut encounter_vec: Vec<usize> = Vec::new();
    for sl in slopes.iter() {
        encounter_vec.push(part_one(&trees, *sl));
    }

    println!("{}", encounter_vec.iter().fold(1, |acc, x| acc * x));
}

fn part_one(trees: &Vec<&str>, slope: Slope) -> usize {
    let mut cur_pos: Pos = (0, 0); // Current position of tobaggons
    let size_of_treeline = trees.get(0).expect("Something went wrong").len();
    let mut encounters: usize = 0;

    for _ in 0..trees.len() {
        let (mut x, y): Pos = cur_pos;
        if y >= trees.len() {
            break;
        }
        x %= size_of_treeline;
        encounters += match get_at_location(&trees, x, y).unwrap() {
            '#' => 1,
            _ => 0,
        };
        cur_pos = (x + slope.0, y + slope.1);
    }

    println!("Number of tree encounters: {}", encounters);
    encounters
}

fn get_at_location(trees: &Vec<&str>, x: usize, y: usize) -> Option<char> {
    trees.get(y)?.chars().nth(x)
}
