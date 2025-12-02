use std::fs;

fn main() {
    //let file_path = "./inputs/day01_test";
    let file_path = "./inputs/day01";
    println!("Read input from file {file_path}");

    let mut start: i32 = 50;
    let mut zeroes: i32 = 0;
    // make it so it circles from 0-99
    // if 50 - 60, it will go back to 90
    // use rem_euclid to deal with negative numbers
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let (letter_part, number_part) = line.split_at(1);
        let letter = letter_part.chars().next().unwrap();
        let number: i32 = number_part.parse().unwrap();

        let value = match letter {
            'L' => -1,
            'R' => 1,
            _ => panic!("Unexpected letter"),
        };

        let rotation: i32 = number * value;

        let result = (start + rotation).rem_euclid(100);
        if result == 0 {
            zeroes += 1;
        }
        start = result
    }

    println!("Password: {}", zeroes);
}
