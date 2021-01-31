use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let result = read_file();
    println!("{} + {} = 2020", result.0, result.1);
    println!("{} * {} = {}", result.0, result.1, result.0 * result.1);
}

fn read_file() -> (u32, u32){
    let file = File::open("input.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut map = HashMap::new();

    for line in file_reader.lines() {
        let num = line.unwrap().parse::<u32>().unwrap();

        if map.contains_key(&num) {
            return (num, *map.get(&num).unwrap());
        }

        let diff = 2020 - num;

        map.insert(diff, num);
    }
    (0, 0)
}
