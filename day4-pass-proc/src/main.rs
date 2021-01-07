use regex::Regex;
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

    fn passport_valid_part2(p: &String) -> bool {
        // let expected_fields = vec!["byr", "eyr", "iyr", "hgt", "hcl", "ecl", "pid"];
        let mut count = 0;

        let passport: Vec<(&str, &str)> = p
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.split_at(v.find(':').unwrap()))
            .collect();

        for (field, value) in passport {
            match field {
                "byr" => {
                    let byr = value.parse::<i32>().unwrap();
                    if byr >= 1920 && byr <= 2002 {
                        count += 1
                    }
                }
                "eyr" => {
                    let eyr = value.parse::<i32>().unwrap();
                    if eyr >= 2010 && eyr <= 2020 {
                        count += 1
                    }
                }
                "iyr" => {
                    let iyr = value.parse::<i32>().unwrap();
                    if iyr >= 2020 && iyr <= 2030 {
                        count += 1
                    }
                }
                "hgt" => {}
                "hcl" => {
                    if value.len() == 7 {
                        // Check for Regex match -> # followed by exactly six characters 0-9 or a-f
                        let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
                        if re.is_match(value) {
                            count += 1
                        }
                    }
                }
                "ecl" => match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => count += 1,
                    _ => {}
                },
                "pid" => {
                    if value.len() == 9 {
                        // Check for Regex match -> a nine-digit number, including leading zeroes
                        let re = Regex::new(r"[0-9]{9}").unwrap();
                        if re.is_match(value) {
                            count += 1
                        }
                    }
                }
                _ => {}
            }
        }

        count >= 7
    }

    // println!("{:#?}", passports_vec);
}
