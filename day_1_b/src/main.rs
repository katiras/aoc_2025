use std::io::{self, BufRead};

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

const INITIAL_POSITION: i32 = 50;
const UPPER_BOUND: i32 = 100;

fn main() {
    let file = std::fs::File::open("./input.txt");
    let lines = io::BufReader::new(file.unwrap()).lines();

    let mut rotations = Vec::<Rotation>::new();

    for line in lines.map_while(Result::ok) {
        let mut chars = line.chars();

        let rotation = match chars.next().unwrap() {
            'L' => Rotation::Left(chars.as_str().parse::<i32>().unwrap()),
            'R' => Rotation::Right(chars.as_str().parse::<i32>().unwrap()),
            _ => {
                panic!("Inalid input");
            }
        };

        rotations.push(rotation);
    }

    let mut current_pos = INITIAL_POSITION;
    let mut zeros = 0;

    for rotation in rotations {
        let prev_pos = current_pos;
        
        match rotation {
            Rotation::Left(n) => {
                let full_rotations = n / UPPER_BOUND;
                let remainder = n - (UPPER_BOUND * full_rotations);

                zeros += full_rotations;

                current_pos = (current_pos - remainder).rem_euclid(UPPER_BOUND);

                if prev_pos != 0 && current_pos != 0 && current_pos > prev_pos {
                    zeros += 1;
                }
            }
            Rotation::Right(n) => {
                let full_rotations = n / UPPER_BOUND;
                let remainder = n - (UPPER_BOUND * full_rotations);

                zeros += full_rotations;

                current_pos = (current_pos + remainder).rem_euclid(UPPER_BOUND);

                if prev_pos != 0 && current_pos != 0 && current_pos < prev_pos {
                    zeros += 1;
                }
            }
        };

        if current_pos == 0 {
            zeros += 1;
        }
    }

    println!("{}", zeros);
}
