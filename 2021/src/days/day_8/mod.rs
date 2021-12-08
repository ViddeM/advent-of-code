use std::collections::HashMap;
use std::ops::Deref;
use crate::days::common::Day;

fn parse<'a>(input: &'a str) -> impl Iterator<Item=(Vec<String>, Vec<String>)> + 'a {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" | ");
            let inputs = parts
                .nth(0)
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();
            let outputs = parts.nth(0)
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();
            (inputs, outputs)
        })
}

pub struct Day8 {}

impl Day for Day8 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let entries = parse(input);
        let count = entries.fold(0, |sum, (_, outputs)| {
           outputs.iter().fold(0, |inner_sum , word| {
               match word.len() {
                   2 => inner_sum + 1, // 1
                   3 => inner_sum + 1, // 7
                   4 => inner_sum + 1, // 4
                   7 => inner_sum + 1, // 8
                   _ => inner_sum,
               }
           }) + sum
        });

        count.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let entries = parse(input);
        let sum = entries
            .map(|(i, o)| {
            let map = map_digits(i);
            let digits: Vec<u8> = o
                .into_iter()
                .map(|v| {
                    let sorted_v = string_to_sorted(v.clone());
                    map.get(&sorted_v).unwrap().clone()
                }).collect();
            digits
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, d)| {
                    (d as u32) * (10 as u32).pow((digits.len() - i - 1) as u32)
            })
                .fold(0, |sum, num| sum + num)
        })
            .fold(0, |sum, v| sum + v);

        sum.to_string()
    }
}

fn map_digits(input: Vec<String>) -> HashMap<String, u8> {
    let mut map: HashMap<u8, String> = HashMap::new();
    let mut zero_six_or_nine: Vec<String> = Vec::new();
    let mut two_three_or_five: Vec<String> = Vec::new();

    for combination in input.clone().into_iter() {
        match combination.len() {
            2 => {map.insert(1, combination);}
            3 => {map.insert(7, combination);}
            4 => {map.insert(4, combination);},
            5 => {two_three_or_five.push(combination);},
            6 => {zero_six_or_nine.push(combination);},
            7 => {map.insert(8, combination);},
            _ => {}
        }
    };

    let mut upper_one_segment: char = ' ';
    zero_six_or_nine
        .into_iter()
        .for_each(|v| {
            let is_zero_or_nine = map.get(&1).unwrap().clone().chars().into_iter().fold(true, |contains, c| {
                if !contains {
                    false
                } else {
                    if v.contains(c) {
                        true
                    } else {
                        upper_one_segment = c;
                        false
                    }
                }
            });

            if !is_zero_or_nine {
                map.insert(6, v);
            } else {
                let is_nine = map.get(&4).unwrap().clone().chars().into_iter().fold(true, |contains, c| {
                    if !contains {
                        false
                    } else {
                        v.contains(c)
                    }
                });

                if is_nine {
                    map.insert(9, v);
                } else {
                    map.insert(0, v);
                }
            }
        });

    for v in two_three_or_five.into_iter() {
        if !v.contains(upper_one_segment) {
            map.insert(5, v);
        } else {
            if map.get(&1).unwrap().chars().fold(true, |contains, c| {
                if !contains {
                    false
                } else {
                    v.contains(c)
                }
            }) {
                map.insert(3, v);
            } else {
                map.insert(2, v);
            }
        }
    }

    let mut return_map: HashMap<String, u8> = HashMap::new();
    map.into_iter().for_each(|(k, v)| {
        return_map.insert(string_to_sorted(v), k);
    });
    return_map
}

fn string_to_sorted(v: String) -> String {
    let mut s = v.chars().collect::<Vec<char>>();
    s.sort_by(|a, b| b.cmp(a));
    String::from_iter(s)
}