use std::collections::HashMap;
use crate::days::common::Day;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=u32> + 'a {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<u32>().expect("Invalid digit"))
}

pub struct Day6 {}

impl Day for Day6 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let mut fishes = parse(input);

        let mut fish_map = HashMap::new();
        fishes.for_each(|fish| {
            match fish_map.get(&fish) {
                Some(f) => fish_map.insert(fish, f + 1),
                None => fish_map.insert(fish, 1),
            };
        });

        for day in 1..=80 {
            let mut new_fish_map: HashMap<u32, u64> = HashMap::new();
            for key in fish_map.keys() {
                if *key == 0 {
                    match new_fish_map.get(&8) {
                        Some(v) => new_fish_map.insert(8, v + fish_map[key]),
                        None => new_fish_map.insert(8, fish_map[key]),
                    };
                    match new_fish_map.get(&6) {
                        Some(v) => new_fish_map.insert(6, v + fish_map[key]),
                        None => new_fish_map.insert(6, fish_map[key]),
                    };
                } else {
                    match new_fish_map.get(&(key - 1)) {
                        None => {new_fish_map.insert(key - 1, fish_map[key])}
                        Some(v) => {new_fish_map.insert(key - 1, v + fish_map[key])}
                    };
                }
            }

            fish_map = new_fish_map;
        }

        fish_map.iter().fold(0, |p, (_, v)| p + v).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut fishes = parse(input);

        let mut fish_map = HashMap::new();
        fishes.for_each(|fish| {
           match fish_map.get(&fish) {
               Some(f) => fish_map.insert(fish, f + 1),
               None => fish_map.insert(fish, 1),
           };
        });

        for day in 1..=256 {
            let mut new_fish_map: HashMap<u32, u64> = HashMap::new();
            for key in fish_map.keys() {
                if *key == 0 {
                    match new_fish_map.get(&8) {
                        Some(v) => new_fish_map.insert(8, v + fish_map[key]),
                        None => new_fish_map.insert(8, fish_map[key]),
                    };
                    match new_fish_map.get(&6) {
                        Some(v) => new_fish_map.insert(6, v + fish_map[key]),
                        None => new_fish_map.insert(6, fish_map[key]),
                    };
                } else {
                    match new_fish_map.get(&(key - 1)) {
                        None => {new_fish_map.insert(key - 1, fish_map[key])}
                        Some(v) => {new_fish_map.insert(key - 1, v + fish_map[key])}
                    };
                }
            }

            fish_map = new_fish_map;
        }

        fish_map.iter().fold(0, |p, (_, v)| p + v).to_string()
    }
}