use crate::days::common::Day;
use std::ops::Div;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
}

pub struct Day7 {}

impl Day for Day7 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let crabs: Vec<i32> = parse(input).collect();
        let median = *crabs.clone().select_nth_unstable(crabs.len() / 2).1;
        crabs.into_iter().map(|v| i32::abs(median - v)).sum::<i32>().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let crabs: Vec<i32> = parse(input).collect();
        let mean = crabs.iter().sum::<i32>().div(crabs.len() as i32);
        crabs.iter().fold(0, |sum, val| {
            let diff = i32::abs(mean - val);
            sum + ((diff.pow(2) + diff).div(2))
        }).to_string()
    }
}