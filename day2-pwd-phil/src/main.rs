use std::fs;

fn main() {
    println!("Day 2: Password Philosophy");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Convert the string representation of the input into a vector of str
    let expense_data = contents
        .split_ascii_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|c| Vec::from(c))
        .collect::<Vec<_>>();

    // part one
    let pt_one_valid_pwds = expense_data.iter().filter(|v| pt_one_validate(v)).count();
    println!(
        "Number of Valid Pwds for Part One: {:#?}",
        pt_one_valid_pwds
    );

    // part one
    let pt_two_valid_pwds = expense_data.iter().filter(|v| pt_two_validate(v)).count();
    println!(
        "Number of Valid Pwds for Part Two: {:#?}",
        pt_two_valid_pwds
    );
}

fn pt_one_validate(pwd_data: &Vec<String>) -> bool {
    let pwd_policy = pwd_data.get(0).unwrap().split('-').collect::<Vec<_>>();
    let min_let = pwd_policy.get(0).unwrap().parse::<usize>().unwrap();
    let max_let = pwd_policy.get(1).unwrap().parse::<usize>().unwrap();

    let letter = pwd_data.get(1).unwrap().chars().nth(0).unwrap();

    let pwd = pwd_data.get(2).unwrap();

    let num_letters = pwd.chars().filter(|c| *c == letter).count();

    num_letters >= min_let && num_letters <= max_let
}

fn pt_two_validate(pwd_data: &Vec<String>) -> bool {
    let pwd_policy = pwd_data.get(0).unwrap().split('-').collect::<Vec<_>>();
    let pos1 = pwd_policy.get(0).unwrap().parse::<usize>().unwrap() - 1;
    let pos2 = pwd_policy.get(1).unwrap().parse::<usize>().unwrap() - 1;

    let letter = pwd_data.get(1).unwrap().chars().nth(0).unwrap();

    let pwd = pwd_data.get(2).unwrap();

    let is_char_p1 = pwd.chars().nth(pos1).unwrap() == letter;
    let is_char_p2 = pwd.chars().nth(pos2).unwrap() == letter;

    !(is_char_p1 && is_char_p2) && !(!is_char_p1 && !is_char_p2)
}
