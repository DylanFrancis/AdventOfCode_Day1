use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let input = read_file();
    let mut valid_password = 0;

    for line in input {
        let mut split = line.split(" ");

        let min_max = parse_min_max(split.next().unwrap().to_string());
        let character = parse_character(split.next().unwrap().to_string());
        let password = split.next().unwrap().to_string();

        let is_valid = validate_password(min_max, character, &password);

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
        (split.next().unwrap().parse::<u32>().unwrap(),
         split.next().unwrap().parse::<u32>().unwrap());
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

