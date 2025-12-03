use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;

    for bank in input.split("\n") {
        let trimmed_bank = bank.trim();
        let mut start_idx = 0;

        for n in (0..12).rev() {
            let mut max_digit: u64 = 0;

            for i in start_idx..trimmed_bank.len() - n {
                let digit = trimmed_bank.chars().nth(i).unwrap().to_digit(10).unwrap() as u64;

                if digit > max_digit {
                    max_digit = digit;
                    start_idx = i + 1;
                }
            }

            sum += max_digit * u64::pow(10, n as u32);
        }
    }

    println!("{}", sum);
}
