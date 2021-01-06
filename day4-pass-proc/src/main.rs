use std::fs;

fn main() {
    println!("Advent of Code 2020: Day 4: Passport Processing");

    let input_filename = String::from("input.txt");

    let contents =
        fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    // Split each passport into a vector of key:value pairs
    let mut passports_vec: Vec<String> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_ascii_whitespace().collect::<Vec<&str>>().join(","))
        .collect();

    // Closure that checks if the passport is valid
    fn passport_valid_part1(p: &String) -> bool {
        let expected_fields = vec!["byr", "eyr", "iyr", "hgt", "hcl", "ecl", "pid"];
        let mut count = 0;
        for e in expected_fields {
            if p.contains(e) {
                count += 1;
            }
        }

        count >= 7
    }

    passports_vec = passports_vec
        .iter()
        .cloned()
        .filter(|p| passport_valid_part1(p))
        .collect();

    println!("{:#?}", passports_vec.len());

    // println!("{:#?}", passports_vec);
}
