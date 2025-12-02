use std::fs;

fn main() {
    let message: String = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;

    for range in message.trim().split(",") {
        let (start, end) = range.trim().split_once("-").unwrap();

        for id in start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap() + 1 {
            let id_str = id.to_string();
            let (a, b) = id_str.split_at(id_str.len() / 2);

            if a == b {
                sum += id;
            }
        }
    }
    
    println!("{}", sum);
}
