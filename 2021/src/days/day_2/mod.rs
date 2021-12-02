use crate::days::common::Day;

enum Dir {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Dir> + 'a {
    input
        .lines()
        .map(|v| {
            let parts: Vec<&str> = v.split(" ").collect();
            let num = parts[1].parse::<u32>().expect("Fuck buddy");
            match parts[0] {
                "forward" => Dir::Forward(num),
                "down" => Dir::Down(num),
                "up" => Dir::Up(num),
                _ => panic!("Invalid dir!")
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
        let mut horizontal = 0;
        let mut depth = 0;
        dirs.for_each(|v| {
            match v {
                Dir::Forward(n) => horizontal += n,
                Dir::Up(n) => depth -= n,
                Dir::Down(n) => depth += n,
            };
        });

        (depth * horizontal).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let dirs = parse(input);
        let mut aim = 0;
        let mut horizontal = 0;
        let mut depth = 0;
        dirs.for_each(|v| {
            match v {
                Dir::Forward(n) => {
                    horizontal += n;
                    depth += aim * n;
                },
                Dir::Up(n) => aim -= n,
                Dir::Down(n) => aim += n,
            }
        });

        (depth * horizontal).to_string()
    }
}