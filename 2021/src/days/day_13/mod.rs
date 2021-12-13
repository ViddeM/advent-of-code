use crate::days::common::Day;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
enum FoldDir {
    X(u32),
    Y(u32),
}

type Pos = (u32, u32);

fn parse(input: &str) -> (Vec<Pos>, Vec<FoldDir>) {
    let mut positions = Vec::new();
    let mut folds = Vec::new();

    let mut parse_pos = true;
    input.lines().for_each(|l| {
        if l.is_empty() {
            parse_pos = false;
        } else {
            if parse_pos {
                let (x, y) = l.split_once(",").unwrap();
                positions.push((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
            } else {
                let (dir, num_str) = l.split_once("=").unwrap();
                let num = num_str.parse::<u32>().unwrap();
                if dir == "fold along y" {
                    folds.push(FoldDir::Y(num))
                } else {
                    folds.push(FoldDir::X(num))
                }
            }
        }
    });

    (positions, folds)
}

pub struct Day13 {}

impl Day for Day13 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let (mut positions, instructions) = parse(input);

        let instruction = instructions[0].clone();
        match instruction {
            FoldDir::X(n) => {
                positions = positions
                    .into_iter()
                    .map(|(x, y)| {
                        if x > n {
                            return (x - 2 * (x - n), y);
                        }
                        (x, y)
                    })
                    .collect()
            }
            FoldDir::Y(n) => {
                positions = positions
                    .into_iter()
                    .map(|(x, y)| {
                        if y > n {
                            return (x, y - 2 * (y - n));
                        }
                        (x, y)
                    })
                    .collect()
            }
        }

        let mut set = HashSet::new();
        positions.into_iter().for_each(|(x, y)| {
            set.insert((x, y));
        });
        set.len().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (mut positions, instructions) = parse(input);
        for instruction in instructions {
            match instruction {
                FoldDir::X(n) => {
                    positions = positions
                        .into_iter()
                        .map(|(x, y)| {
                            if x > n {
                                return (x - 2 * (x - n), y);
                            }
                            (x, y)
                        })
                        .collect()
                }
                FoldDir::Y(n) => {
                    positions = positions
                        .into_iter()
                        .map(|(x, y)| {
                            if y > n {
                                return (x, y - 2 * (y - n));
                            }
                            (x, y)
                        })
                        .collect()
                }
            }
        }

        let mut max_x = 0;
        let mut max_y = 0;
        let mut set = HashSet::new();
        positions.into_iter().for_each(|(x, y)| {
            set.insert((x, y));
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        });
        max_x += 1; // Zero-indexed;
        max_y += 1;

        let mut map = Vec::new();
        for _ in 0..(max_x * max_y) {
            map.push('.');
        }

        for (x, y) in set.into_iter() {
            map[(x + y * max_x) as usize] = '#'
        }

        let mut map_str = String::new();
        for (i, c) in map.into_iter().enumerate() {
            map_str.push(c);
            if (i as u32) % max_x == 0 && i != 0 {
                map_str.push('\n');
            }
        }

        println!("Map: \n{}", map_str);

        "".to_string()
    }
}
