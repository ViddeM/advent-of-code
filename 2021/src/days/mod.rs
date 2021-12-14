pub mod common;
mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::days::common::Day;

pub fn get_day(day: u8) -> Box<dyn Day> {
    match day {
        1 => Box::new(day_1::Day1 {}),
        2 => Box::new(day_2::Day2 {}),
        3 => Box::new(day_3::Day3 {}),
        4 => Box::new(day_4::Day4 {}),
        5 => Box::new(day_5::Day5 {}),
        6 => Box::new(day_6::Day6 {}),
        7 => Box::new(day_7::Day7 {}),
        8 => Box::new(day_8::Day8 {}),
        9 => Box::new(day_9::Day9 {}),
        10 => Box::new(day_10::Day10 {}),
        11 => Box::new(day_11::Day11 {}),
        12 => Box::new(day_12::Day12 {}),
        13 => Box::new(day_13::Day13 {}),
        14 => Box::new(day_14::Day14 {}),
        day => {
            println!("Day not implemented: {}", day);
            panic!("Exiting")
        }
    }
}

pub fn run(day: u8, input: &str) {
    let day_solver = get_day(day);

    println!("Running day {:02}...", day);
    let part_1 = handle_solution_result(day_solver.part_1(input));
    println!("Part 1:: {}", part_1);
    let part_2 = handle_solution_result(day_solver.part_2(input));
    println!("Part 2:: {}", part_2);
}

fn handle_solution_result(result: String) -> String {
    return if result.is_empty() {
        "unsolved".to_string()
    } else {
        result
    };
}
