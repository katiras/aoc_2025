use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_y = grid.len();
    let max_x = grid.first().unwrap().len();

    let mut split_count = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            if y + 1 >= max_y {
                continue;
            }

            match grid[y][x] {
                'S' | '|' => match grid[y + 1][x] {
                    '.' => grid[y + 1][x] = '|',
                    '^' => {
                        split_count += 1;
                        grid[y + 1][x - 1] = '|';
                        grid[y + 1][x + 1] = '|';
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", grid[y][x]);
        }
        println!();
    }

    println!("{}", split_count);
}
