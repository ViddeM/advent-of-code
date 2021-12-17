use crate::days::common::Day;
use std::cmp::max;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos: {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn parse(input: &str) -> (Pos, Pos) {
    let (_, vals) = input.trim().split_once(": ").unwrap();
    let (x_range, y_range) = vals.split_once(", ").unwrap();
    let (min_x, max_x) = x_range.split_once("=").unwrap().1.split_once("..").unwrap();
    let (min_y, max_y) = y_range.split_once("=").unwrap().1.split_once("..").unwrap();
    return (
        Pos {
            x: min_x.parse().unwrap(),
            y: min_y.parse().unwrap(),
        },
        Pos {
            x: max_x.parse().unwrap(),
            y: max_y.parse().unwrap(),
        },
    );
}

pub struct Day17 {}

impl Day for Day17 {
    #[allow(unused_must_use)]
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let (min, max) = parse(input);
        let mut highest = 0;
        for x in 0..=max.x {
            for y in 0..=1000 {
                let test_velocity = Pos {
                    x: x.clone(),
                    y: y.clone(),
                };
                if let Some(h) = test_init_velocity(test_velocity.clone(), min.clone(), max.clone())
                {
                    if h > highest {
                        highest = h;
                    }
                }
            }
        }
        highest.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (min, max) = parse(input);
        let mut ends_in_area = Vec::new();
        for x in 0..=max.x {
            for y in -1000..=1000 {
                let test_velocity = Pos {
                    x: x.clone(),
                    y: y.clone(),
                };
                if let Some(h) = test_init_velocity(test_velocity.clone(), min.clone(), max.clone())
                {
                    ends_in_area.push(h);
                }
            }
        }
        ends_in_area.len().to_string()
    }
}

fn test_init_velocity(velocity: Pos, area_min: Pos, area_max: Pos) -> Option<i32> {
    let steps = 5000;
    let mut highest = velocity.y;
    let mut pos = Pos { x: 0, y: 0 };
    let mut new_velocity = velocity.clone();
    for _ in 0..steps {
        pos = Pos {
            x: max(pos.x + new_velocity.x, 0),
            y: pos.y.clone() + new_velocity.y,
        };
        new_velocity = Pos {
            x: max(new_velocity.x - 1, 0),
            y: new_velocity.y - 1,
        };

        if pos.y > highest {
            highest = pos.y.clone();
        }

        if pos.x > area_max.x || pos.y < area_min.y {
            return None;
        }

        if pos.x >= area_min.x && pos.y <= area_max.y {
            return Some(highest);
        }
    }
    None
}
