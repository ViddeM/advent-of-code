use crate::days::common::{AOCInput};

struct Policy {
    min: u32,
    max: u32,
    letter: char,
}

impl Policy {
    fn parse(input: &str) -> Policy {
        let parts = input.split(" ").collect::<Vec<&str>>();
        let span = parts[0].split("-").collect::<Vec<&str>>();
        let min = span[0].parse::<u32>().expect("Failed to parse min");
        let max = span[1].parse::<u32>().expect("Failed to parse max");
        let letter = parts[1].parse::<char>().expect("Failed to parse char");

        Policy {
            min,
            max,
            letter
        }
    }
}

pub struct Password {
    policy: Policy,
    password: String,
}

impl Password {
    fn parse(input: &str) -> Password {
        let parts = input.split(":").collect::<Vec<&str>>();
        let policy = Policy::parse(parts[0]);
        let password = parts[1].trim().to_string();

        Password {
            policy,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        let occurances = self.password
            .chars()
            .filter(|c| c.eq(&self.policy.letter))
            .collect::<Vec<char>>()
            .len() as u32;
        self.policy.min <= occurances && occurances <= self.policy.max
    }

    fn is_valid_2(&self) -> bool {
        let chars = self.password.chars();
        let a = chars.clone().nth((self.policy.min - 1) as usize).expect("No char at min");
        let b = chars.clone().nth((self.policy.max - 1) as usize).expect("No char at max");
        (a.eq(&self.policy.letter) || b.eq(&self.policy.letter)) && a.eq(&b) == false
    }
}

pub fn parse(input_name: &str) -> Vec<Password> {
    input_name
        .split("\n")
        .filter(|v| v.is_empty() == false)
        .map(|val| Password::parse(val))
        .collect()
}

pub fn part_1(input_name: &str) -> u32 {
    let data = parse(input_name);
    data
        .into_iter()
        .filter(|p| p.is_valid())
        .collect::<Vec<Password>>()
        .len() as u32
}

pub fn part_2(input_name: &str) -> u32 {
    let data = parse(input_name);
    data
        .into_iter()
        .filter(|p| p.is_valid_2())
        .collect::<Vec<Password>>()
        .len() as u32
}


pub fn run(input: AOCInput) -> () {
    let data = match input {
        AOCInput::Input(input) => input,
        AOCInput::Test => include_str!("test_input.txt"),
    };

    let part_one = part_1(data);
    println!("Solution to part one: {}", part_one);
    let part_two = part_2(data);
    println!("Solution to part two: {}", part_two);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_parse(b: &mut Bencher) {
        let data = include_str!("input.txt");
        b.iter(|| parse(data))
    }

    #[bench]
    pub fn bench_part_1(b: &mut Bencher) {
        let data = include_str!("input.txt");
        b.iter(|| part_1(data))
    }

    #[bench]
    pub fn bench_part_2(b: &mut Bencher) {
        let data = include_str!("input.txt");
        b.iter(|| part_2(data))
    }
}