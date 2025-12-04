use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().map(|c| c).collect())
        .collect();

    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible_rolls = 0;

    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let mut adj_rolls = 0;

            if grid[i][j] != '@' {
                continue;
            }

            for d in &directions {
                let x = j as i32 + d.1;
                let y = i as i32 + d.0;

                if x < 0 || y < 0 || y > max_y - 1 || x > max_x - 1 {
                    continue;
                }

                if grid[y as usize][x as usize] == '@' {
                    adj_rolls += 1;
                }
            }

            if adj_rolls < 4 {
                accessible_rolls += 1;
            }
        }
    }

    println!("{}", accessible_rolls);
}
