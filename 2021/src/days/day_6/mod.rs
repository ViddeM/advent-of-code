use crate::days::common::Day;

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
            let zero_index = day % NUMBERS;
            let six_index = (zero_index + 7) % NUMBERS;
            fishes_arr[six_index] = fishes_arr[six_index] + fishes_arr[zero_index];
        }

        fishes_arr.into_iter().fold(0, |t, v| t + v).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut fishes_arr = parse(input);

        for day in 1..256 {
            let zero_index = day % NUMBERS;
            let six_index = (zero_index + 7) % NUMBERS;
            fishes_arr[six_index] = fishes_arr[six_index] + fishes_arr[zero_index];
        }

        fishes_arr.into_iter().fold(0, |t, v| t + v).to_string()
    }
}

#[allow(dead_code)]
fn print_arr(zero_index: usize, arr: [u64; NUMBERS]) -> String {
    let mut a = String::new();
    for i in 0..NUMBERS {
        let f = arr[(zero_index + i) % NUMBERS];
        a.push_str(&format!("{}: {: <3}  | ", i, f))
    }
    a
}
