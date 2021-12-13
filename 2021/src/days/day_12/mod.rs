use crate::days::common::Day;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

    input
        .trim()
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .for_each(|(from, to)| {
            if let Some(v) = caves.get(from) {
                let mut vs = v.clone();
                vs.push(to);
                caves.insert(from, vs);
            } else {
                let mut v = Vec::new();
                v.push(to);
                caves.insert(from, v);
            }

            if let Some(v) = caves.get(to) {
                let mut vs = v.clone();
                vs.push(from);
                caves.insert(to, vs);
            } else {
                let mut v = Vec::new();
                v.push(from);
                caves.insert(to, v);
            }
        });

    caves
}

pub struct Day12 {}

impl Day for Day12 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let caves = parse(input);

        let count = visit_cave("start", Vec::new(), &caves);

        count.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let caves = parse(input);

        let count = visit_cave_2("start", Vec::new(), false, &caves);

        count.to_string()
    }
}

fn is_small_cave(cave: &str) -> bool {
    if let Some(c) = cave.chars().nth(0) {
        c.is_lowercase()
    } else {
        false
    }
}

fn visit_cave(curr: &str, visited_small: Vec<&str>, caves_map: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut new_visited = visited_small.clone();
    if is_small_cave(curr) {
        new_visited.push(curr);
    }

    let mut sum = 0;
    let neighbours = caves_map.get(curr).unwrap();
    for neighbour in neighbours {
        if new_visited.contains(neighbour) == false {
            sum += if *neighbour == "end" {
                1
            } else {
                visit_cave(neighbour, new_visited.clone(), caves_map)
            }
        }
    }

    sum
}

fn visit_cave_2(
    curr: &str,
    visited_small: Vec<&str>,
    small_visited_twice: bool,
    caves_map: &HashMap<&str, Vec<&str>>,
) -> u32 {
    let mut new_visited = visited_small.clone();
    let mut visited_twice = small_visited_twice;
    if new_visited.contains(&curr) {
        visited_twice = true;
    } else {
        if is_small_cave(curr) {
            new_visited.push(curr);
        }
    }

    let mut sum = 0;
    let neighbours = caves_map.get(curr).unwrap();

    for neighbour in neighbours {
        if *neighbour == "start" {
            continue;
        }

        if new_visited.contains(neighbour) && visited_twice {
            continue;
        }

        if *neighbour == "end" {
            sum += 1
        } else {
            sum += visit_cave_2(neighbour, new_visited.clone(), visited_twice, caves_map)
        }
    }

    sum
}
