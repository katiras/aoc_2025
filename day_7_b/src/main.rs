use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("Unable to read file");
    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_x = grid[0].iter().position(|&c| c == 'S').unwrap();

    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

    let split_count = calc(&grid, start_x, 0, &mut memo);

    println!("{}", split_count);
}

fn calc(grid: &Vec<Vec<char>>, x: usize, y: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if y + 2 > grid.len() {
        return 1;
    }

    if let Some(&n) = memo.get(&(x, y)) {
        return n;
    }

    let n = match grid[y + 1][x] {
        '.' => calc(grid, x, y + 1, memo),
        '^' => {
            let left = calc(grid, x - 1, y + 2, memo);
            let right = calc(grid, x + 1, y + 2, memo);

            left + right
        }
        _ => panic!("Invalid character"),
    };

    memo.insert((x, y), n);

    return n;
}
