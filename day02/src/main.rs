use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> io::Result<Vec<String>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut input: Vec<String> = vec![];

    for line in reader.lines() {
        input.push(line.unwrap());
    }

    Ok(input)
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> i32 {
    let input = read_input().unwrap();
    let mut score = 0;

    for line in input {
        if line.ends_with("X") {
            score += 1;

            if line.starts_with("A") {
                score += 3
            } else if line.starts_with("C") {
                score += 6
            }
        } else if line.ends_with("Y") {
            score += 2;
            if line.starts_with("A") {
                score += 6
            } else if line.starts_with("B") {
                score += 3
            }
        } else if line.ends_with("Z") {
            score += 3;
            if line.starts_with("C") {
                score += 3
            } else if line.starts_with("B") {
                score += 6
            }
        }
    }

    score
}

fn part_two() -> i32 {
    let input = read_input().unwrap();
    let mut score = 0;

    for line in input {
        if line.ends_with("X") {
            if line.starts_with("A") {
                score += 3
            } else if line.starts_with("B") {
                score += 1
            } else if line.starts_with("C") {
                score += 2
            }
        } else if line.ends_with("Y") {
            if line.starts_with("A") {
                score += 1 + 3
            } else if line.starts_with("B") {
                score += 2 + 3
            } else if line.starts_with("C") {
                score += 3 + 3
            }
        } else if line.ends_with("Z") {
            if line.starts_with("C") {
                score += 1 + 6
            } else if line.starts_with("B") {
                score += 3 + 6
            } else if line.starts_with("A") {
                score += 2 + 6
            }
        }
    }

    score
}
