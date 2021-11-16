use crate::days::common::AOCInput;

pub fn parse(input: &str) -> Vec<u64> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|v| v.parse::<u64>().expect("Invalid input"))
        .collect()
}

pub fn part_1(input: &str) -> Option<u64> {
    let numbers = parse(input);
    for outer in numbers.iter() {
        for inner in numbers.iter() {
            if inner + outer == 2020 {
                return Some(inner * outer)
            }
        }
    }
    None
}

pub fn part_2(input: &str) -> Option<u64> {
    let numbers = parse(input);
    for outer in numbers.iter() {
        for middle in numbers.iter() {
            for inner in numbers.iter() {
                if inner + middle + outer == 2020 {
                    return Some(inner * middle * outer)
                }
            }
        }
    }
    None
}

pub fn run(input: AOCInput) -> () {
    let data = match input {
        AOCInput::Input(input) => input,
        AOCInput::Test => include_str!("test_input.txt"),
    };

    let part_one = part_1(data);
    if let Some(val) = part_one {
        println!("Solution to part 1: {}", val);
    } else {
        println!("Found no solution to part 1 :(");
    }
    let part_two = part_2(data);
    if let Some(val) = part_two {
        println!("Solution to part 2: {}", val);
    } else {
        println!("Found no solution to part 2 :(");
    }
}