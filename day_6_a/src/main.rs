use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut grid: Vec<Vec<&str>> = content
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let operators = grid.pop().expect("Unable to pop operator row");

    let total_sum: u64 = operators
        .iter()
        .enumerate()
        .map(|(col_idx, &symbol)| {
            let col_iter = grid
                .iter()
                .filter_map(|row| row.get(col_idx))
                .map(|s| s.parse::<u64>().expect("Unable to parse number"));

            match symbol {
                "+" => col_iter.sum::<u64>(),
                "*" => col_iter.product::<u64>(),
                _ => panic!("Unknown operator symbol: {}", symbol),
            }
        })
        .sum();

    println!("{}", total_sum);
}
