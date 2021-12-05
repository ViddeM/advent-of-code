use std::collections::HashMap;
use crate::days::common::Day;

type Line = (Coord, Coord);

type Coord = (u32, u32);

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Line> + 'a {
    input
        .lines()
        .map(|v| {
            let (part1, part2) = v.split_once(" -> ").unwrap();
            (parse_coord(part1), parse_coord(part2))
        })
}

fn parse_coord(input: &str) -> Coord {
    let (num1, num2) = input.split_once(",").unwrap();
    (num1.parse::<u32>().unwrap(), num2.parse::<u32>().unwrap())
}

pub struct Day5 {}

impl Day for Day5 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let lines = parse(input);
        let mut worldMap: HashMap<Coord, u32> = HashMap::new();
        lines.for_each(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
                let (start, end) = if y2 > y1  {
                    (y1, y2)
                } else {
                    (y2, y1)
                };
                for y in start..=end {
                    let c = (x1, y);
                    match worldMap.get(&c) {
                        None => worldMap.insert(c, 1),
                        Some(v) => worldMap.insert(c, v + 1),
                    };
                }
            } else if y1 == y2 {
                let (start, end) = if x2 > x1 {
                    (x1, x2)
                } else {
                    (x2, x1)
                };
                for x in start..=end {
                    let c = (x, y1);
                    match worldMap.get(&c) {
                        None => worldMap.insert(c, 1),
                        Some(v) => worldMap.insert(c, v + 1),
                    };
                }
            }
        });
        worldMap.values().filter(|v| v >= &&2).count().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let lines = parse(input);
        let mut world_map: HashMap<Coord, u32> = HashMap::new();
        lines.for_each(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
                let (start, end) = if y2 > y1  {
                    (y1, y2)
                } else {
                    (y2, y1)
                };
                for y in start..=end {
                    let c = (x1, y);
                    match world_map.get(&c) {
                        None => world_map.insert(c, 1),
                        Some(v) => world_map.insert(c, v + 1),
                    };
                }
            } else if y1 == y2 {
                let (start, end) = if x2 > x1 {
                    (x1, x2)
                } else {
                    (x2, x1)
                };
                for x in start..=end {
                    let c = (x, y1);
                    match world_map.get(&c) {
                        None => world_map.insert(c, 1),
                        Some(v) => world_map.insert(c, v + 1),
                    };
                }
            } else {
                let (start_x, start_y, y_diff) = if x1 > x2 {
                    if y1 > y2 {
                        (x2, y2, 1 as i32)
                    } else {
                        (x2, y2, -1 as i32)
                    }
                } else {
                    if y1 > y2 {
                        (x1, y1, -1 as i32)
                    } else {
                        (x1, y1, 1 as i32)
                    }
                };
                let steps = i32::abs(((x1 as i32) - (x2 as i32))) as u32;

                for i in 0..=steps {
                    let c = (start_x + i, (start_y as i32 + (y_diff * i as i32)) as u32);
                    // println!("Inserting diagonal on {:?} | Initial input ({}, {}) -> ({}, {})", c, x1, y1, x2, y2);
                    match world_map.get(&c) {
                        None => world_map.insert(c, 1),
                        Some(v) => world_map.insert(c, v + 1),
                    };
                }
            }
        });
        // println!("worldMap: {:?}", world_map);
        world_map.values().filter(|v| v >= &&2).count().to_string()
    }
}