use std::io::{self, BufRead};

#[derive(Debug)]
struct FreshRange {
    start: u64,
    end: u64,
}

fn main() {
    let file = std::fs::File::open("./input.txt");
    let lines = io::BufReader::new(file.unwrap()).lines();

    let mut fresh_ranges = Vec::<FreshRange>::new();

    for line in lines.map_while(Result::ok) {
        if line.contains("-") {
            let (a, b) = line.split_once("-").unwrap();
            let start = a.parse::<u64>().unwrap();
            let end = b.parse::<u64>().unwrap();

            fresh_ranges.push(FreshRange { start, end });
        }
    }

    fresh_ranges.sort_by(|x, y| {
        if x.start < y.start {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut merged_ranges = Vec::<FreshRange>::new();

    for range in fresh_ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
            continue;
        }

        let last_range = merged_ranges.last().unwrap();

        if range.end <= last_range.end {
            continue;
        }

        if range.start <= last_range.end && range.end >= last_range.end {
            let popped = merged_ranges.pop().unwrap();
            merged_ranges.push(FreshRange {
                start: popped.start,
                end: range.end,
            });
            continue;
        }

        merged_ranges.push(FreshRange {
            start: range.start,
            end: range.end,
        });
    }

    let mut sum = 0;

    for r in merged_ranges {
        sum += r.end - r.start + 1;
    }

    println!("{:?}", sum);
}
