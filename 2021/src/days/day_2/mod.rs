use crate::days::common::Day;
use std::str;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=(i32, i32)> + 'a {
    input
        .lines()
        .map(|v| {
            let mut space_index = usize::MAX;
            {
                let chars = v.chars();
                for (i, c) in chars.enumerate() {
                    if c == ' ' {
                        space_index = i;
                    }
                }
            }

            let mut chars = v.chars();
            let first_char = chars.next().unwrap();
            let num = chars.skip(space_index).collect::<String>().parse::<i32>().unwrap();
            if first_char == 'd' {
                (0, num)
            } else if first_char == 'f' {
                (num, 0)
            } else {
                (0, -num)
            }
        })
}

pub struct Day2 {}

impl Day for Day2 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let dirs = parse(input);
        let (horizontal, depth) = dirs
            .reduce(|(h, d), (sum_h, sum_d)| (sum_h + h, sum_d + d))
            .expect("Reduce failed");
        (depth * horizontal).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let dirs = parse(input);
        let (horizontal, depth, _) = dirs.fold((0, 0, 0), |(sum_h, sum_d, a), (h, d)| {
            if h != 0 {
                (sum_h + h, sum_d + h * a, a)
            } else {
                (sum_h, sum_d, a + d)
            }
        });
        (depth * horizontal).to_string()
    }
}