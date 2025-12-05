use std::io::{self, BufRead};

fn main() {
    let file = std::fs::File::open("./input.txt");
    let lines = io::BufReader::new(file.unwrap()).lines();

    let mut fresh_ranges = Vec::<(u64, u64)>::new();
    let mut available = Vec::<u64>::new();
    let mut fresh_count = 0;

    for line in lines.map_while(Result::ok) {
        if line.len() == 0 {
            continue;
        }

        if line.contains("-") {
            let (a, b) = line.split_once("-").unwrap();
            let start = a.parse::<u64>().unwrap();
            let end = b.parse::<u64>().unwrap() + 1;

            fresh_ranges.push((start, end));
        } else {
            available.push(line.parse::<u64>().unwrap());
        }
    }

    for ingredient in available {
        for range in &fresh_ranges {
            if ingredient > range.0 && ingredient < range.1 + 1 {
                fresh_count += 1;
                break;
            }
        }
    }

    println!("{}", fresh_count);
}
