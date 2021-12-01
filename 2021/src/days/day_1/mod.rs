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
        let mut prev_sum = 0;
        let mut increased = 0;
        for (i, num) in nums.iter().enumerate() {
            if i >= 2 {
                let new_sum = num + nums[i - 1] + nums[i - 2];
                if prev_sum > 0 && new_sum > prev_sum {
                    increased += 1;
                }
                prev_sum = new_sum;
            }
        }
        increased.to_string()
    }
}
