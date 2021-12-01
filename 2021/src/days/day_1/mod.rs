use crate::days::common::Day;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=u32> + 'a {
    input
        .lines()
        .map(|s| s.parse::<u32>().expect("Fuck buddy"))
}

pub struct Day1 {}

impl Day for Day1 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let nums = parse(input);
        let mut prev = u32::MAX;
        let mut increased = 0;
        nums.for_each(|v| {
            if v > prev {
                increased += 1;
            }
            prev = v
        });
        increased.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let nums = parse(input).collect::<Vec<u32>>();
        let mut increased = 0;
        for i in 0..nums.len() {
            if i >= 2 {
                if nums[i] > nums[i - 2] {
                    increased += 1;
                }
            }
        }
        increased.to_string()
    }
}
