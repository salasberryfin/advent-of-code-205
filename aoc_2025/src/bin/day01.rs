use std::fs;

fn main() {
    //let file_path = "./inputs/day01_test";
    let file_path = "./inputs/day01";

    part2(file_path);
}

pub fn part1(file_path: &str) {
    println!("Part 1");

    let mut pos: i32 = 50;
    let mut zeroes: i32 = 0;
    // make it so it circles from 0-99
    // if 50 - 60, it will go back to 90
    // use rem_euclid to deal with negative numbers
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let (letter, number_part) = line.split_at(1);
        let number: i32 = number_part.parse().unwrap();

        match letter {
            "R" => {
                pos = (pos + number).rem_euclid(100);
            }
            "L" => {
                pos = (pos - number).rem_euclid(100);
            }
            _ => panic!("Invalid operation"),
        }

        if pos == 0 {
            zeroes += 1;
        }
    }

    println!("Password part one: {}", zeroes);
}

pub fn part2(file_path: &str) {
    println!("Part 2");

    let mut absolute_pos: i32 = 50;
    let mut zeroes_part_two: i32 = 0;
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let (letter, number_part) = line.split_at(1);
        let number: i32 = number_part.parse().unwrap();

        zeroes_part_two += number.div_euclid(100);

        let remainder = number % 100;

        if remainder > 0 {
            let dist_to_zero = match letter {
                "R" => 100 - absolute_pos,
                "L" => {
                    if absolute_pos == 0 {
                        100
                    } else {
                        absolute_pos
                    }
                }
                _ => panic!("Invalid operation"),
            };
            if remainder >= dist_to_zero {
                zeroes_part_two += 1;
            }
        }
        match letter {
            "R" => {
                absolute_pos = (absolute_pos + number).rem_euclid(100);
            }
            "L" => {
                absolute_pos = (absolute_pos - number).rem_euclid(100);
            }
            _ => panic!("Invalid operation"),
        }
    }

    println!("Password part two: {}", zeroes_part_two);
}
