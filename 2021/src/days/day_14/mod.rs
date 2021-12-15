use crate::days::common::Day;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<char>, Vec<(char, char, char)>) {
    let mut lines = input.lines();
    let polymer = lines.nth(0).unwrap().chars().into_iter().collect();
    let trans = lines
        .skip(1)
        .map(|l| {
            let mut chars = l.chars();
            (
                chars.nth(0).unwrap(),
                chars.nth(0).unwrap(),
                chars.last().unwrap(),
            )
        })
        .collect();
    (polymer, trans)
}

pub struct Day14 {}

impl Day for Day14 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let (mut polymer, transformations) = parse(input);
        let mut new_polymer: Vec<char>;
        for _ in 1..=10 {
            new_polymer = Vec::new();
            for i in 0..polymer.len() {
                let c = polymer[i];
                new_polymer.push(c);

                if i < polymer.len() - 1 {
                    let c2 = polymer[i + 1];
                    for (b1, b2, res) in transformations.iter() {
                        if *b1 == c && *b2 == c2 {
                            new_polymer.push(*res);
                        }
                    }
                }
            }
            polymer = new_polymer;
        }

        let map = polymer.into_iter().fold(HashMap::new(), |mut map, c| {
            map.insert(
                c,
                match map.get(&c) {
                    Some(v) => v + 1,
                    None => 1,
                },
            );
            map
        });

        let (mut highest, mut highest_count) = (' ', 0);
        let (mut lowest, mut lowest_count) = (' ', u32::MAX);

        map.into_iter().for_each(|(k, v)| {
            if v > highest_count {
                highest = k;
                highest_count = v;
            }

            if v < lowest_count {
                lowest = k;
                lowest_count = v;
            }
        });

        (highest_count - lowest_count).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (polymer, transformations) = parse(input);
        let mut polymer_map: HashMap<(char, char), u64> = HashMap::new();
        for i in 0..(polymer.len() - 1) {
            let pair = (polymer[i], polymer[i + 1]);
            polymer_map.insert(
                pair,
                match polymer_map.get(&pair) {
                    None => 1,
                    Some(v) => v + 1,
                },
            );
        }

        let mut start_pair = (polymer[0], polymer[1]);

        for _ in 1..=40 {
            let mut new_map = polymer_map.clone();
            for (c, c2) in polymer_map.clone().keys() {
                let orig_pair = (*c, *c2);
                let orig_pair_count = polymer_map.get(&orig_pair).unwrap();
                for (b1, b2, res) in transformations.iter() {
                    if *c == *b1 && *c2 == *b2 {
                        if let Some(v) = new_map.get(&orig_pair) {
                            let vc = v.clone();
                            if vc <= *orig_pair_count {
                                new_map.remove(&orig_pair);
                            } else {
                                new_map.insert(orig_pair, vc - orig_pair_count);
                            }
                        }

                        let pair_a = (*c, *res);
                        if orig_pair == start_pair {
                            start_pair = pair_a;
                        }

                        let pair_b = (*res, *c2);

                        new_map.insert(
                            pair_a,
                            match new_map.get(&pair_a) {
                                Some(v) => v + orig_pair_count,
                                None => *orig_pair_count,
                            },
                        );

                        new_map.insert(
                            pair_b,
                            match new_map.get(&pair_b) {
                                Some(v) => v + orig_pair_count,
                                None => *orig_pair_count,
                            },
                        );
                    }
                }
            }

            polymer_map = new_map;
        }

        let map = polymer_map
            .into_iter()
            .fold(HashMap::new(), |mut map, ((c1, c2), v)| {
                if (c1, c2) == start_pair {
                    map.insert(
                        c1,
                        match map.get(&c1) {
                            Some(v2) => v2 + 1,
                            None => 1,
                        },
                    );
                }

                map.insert(
                    c2,
                    match map.get(&c2) {
                        Some(v2) => v + v2,
                        None => v,
                    },
                );
                map
            });

        let (mut highest, mut highest_count) = (' ', 0);
        let (mut lowest, mut lowest_count) = (' ', u64::MAX);

        map.into_iter().for_each(|(k, v)| {
            if v > highest_count {
                highest = k;
                highest_count = v;
            }

            if v < lowest_count {
                lowest = k;
                lowest_count = v;
            }
        });

        (highest_count - lowest_count).to_string()
    }
}
