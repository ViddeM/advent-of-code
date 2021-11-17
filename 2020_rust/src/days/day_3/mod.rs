use crate::days::common::{AOCInput};

#[derive(Clone, Debug)]
pub enum Tile {
    Open,
    Tree,
}

type World = Vec<Vec<Tile>>;

fn get_at(world: &World, x: usize, y: usize) -> Tile {
    let row = &world[y];
    row[x % row.len()].clone()
}

pub fn parse(input_name: &str) -> World {
    let rows = input_name
        .split("\n")
        .filter(|r| r.is_empty() == false)
        .collect::<Vec<&str>>();
    let mut world: World = vec![];
    for row in rows {
        let mut world_row: Vec<Tile> = Vec::new();
        for tile in row.chars() {
            world_row.push(match tile {
                '.' => Tile::Open,
                '#' => Tile::Tree,
                _ => panic!("INVALID CHAR!!"),
            })
        }
        world.push(world_row);
    }
    world
}

fn test_slope(world: &World, move_x: &usize, move_y: &usize) -> u32 {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut tree_count = 0;
    while pos_y < world.len() - 1 {
        pos_x += move_x;
        pos_y += move_y;
        tree_count += match get_at(&world, pos_x, pos_y) {
            Tile::Open => 0,
            Tile::Tree => 1,
        }
    }
    tree_count
}

pub fn part_1(input_name: &str) -> u32 {
    let world = parse(input_name);
    test_slope(&world, &3, &1)
}

pub fn part_2(input_name: &str) -> u32 {
    let world = parse(input_name);
    let slopes: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    slopes.iter().map(|(x, y)| test_slope(&world, x, y)).product()
}


pub fn run(input: AOCInput) -> () {
    let data = match input {
        AOCInput::Input(input) => input,
        AOCInput::Test => include_str!("test_input.txt"),
    };

    let part_one = part_1(data);
    println!("Solution to part 1: {}", part_one);
    let part_two = part_2(data);
    println!("Solution to part 2: {}", part_two);
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