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
        let nums = parse(input);
        let mut prevs: Vec<u32> = vec![];
        let mut increased = 0;
        nums.for_each(|v| {
            let mut new_prevs:Vec<u32> = prevs.clone();
            new_prevs.push(v);
            if new_prevs.len() > 3 {
                new_prevs.remove(0);
            }
            let old_sum: u32 = prevs.iter().sum();
            let new_sum: u32 = new_prevs.iter().sum();

            if prevs.len() == 3 && new_sum > old_sum {
                increased += 1;
            }

            prevs = new_prevs;
        });
        increased.to_string()
    }
}
