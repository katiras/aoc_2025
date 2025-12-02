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
    let mut zero_count = 0;
    let mut zero_passes = 0;

    for rotation in rotations {
        match rotation {
            Rotation::Left(n) => {                
                let incr = n / UPPER_BOUND;
                let rem = n - (UPPER_BOUND * incr);
                
                zero_passes += incr;
                
                let prev_pos = current_pos;
                
                current_pos = (current_pos - rem).rem_euclid(UPPER_BOUND);
                
                if prev_pos != 0 && current_pos != 0 && current_pos > prev_pos {
                    zero_passes += 1;
                }
            }
            Rotation::Right(n) => {                
                let incr = n / UPPER_BOUND;
                let rem = n - (UPPER_BOUND * incr);
                
                zero_passes += incr;
                
                let prev_pos = current_pos;
                
                current_pos = (current_pos + rem).rem_euclid(UPPER_BOUND);
                
                if prev_pos != 0 && current_pos != 0 && current_pos < prev_pos {
                    zero_passes += 1;
                }
            }
        };

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_passes + zero_count);
}
