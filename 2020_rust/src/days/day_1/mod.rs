use std::iter::Map;
use crate::days::common::{Day};
use std::collections::HashSet;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a {
    input
        .trim()
        .lines()
        .map(|v| v.parse::<i32>().expect("Invalid input"))
}

const DESIRED: i32 = 2020;

pub struct Day1 {}

impl Day for Day1 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let numbers = parse(input);
        let numbers_set = numbers.collect::<HashSet<i32>>();

        for num in numbers_set.iter() {
            if numbers_set.contains(&(DESIRED - num)) {
                return (num * (DESIRED - num)).to_string();
            }
        }
        panic!("Failed to solve part 1, no matching numbers")
    }

    fn part_2(&self, input: &str) -> String {
        let numbers = parse(input);
        let numbers_set = numbers.collect::<HashSet<i32>>();

        for outer in numbers_set.iter() {
            for inner in numbers_set.iter() {
                if numbers_set.contains(&(DESIRED - outer - inner)) {
                    return ((DESIRED - outer - inner) * outer * inner).to_string();
                }
            }
        }
        panic!("Failed to solve part 1, no matching numbers")
    }
}