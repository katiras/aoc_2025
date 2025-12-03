use std::fs;

fn main() {
    let message: String = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;

    for range in message.trim().split(",") {
        let (start, end) = range.trim().split_once("-").unwrap();

        for id in start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap() + 1 {
            let id_str = id.to_string();

            for l in 1..id_str.len() {
                if id_str.len() % l != 0 {
                    continue;
                }

                let mut str = "".to_string();

                for _ in 0..id_str.len() / l {
                    str.push_str(&id_str[0..l]);
                }

                if str == id_str {
                    println!("{} {} {}", id_str, str, l);
                    sum += id;
                    break
                }
            }
        }
    }

    println!("{}", sum);
}
