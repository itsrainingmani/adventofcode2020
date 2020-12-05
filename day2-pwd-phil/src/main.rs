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
        .map(|c| process(&Vec::from(c)))
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

struct PwdData {
    p1: usize,
    p2: usize,
    letter: char,
    pwd: String,
}

fn process(pwd_data: &Vec<String>) -> PwdData {
    let pwd_policy = pwd_data.get(0).unwrap().split('-').collect::<Vec<_>>();
    let min_let = pwd_policy.get(0).unwrap().parse::<usize>().unwrap();
    let max_let = pwd_policy.get(1).unwrap().parse::<usize>().unwrap();

    let letter = pwd_data.get(1).unwrap().chars().nth(0).unwrap();

    let pwd = pwd_data.get(2).unwrap();

    PwdData {
        p1: min_let,
        p2: max_let,
        letter,
        pwd: pwd.clone(),
    }
}

fn pt_one_validate(pwd_data: &PwdData) -> bool {
    let num_letters = pwd_data
        .pwd
        .chars()
        .filter(|c| *c == pwd_data.letter)
        .count();

    num_letters >= pwd_data.p1 && num_letters <= pwd_data.p2
}

fn pt_two_validate(pwd_data: &PwdData) -> bool {
    let is_char_p1 = pwd_data.pwd.chars().nth(pwd_data.p1 - 1).unwrap() == pwd_data.letter;
    let is_char_p2 = pwd_data.pwd.chars().nth(pwd_data.p2 - 1).unwrap() == pwd_data.letter;

    !(is_char_p1 && is_char_p2) && !(!is_char_p1 && !is_char_p2)
}
