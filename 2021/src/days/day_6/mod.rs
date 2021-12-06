use std::collections::HashMap;
use crate::days::common::Day;
use std::fmt::Alignment;

const NUMBERS: usize = 9;

fn parse<'a>(input: &'a str) -> [u64; NUMBERS] {
    let mut arr = [0u64; NUMBERS];
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .for_each(|v| arr[v as usize] = arr[v as usize] + 1);
    arr
}

pub struct Day6 {}

impl Day for Day6 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let mut fishes_arr = parse(input);

        for day in 1..80 {
            let zero_index = (day % NUMBERS);
            let six_index = (zero_index + 7) % NUMBERS;
            fishes_arr[six_index] = fishes_arr[six_index] + fishes_arr[zero_index];
        }

        fishes_arr.into_iter().fold(0, |t, v| t + v).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        // let mut fishes = parse(input);
        //
        // let mut fish_map = HashMap::new();
        // fishes.for_each(|fish| {
        //    match fish_map.get(&fish) {
        //        Some(f) => fish_map.insert(fish, f + 1),
        //        None => fish_map.insert(fish, 1),
        //    };
        // });
        //
        // for day in 1..=256 {
        //     let mut new_fish_map: HashMap<u32, u64> = HashMap::new();
        //     for key in fish_map.keys() {
        //         if *key == 0 {
        //             match new_fish_map.get(&8) {
        //                 Some(v) => new_fish_map.insert(8, v + fish_map[key]),
        //                 None => new_fish_map.insert(8, fish_map[key]),
        //             };
        //             match new_fish_map.get(&6) {
        //                 Some(v) => new_fish_map.insert(6, v + fish_map[key]),
        //                 None => new_fish_map.insert(6, fish_map[key]),
        //             };
        //         } else {
        //             match new_fish_map.get(&(key - 1)) {
        //                 None => {new_fish_map.insert(key - 1, fish_map[key])}
        //                 Some(v) => {new_fish_map.insert(key - 1, v + fish_map[key])}
        //             };
        //         }
        //     }
        //
        //     fish_map = new_fish_map;
        // }
        //
        // fish_map.iter().fold(0, |p, (_, v)| p + v).to_string()
        "".to_string()
    }
}

fn print_arr(zero_index: usize, arr: [u64; NUMBERS]) -> String {
    let mut a = String::new();
    for i in 0..NUMBERS {
        let f = arr[(zero_index + i) % NUMBERS];
        a.push_str(&format!("{}: {: <3}  | ", i, f))
    }
    a
}