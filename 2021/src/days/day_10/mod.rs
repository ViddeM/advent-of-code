use crate::days::common::Day;

enum LineState {
    Corrupted(char), // The first illegal character in the line
    Incomplete(String), // The missing string to complete the string
}

impl LineState {
    fn is_corrupted(&self) -> bool {
        match self {
            LineState::Corrupted(_) => true,
            _ => false
        }
    }

    fn is_incomplete(&self) -> bool {
        match self {
            LineState::Incomplete(_) => true,
            _ => false
        }
    }

    fn unwrap_corrupted(&self) -> char {
        match self {
            LineState::Corrupted(c) => return c.clone(),
            _ => panic!("Not an incomplete!"),
        }
    }

    fn unwrap_incomplete(&self) -> String {
        match self {
            LineState::Incomplete(s) => return s.clone(),
            _ => panic!("Not an incomplete!"),
        }
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item=&str> + 'a {
    input
        .lines()
}

pub struct Day10 {}

impl Day for Day10 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let lines = parse(input);
        let sum = lines
            .map(|l| corrupted_or_incomplete(l))
            .filter(|v| v.is_corrupted() )
            .map(|v| v.unwrap_corrupted())
            .map(|c| {
                match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("Unexpected char {}", c)
                }
            }).fold(0, |sum, num| num + sum);

        sum.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let lines = parse(input);
        let mut scores: Vec<u64> = lines
            .map(|l| corrupted_or_incomplete(l))
            .filter(|v| v.is_incomplete() )
            .map(|v| v.unwrap_incomplete())
            .map(|s| {
                s
                    .chars()
                    .map(| c| {
                        match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!("Invalid {}", c)
                        }
                    })
                    .fold(0, |sum, val| sum * 5 + val)
            }).collect();
        scores.sort();
        scores[(scores.len() - 1) / 2].to_string()
    }
}

fn corrupted_or_incomplete(line: &str) -> LineState {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '{' | '[' | '(' | '<' => stack.push(c),
            '}' | ']' | ')' | '>' => {
                match stack.last() {
                    Some(other) => {
                        if compare_bracket_pair(*other, c) {
                            stack.pop();
                        } else {
                            return LineState::Corrupted(c)
                        }
                    },
                    None => return LineState::Corrupted(c)
                }
            },
            uk => panic!("Unknown character received! {}", uk)
        }
    }

    let mut remaining = String::new();
    while let Some(a) = stack.pop() {
        match a {
            '{' => remaining.push('}'),
            '[' => remaining.push(']'),
            '(' => remaining.push(')'),
            '<' => remaining.push('>'),
            _ => panic!("Unknown smth: {}", a)
        }
    }
    LineState::Incomplete(remaining)
}

fn compare_bracket_pair(open: char, close: char) -> bool {
    match open {
        '{' => return close == '}',
        '[' => return close == ']',
        '<' => return close == '>',
        '(' => return close == ')',
        _ => panic!("The fuck?? {}", open)
    }
}