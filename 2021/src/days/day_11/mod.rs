use crate::days::common::Day;
use std::ops::Deref;

struct Cave {
    octopuses: Vec<u8>,
    width: i32,
    height: i32,
}

fn parse<'a>(input: &'a str) -> Cave {
    let mut width = 0;
    let lines = input.trim().lines().map(|l| {
        width = l.len();
        l.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
    });
    let mut height = 0;
    Cave {
        octopuses: lines.fold(Vec::new(), |v, l| {
            height += 1;
            let mut v2 = v.clone();
            l.iter().for_each(|c| v2.push(*c));
            v2
        }),
        width: width as i32,
        height: height as i32,
    }
}

pub struct Day11 {}

impl Day for Day11 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let mut cave = parse(input);
        let mut num_flashes = 0;
        for step in 1..=100 {
            num_flashes += run_step(&mut cave);
        }
        num_flashes.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut cave = parse(input);
        first_all_flash_step(&mut cave).to_string()
    }
}

fn first_all_flash_step(cave: &mut Cave) -> u32 {
    let num_octopus = cave.octopuses.len() as u32;
    for step in 1..=1000 {
        let num_flahes = run_step(cave);
        if num_flahes == num_octopus {
            return step;
        }
    }
    0
}

fn run_step(cave: &mut Cave) -> u32 {
    cave.octopuses = cave.octopuses.clone().into_iter().map(|n| n + 1).collect();

    let mut num_flashes = 0;
    for x in 0..cave.width {
        for y in 0..cave.height {
            if let Some(octopus) = octopus_at(x, y, cave) {
                if octopus > 9 {
                    num_flashes += flash(x, y, cave);
                }
            }
        }
    }

    num_flashes
}

fn octopus_at(x: i32, y: i32, cave: &mut Cave) -> Option<u8> {
    if x < 0 || y < 0 || x >= cave.width || y >= cave.height {
        return None;
    }
    Some(cave.octopuses[to_index(x, y, cave.width)])
}

fn flash(x: i32, y: i32, cave: &mut Cave) -> u32 {
    cave.octopuses[to_index(x, y, cave.width)] = 0;
    1
        + check_flash(x, y - 1, cave) // Above
        + check_flash(x + 1, y - 1, cave) // Above right
        + check_flash(x + 1, y, cave) // Right
        + check_flash(x + 1, y + 1, cave) // Lower right
        + check_flash(x, y + 1, cave) // Below
        + check_flash(x - 1, y + 1, cave) // Lower left
        + check_flash(x - 1, y, cave) // Left
        + check_flash(x - 1, y - 1, cave) // Above left
}

fn check_flash(x: i32, y: i32, cave: &mut Cave) -> u32 {
    if let Some(octopus) = octopus_at(x, y, cave) {
        if octopus == 0 {
            return 0;
        }

        let new_octopus = octopus + 1;
        cave.octopuses[to_index(x, y, cave.width)] = new_octopus;
        if new_octopus > 9 {
            return flash(x, y, cave);
        }
    }
    0
}

fn to_index(x: i32, y: i32, width: i32) -> usize {
    (x + y * width) as usize
}

fn print_cave(cave: &Cave) {
    for y in 0..cave.height {
        let mut row = String::new();
        for x in 0..cave.width {
            row.push_str(&format!("{}", cave.octopuses[to_index(x, y, cave.width)]));
        }
        println!("{}", row);
    }
}
