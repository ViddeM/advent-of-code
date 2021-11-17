pub mod common;
mod day_1;
mod day_2;
mod day_3;

use crate::days::common::AOCInput;

pub fn run(day: u8, input: AOCInput) {
    match day {
        1 => day_1::run(input),
        2 => day_2::run(input),
        3 => day_3::run(input),
        day => println!("Invalid day received: {}", day)
    };
}