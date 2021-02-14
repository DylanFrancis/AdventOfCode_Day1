use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::min;

pub fn run() {
    let input = read_file();
    let mut valid_password = 0;

    for line in input {
        let mut split = line.split(" ");

        let min_max = parse_min_max(split.next().unwrap().to_string());
        let character = parse_character(split.next().unwrap().to_string());
        let password = split.next().unwrap().to_string();

        let is_valid = validate_password_new_policy(min_max, character, &password);

        if is_valid {
            valid_password += 1;
        }
    }

    println!("valid passwords {}", valid_password);
}

fn parse_character(character: String) -> char {
    character.chars().next().unwrap()
}

fn parse_min_max(min_max: String) -> (u32, u32) {
    let mut split = min_max.split("-");
    return
        (split.next().unwrap().parse::<u32>().unwrap() - 1,
         split.next().unwrap().parse::<u32>().unwrap() - 1);
}

fn validate_password_new_policy(min_max: (u32, u32), character: char, password: &String) -> bool {
    let mut count = 0;
    let mut contains = false;

    for c in password.chars() {
        if count == min_max.0 {
            if c == character {
                contains = true;
            }
            count += 1;
            continue
        }

        if count == min_max.1 {
            if c == character {
                if contains {
                    println!("false");
                    return false;
                }
                else {
                    println!("true");
                    return true;
                }
            }
            return contains;
        }
        count += 1;
    }
    return false;
}

fn validate_password(min_max: (u32, u32), character: char, password: &String) -> bool {
    let mut count = 0;
    for c in password.chars() {
        if c == character {
            count += 1;
        }
        if count > min_max.1 {
            return false
        }
    }

    if count < min_max.0 {
        return false
    }

    true
}

fn read_file() -> Vec<String> {
    let file = File::open("passwords.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut arr: Vec<String> = Vec::new();

    for line in file_reader.lines() {
        arr.push(line.unwrap())
    }

    arr
}

