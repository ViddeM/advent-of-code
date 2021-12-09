use std::ops::Div;
use crate::days::common::Day;

#[derive(Clone, Debug)]
struct World {
    map: Vec<u8>,
    width: u32,
    height: u32,
}

fn parse<'a>(input: &'a str) -> World {
    let mut width = 0;
    let mut height = 0;
    let vec = input
        .lines()
        .map(|l| {
            width = l.len();
            l.chars().map(|c| (c as u8) - 48).collect::<Vec<u8>>()
        }).fold(Vec::new(), |v, ls| {
        let mut s =v.clone();
        s.extend(ls.into_iter());
        height = height + 1;
        s
    });

    World {
        map: vec,
        width: width as u32,
        height: height as u32,
    }
}

pub struct Day9 {}

impl Day for Day9 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let world = parse(input);
        world
            .map
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(index, _)| {
                let x = *index as u32 % world.width;
                let y = (*index as u32 - x) / world.width;
                is_smaller_than_neighbours(x, y, &world)
            })
            .map(|(_, i)| i + 1)
            .fold(0 as u64, |sum, i| sum + (i as u64)).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let world = parse(input);
        let mut low_spots = find_low_spots(&world);

        let mut low_spot_sizes = low_spots
            .into_iter()
            .map(|(x, y)| {
                get_low_spot_size(x, y, &world, &mut Vec::new())
            })
            .collect::<Vec<u32>>();
        low_spot_sizes.sort();
        low_spot_sizes.reverse();

        (low_spot_sizes[0] * low_spot_sizes[1] * low_spot_sizes[2]).to_string()
    }
}

fn is_smaller_than_neighbours(x: u32, y: u32, world: &World) -> bool {
    let point = get_at(x, y, world).unwrap();
    if let Some(v) = get_at(x + 1, y, world) {
        if v <= point {
            return false;
        }
    }
    if let Some(lower_x) = x.checked_sub(1) {
        if let Some(v) = get_at(lower_x, y, world) {
            if v <= point {
                return false;
            }
        }
    }
    if let Some(v) = get_at(x, y + 1, world) {
        if v <= point {
            return false;
        }
    }
    if let Some(lower_y) = y.checked_sub(1) {
        if let Some(v) = get_at(x, lower_y, world) {
            if v <= point {
                return false;
            }
        }
    }
    true
}

fn get_at(x: u32, y: u32, world: &World) -> Option<u8> {
    if x < 0
        || y < 0
        || x >= world.width
        || y >= world.height {
        return None
    }
    let index = (x + (y * world.width)) as usize;
    if index >= world.map.len() {
        return None
    }
    Some(world.map[index])
}

fn find_low_spots(world: &World) -> Vec<(u32, u32)> {
    world
        .map
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(index, _)| {
            let x = *index as u32 % world.width;
            let y = (*index as u32 - x) / world.width;
            is_smaller_than_neighbours(x, y, &world)
        })
        .map(|(index, _)| {
            let x = index as u32 % world.width;
            let y = (index as u32 - x) / world.width;
            (x, y)
        })
        .collect()
}

fn get_low_spot_size(x: u32, y: u32, world: &World, checked: &mut Vec<u32>) -> u32 {
    checked.push(y * world.width + x);
    let val = get_at(x, y, world);
    match val {
        None => 0,
        Some(v) => {
            let s = v as u32;
            if s == 9 {
                return 0
            }

            let mut left = 0;
            if let Some(lower_y) = y.checked_sub(1) {
                if checked.contains(&(x + lower_y * world.width)) == false {
                    left = get_low_spot_size(x, lower_y, world, checked);
                }
            }
            let mut up = 0;
            if let Some(lower_x) = x.checked_sub(1) {
                if checked.contains(&(lower_x + y * world.width)) == false {
                    up = get_low_spot_size(lower_x, y, world, checked);
                }
            }
            let mut right = 0;
            if checked.contains(&(x + 1 + y * world.width)) == false {
                right = get_low_spot_size(x + 1, y, world, checked);
            }
            let mut down = 0;
            if checked.contains(&(x + (y + 1) * world.width)) == false {
                down = get_low_spot_size(x, y + 1, world, checked);
            }

            1 + up + left + right + down
        }
    }
}