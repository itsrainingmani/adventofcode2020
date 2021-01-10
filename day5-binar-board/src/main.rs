use std::fs;

fn main() {
    println!("Day 5: Binary Board");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Split each passport into a vector of key:value pairs
    let boarding_vec: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    // println!("{:#?}", boarding_vec);

    part_one(boarding_vec);
}

fn part_one(boarding_vec: Vec<&str>) {
    let mut rows = (0, 127);
    let mut cols = (0, 7);
    let mut highest_seat = 0;

    for boarding in boarding_vec {
        for b in boarding.chars() {
            match b {
                'F' => rows = (rows.0, rows.1 / 2),
                'B' => rows = (rows.0 / 2, rows.1),
                'L' => cols = (cols.0, cols.1 / 2),
                'R' => cols = (cols.0 / 2, cols.1),
                _ => {}
            }
        }

        println!("{} {:#?} {:#?}", boarding, rows, cols);
    }
}
