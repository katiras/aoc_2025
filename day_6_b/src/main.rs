use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut lines: Vec<&str> = content.lines().collect();

    let op_chars: Vec<char> = lines.pop().expect("File is empty").chars().collect();

    let mut total_sum: u64 = 0;
    let mut current_term: u64 = 0;
    let mut current_op = ' ';

    for (i, &char_at_i) in op_chars.iter().enumerate() {
        if char_at_i != ' ' {
            current_op = char_at_i;
        }

        let next_char = *op_chars.get(i + 1).unwrap_or(&' ');

        if next_char != ' ' {
            total_sum += current_term;
            current_term = 0;
            continue;
        }

        let col_string: String = lines
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|c| !c.is_whitespace())
            .collect();

        let value = col_string.parse::<u64>().expect("Unable to parse number");

        match current_op {
            '+' => current_term += value,
            '*' => {
                if current_term == 0 {
                    current_term = value;
                } else {
                    current_term *= value;
                }
            }
            _ => {}
        }
    }

    total_sum += current_term;

    println!("{}", total_sum);
}
