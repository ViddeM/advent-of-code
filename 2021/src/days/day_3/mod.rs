use crate::days::common::Day;
use std::collections::HashMap;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.lines().map(|l| u32::from_str_radix(l, 2).unwrap())
}

fn find_bit_counts<'a>(vals: &Vec<u32>, size: u8) -> (HashMap<u8, u32>, usize) {
    let mut map: HashMap<u8, u32> = HashMap::new();
    for i in 1..=size {
        map.insert(i, 0);
    }

    let mut s: usize = 0;
    for (e, v) in vals.iter().enumerate() {
        for i in 1..=size {
            if (v >> (i - 1)) & 1 == 1 {
                map.insert(i, map.get(&i).unwrap() + 1);
            }
        }
        s = e;
    }

    (map, s)
}

pub struct Day3 {}

impl Day for Day3 {
    #[allow(unused_must_use)]
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let vals = parse(input).collect();
        let bit_len = 12;
        let (map, size) = find_bit_counts(&vals, bit_len);

        let breaking_val = ((size + 1) / 2) as u32;
        let mut gamma_rate: u16 = 0;
        for i in 1..=12 {
            let v = map.get(&i).unwrap();
            if v >= &breaking_val {
                gamma_rate |= 1 << (i - 1);
            }
        }
        let epsilon_rate = gamma_rate ^ 0b111111111111;
        (gamma_rate as u64 * epsilon_rate as u64).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let vals = parse(input).collect::<Vec<u32>>();

        let bits = 12;
        let oxygen_rating = find_oxygen_rating(vals.clone(), bits);
        let co2_scrubber_rating = find_co2_scrubber_rating(vals.clone(), bits);
        (oxygen_rating * co2_scrubber_rating).to_string()
    }
}

fn find_oxygen_rating(vals: Vec<u32>, bit: u8) -> u32 {
    let (map, size) = find_bit_counts(&vals, bit);
    let should_be_one = map.get(&bit).unwrap() > &((size as u32) / 2);
    let new_vals = vals
        .into_iter()
        .filter(|v| {
            let is_one = ((v.clone() as u16) >> (bit - 1)) & 1 == 1;
            should_be_one == is_one
        })
        .collect::<Vec<u32>>();

    if new_vals.len() == 1 {
        new_vals[0]
    } else if bit == 0 {
        panic!("Failed to find a value");
    } else {
        find_oxygen_rating(new_vals, bit - 1)
    }
}

fn find_co2_scrubber_rating(vals: Vec<u32>, bit: u8) -> u32 {
    let (map, size) = find_bit_counts(&vals, bit);
    let should_be_one = map.get(&bit).unwrap() <= &((size as u32) / 2);
    let new_vals = vals
        .into_iter()
        .filter(|v| {
            let is_one = ((v.clone() as u16) >> (bit - 1)) & 1 == 1;
            should_be_one == is_one
        })
        .collect::<Vec<u32>>();

    if new_vals.len() == 1 {
        new_vals[0]
    } else if bit == 0 {
        panic!("Failed to find a value");
    } else {
        find_co2_scrubber_rating(new_vals, bit - 1)
    }
}
