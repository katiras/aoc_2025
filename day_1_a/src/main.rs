use std::io::{self, BufRead};

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

const INITIAL_POSITION: u32 = 50;

fn main() {
    let file = std::fs::File::open("./input.txt");
    let lines = io::BufReader::new(file.unwrap()).lines();

    let mut rotations = Vec::<Rotation>::new();

    for line in lines.map_while(Result::ok) {
        let mut chars = line.chars();

        let rotation = match chars.next().unwrap() {
            'L' => Rotation::Left(chars.as_str().parse::<u32>().unwrap()),
            'R' => Rotation::Right(chars.as_str().parse::<u32>().unwrap()),
            _ => {
                panic!("Inalid input");
            }
        };

        rotations.push(rotation);
    }

    let mut current_pos = INITIAL_POSITION as i32;
    let mut zero_count = 0;

    for rotation in rotations {
        let pos_diff = match rotation {
            Rotation::Left(n) => n as i32 * -1,
            Rotation::Right(n) => n as i32,
        };

        current_pos = (current_pos + pos_diff).rem_euclid(100);

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}
