use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;

    for bank in input.split("\n") {
        let mut max_first_digit = 0;
        let mut max_first_digit_idx = 0;
        
        let trimmed_bank = bank.trim();

        for i in 0..trimmed_bank.len() - 1 {
            let first_digit = trimmed_bank.chars().nth(i).unwrap().to_digit(10).unwrap();

            if first_digit > max_first_digit {
                max_first_digit = first_digit;
                max_first_digit_idx = i;
            }
        }

        let mut max_second_digit = 0;

        for i in max_first_digit_idx + 1..trimmed_bank.len() {
            let second_digit = trimmed_bank.chars().nth(i).unwrap().to_digit(10).unwrap();

            if second_digit > max_second_digit {
                max_second_digit = second_digit;
            }
        }

        sum += max_first_digit * 10 + max_second_digit;
    }
    
    println!("{}", sum);
}
