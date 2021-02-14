mod sum;
mod passwords;

use std::env;

fn main() {
    passwords::run();
}

fn sum() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    let target = args.get(0).unwrap().parse::<i32>().unwrap();
    let parts = args.get(1).unwrap().parse::<u32>().unwrap();

    let res = sum::run(target, parts);

    if res.is_some() {
        let v = res.unwrap();
        println!("{} = {:?}", target, v);

        let mut product = 1;
        for x in v {
            product *= x;
        }

        println!("product = {}", product);
    }
    else {
        println!("no result");
    }
}
