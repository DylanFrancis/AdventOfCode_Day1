use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

pub fn run(target: i32, parts: u32) -> Option<Vec<i32>> {
    let arr = read_file();

    let mut count = 0;
    for x in &arr {
        let res = sum(parts, target, arr[count..].to_vec());

        if res.is_some()
        {
            return res;
        }

        count += 1;
    }

    None
}

fn sum(parts: u32, target: i32, arr_to_search: Vec<i32>) -> Option<Vec<i32>> {
    if arr_to_search.len() == 0 {
        return None;
    }

    if parts > 2 {
        let cur = arr_to_search[0];
        let mut res = sum(parts - 1, target - cur, arr_to_search[1..].to_vec());

        return if res.is_none() {
            None
        } else {
            res.as_mut().unwrap().push(cur);
            Some(res.unwrap())
        }
    }
    else {
        return find_sum(target, arr_to_search)
    }

    return None
}

fn find_sum(number_to_find: i32, arr: Vec<i32>) -> Option<Vec<i32>> {
    let mut map = HashMap::new();

    for num in arr {
        if map.contains_key(&num) {
            let mut v = Vec::new();
            v.push(num);
            v.push(*map.get(&num).unwrap());
            return Some(v);
        }

        let diff = number_to_find - num;

        map.insert(diff, num);
    }

    None
}

fn read_file() -> Vec<i32>{
    let file = File::open("input.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut arr: Vec<i32> = Vec::new();

    for line in file_reader.lines() {
        arr.push(line.unwrap().parse::<i32>().unwrap())
    }

    arr
}

