use crate::days::common::Day;
use std::fmt::{Display, Formatter};
use std::str::Chars;

#[derive(Clone, Debug)]
enum Val {
    Regular(u32),
    Pair(Box<Val>, Box<Val>),
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Regular(n) => write!(f, "{}", n),
            Val::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

impl Val {
    fn add(&self, other: Val) -> Val {
        Val::Pair(Box::new(self.clone()), Box::new(other.clone()))
    }

    fn add_left(&self, num: u32) -> Val {
        match self {
            Val::Regular(n) => Val::Regular(n + num),
            Val::Pair(a, b) => Val::Pair(Box::new(a.add_left(num)), b.clone()),
        }
    }

    fn add_right(&self, num: u32) -> Val {
        match self {
            Val::Regular(n) => Val::Regular(n + num),
            Val::Pair(a, b) => Val::Pair(a.clone(), Box::new(b.add_right(num))),
        }
    }
}

fn parse(input: &str) -> Vec<Val> {
    input
        .trim()
        .lines()
        .map(|l| parse_rec(&mut l.chars()))
        .collect()
}

fn parse_rec(part: &mut Chars) -> Val {
    let first = part.next().unwrap();
    let left = match first {
        '[' => parse_rec(part),
        n => Val::Regular(n.to_digit(10).unwrap()),
    };

    if let Some(c) = part.next() {
        if c != ',' {
            panic!("Parsing failed, expected ',', got '{}'", c);
        }
    } else {
        return left;
    }

    let second = part.next().unwrap();

    let right = match second {
        '[' => parse_rec(part),
        n => Val::Regular(n.to_digit(10).unwrap()),
    };

    let c_2 = part.next().unwrap();
    if c_2 != ']' {
        panic!("Popped invalid character: {}, expected ']'", c_2);
    }
    return Val::Pair(Box::new(left), Box::new(right));
}

pub struct Day18 {}

impl Day for Day18 {
    #[allow(unused_must_use)]
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let mut vals = parse(input);
        vals.reverse();

        while vals.len() > 1 {
            let first = vals.pop().unwrap();
            let second = vals.pop().unwrap();
            let added = first.add(second.clone());
            let sum = reduce(added.clone());
            vals.push(sum);
        }

        calc_magnitude(vals.pop().unwrap()).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let vals = parse(input);

        let mut largest = 0;
        for (i1, v1) in vals.iter().enumerate() {
            for (i2, v2) in vals.iter().enumerate() {
                if i1 == i2 {
                    // Don't add to itself.
                    continue;
                }

                let sum = calc_magnitude(reduce(v1.add(v2.clone())));
                if sum > largest {
                    largest = sum;
                }
            }
        }

        largest.to_string()
    }
}

fn reduce(val: Val) -> Val {
    let (mut new_v, exploaded) = explode(val);
    if exploaded {
        new_v = reduce(new_v);
    }

    let (split_v, split) = split(new_v);
    if split {
        reduce(split_v)
    } else {
        split_v
    }
}

fn explode(val: Val) -> (Val, bool) {
    let (new_v, exploaded, _, _) = explode_rec(val, 0);
    return (new_v, exploaded);
}

fn explode_rec(val: Val, depth: u32) -> (Val, bool, u32, u32) {
    match val {
        Val::Regular(_) => (val, false, 0, 0),
        Val::Pair(a, b) => {
            if depth >= 4 {
                if let Val::Regular(left) = *a {
                    if let Val::Regular(right) = *b {
                        return (Val::Regular(0), true, left, right);
                    }
                }
            }

            let (left, left_exploded, left_left, left_right) = explode_rec(*a.clone(), depth + 1);
            let (right, right_exploded, right_left, right_right) =
                explode_rec(*b.clone(), depth + 1);
            if left_exploded {
                (
                    Val::Pair(Box::new(left), Box::new(b.add_left(left_right))),
                    true,
                    left_left,
                    0,
                )
            } else if right_exploded {
                (
                    Val::Pair(Box::new(a.add_right(right_left)), Box::new(right)),
                    true,
                    0,
                    right_right,
                )
            } else {
                (Val::Pair(a, b), false, 0, 0)
            }
        }
    }
}

fn split(val: Val) -> (Val, bool) {
    match val {
        Val::Regular(n) => {
            if n >= 10 {
                let new = (n as f32) / 2f32;
                (
                    Val::Pair(
                        Box::new(Val::Regular(new.floor() as u32)),
                        Box::new(Val::Regular(new.ceil() as u32)),
                    ),
                    true,
                )
            } else {
                (val, false)
            }
        }
        Val::Pair(a, b) => {
            let (new_a, did_split) = split(*a.clone());
            if did_split {
                (Val::Pair(Box::new(new_a), b), did_split)
            } else {
                let (new_b, did_split) = split(*b);
                (Val::Pair(a, Box::new(new_b)), did_split)
            }
        }
    }
}

fn calc_magnitude(val: Val) -> u32 {
    match val {
        Val::Regular(n) => n,
        Val::Pair(a, b) => 3 * calc_magnitude(*a) + 2 * calc_magnitude(*b),
    }
}
