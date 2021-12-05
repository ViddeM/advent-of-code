use std::cmp::max;
use crate::days::common::Day;

const WORLD_SIZE: u16 = 999;

type Line = (Coord, Coord);

type Coord = (u16, u16);

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Line> + 'a {
    input
        .lines()
        .map(|v| {
            let (part1, part2) = v.split_once(" -> ").unwrap();
            let (x1, y1) = parse_coord(part1);
            let (x2, y2) = parse_coord(part2);

            ((x1, y1), (x2, y2))
        })
}

fn parse_coord(input: &str) -> Coord {
    let (num1, num2) = input.split_once(",").unwrap();
    (num1.parse::<u16>().unwrap(), num2.parse::<u16>().unwrap())
}

pub struct Day5 {}

impl Day for Day5 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let mut world = [0 as u16; WORLD_SIZE as usize * WORLD_SIZE as usize];

        let lines = parse(input);
        lines.for_each(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
                println!("A!");
                for i in 0..= (i16::abs(y2 as i16 - y1 as i16) as u16) {
                    let index = (x1 as usize) + (i as usize) * (WORLD_SIZE as usize);
                    world[index] = world[index] + 1;
                }
            } else if y1 == y2 {
                println!("B!");
                let (start_x, end_x) = if x1 > x2 {
                    (x2, x1)
                } else {
                    (x1, x2)
                };

                let base = y1 as u32 * WORLD_SIZE as u32;
                println!("start_x: {} | end_x: {}", start_x, end_x);
                for i in start_x..=end_x {
                    let index = (base + i as u32) as usize;
                    world[index] = world[index] + 1;
                }
            }
        });

        let mut counter = 0;
        for i in world {
            if i >= 2 {
                counter += 1;
            }
        }

        counter.to_string()

        // let mut world_map: HashMap<Coord, u32> = HashMap::new();
        // lines.iter().for_each(|((x1, y1), (x2, y2))| {
        //     if x1 == x2 {
        //         let (start, end) = if y2 > y1  {
        //             (y1, y2)
        //         } else {
        //             (y2, y1)
        //         };
        //         for y in start..=end {
        //             let c = (x1, y);
        //             match world_map.get(&c) {
        //                 None => world_map.insert(c, 1),
        //                 Some(v) => world_map.insert(c, v + 1),
        //             };
        //         }
        //     } else if y1 == y2 {
        //         let (start, end) = if x2 > x1 {
        //             (x1, x2)
        //         } else {
        //             (x2, x1)
        //         };
        //         for x in start..=end {
        //             let c = (x, y1);
        //             match world_map.get(&c) {
        //                 None => world_map.insert(c, 1),
        //                 Some(v) => world_map.insert(c, v + 1),
        //             };
        //         }
        //     }
        // });
        // world_map.values().filter(|v| v >= &&2).count().to_string()
        // "".to_string()
    }

    fn part_2(&self, input: &str) -> String {
        "".to_string()
        // let lines = parse(input);
        // let mut world_map: HashMap<Coord, u32> = HashMap::new();
        // lines.for_each(|((x1, y1), (x2, y2))| {
        //     if x1 == x2 {
        //         let (start, end) = if y2 > y1  {
        //             (y1, y2)
        //         } else {
        //             (y2, y1)
        //         };
        //         for y in start..=end {
        //             let c = (x1, y);
        //             match world_map.get(&c) {
        //                 None => world_map.insert(c, 1),
        //                 Some(v) => world_map.insert(c, v + 1),
        //             };
        //         }
        //     } else if y1 == y2 {
        //         let (start, end) = if x2 > x1 {
        //             (x1, x2)
        //         } else {
        //             (x2, x1)
        //         };
        //         for x in start..=end {
        //             let c = (x, y1);
        //             match world_map.get(&c) {
        //                 None => world_map.insert(c, 1),
        //                 Some(v) => world_map.insert(c, v + 1),
        //             };
        //         }
        //     } else {
        //         let (start_x, start_y, y_diff) = if x1 > x2 {
        //             if y1 > y2 {
        //                 (x2, y2, 1 as i32)
        //             } else {
        //                 (x2, y2, -1 as i32)
        //             }
        //         } else {
        //             if y1 > y2 {
        //                 (x1, y1, -1 as i32)
        //             } else {
        //                 (x1, y1, 1 as i32)
        //             }
        //         };
        //         let steps = i32::abs((x1 as i32) - (x2 as i32)) as u32;
        //
        //         for i in 0..=steps {
        //             let c = (start_x + i, (start_y as i32 + (y_diff * i as i32)) as u32);
        //             // println!("Inserting diagonal on {:?} | Initial input ({}, {}) -> ({}, {})", c, x1, y1, x2, y2);
        //             match world_map.get(&c) {
        //                 None => world_map.insert(c, 1),
        //                 Some(v) => world_map.insert(c, v + 1),
        //             };
        //         }
        //     }
        // });
        // // println!("worldMap: {:?}", world_map);
        // world_map.values().filter(|v| v >= &&2).count().to_string()
    }
}