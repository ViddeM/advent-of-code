use crate::days::common::Day;
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

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Password> + 'a {
    input
        .split("\n")
        .filter(|v| v.is_empty() == false)
        .map(|val| Password::parse(val))
}

pub struct Day2 {}

impl Day for Day2 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let data = parse(input);
        data
            .filter(|p| p.is_valid())
            .collect::<Vec<Password>>()
            .len().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let data = parse(input);
        data
            .filter(|p| p.is_valid_2())
            .collect::<Vec<Password>>()
            .len().to_string()
    }
}
