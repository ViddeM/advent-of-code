use crate::days::common::Day;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
}

fn fuel_to_pos<'a>(nums: impl Iterator<Item=i32> + 'a, pos: i32) -> i32 {
    nums.fold(0, |tot, num| (pos - num).abs() + tot)
}

fn fuel_to_pos_2<'a>(nums: impl Iterator<Item=i32> + 'a, pos: i32) -> i32 {
    nums.fold(0, |tot, num| {
        let mut sum = 0;
        for i in 1..=(pos - num).abs() {
            sum += i;
        }
        sum + tot
    })
}

pub struct Day7 {}

impl Day for Day7 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let crab_positions = parse(input).collect::<Vec<i32>>();

        crab_positions.iter()
            .clone()
            .map(|p| fuel_to_pos(crab_positions.clone().into_iter(), *p))
            .fold(i32::MAX, |max, num| {
                if num < max {
                    return num
                }
                max
            }).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let crab_positions = parse(input).collect::<Vec<i32>>();

        crab_positions.iter()
            .clone()
            .map(|p| fuel_to_pos_2(crab_positions.clone().into_iter(), *p))
            .fold(i32::MAX, |max, num| {
                if num < max {
                    return num
                }
                max
            }).to_string()
    }
}