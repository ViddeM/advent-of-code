pub mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use crate::days::common::Day;

pub fn get_day(day: u8) -> Box<dyn Day> {
    match day {
        1 => Box::new(day_1::Day1 {}),
        2 => Box::new(day_2::Day2 {}),
        3 => Box::new(day_3::Day3 {}),
        4 => Box::new(day_4::Day4 {}),
        5 => Box::new(day_5::Day5 {}),
        6 => Box::new(day_6::Day6 {}),
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
