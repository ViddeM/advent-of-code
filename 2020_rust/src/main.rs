#![feature(test)]

mod config;
pub mod days;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use config::Config;

const DEFAULT_DAY: u8 = 2;
const YEAR: u16 = 2020;

fn main() {
    let config = Config::new();

    let args: Vec<String> = env::args().collect();
    let day_arg = if args.len() > 1 {
        args[1].parse::<u8>().expect("First argument must be a valid u16 representing the day to run")
    } else {
        DEFAULT_DAY
    };

    let test_arg = args.len().eq(&(3 as usize)) && args[2].eq("test");
    let input = get_input(day_arg, &config).expect("Failed to get input");
    days::run(day_arg, if test_arg {
        get_test_input(day_arg, &config)
    } else {
        input
    }.as_str())
}

fn download_input(day: u8, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/{}/day/{}/input", config.url, YEAR, day);
    let resp = client
        .get(&url)
        .header("cookie", format!("session={}", config.session))
        .send()?;
    Ok(resp.text()?)
}

fn get_input(day: u8, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let file_path = format!("{}/src/days/day_{}/input.txt", config.base_download_path, day);
    let downloaded = Path::new(&file_path);
    if downloaded.exists() {
        let mut data = String::new();
        let mut file = File::open(&downloaded)?;
        file.read_to_string(&mut data)?;
        Ok(data)
    } else {
        let data = download_input(day, config)?;
        // Save the file
        let mut file = File::create(&downloaded)?;
        file.write_all(data.as_bytes())?;
        Ok(data)
    }
}

fn get_test_input(day: u8, config: &Config) -> String {
    let file_path = format!("{}/src/days/day_{}/test_input.txt", config.base_download_path, day);
    let path = Path::new(&file_path);
    if path.exists() == false {
        panic!("No test file exists for the given day!");
    }

    let mut file = File::open(&path).unwrap();
    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();
    return data;
}

#[cfg(test)]
mod test {
    extern crate test;

    use std::env;
    use test::Bencher;
    use crate::config::Config;
    use crate::days::common::Day;
    use crate::days::get_day;
    use crate::get_input;

    fn load_bench_data() -> (Box<dyn Day>, String) {
        let config = Config::new();
        let day = env::var("DAY")
            .expect("Failed to read 'DAY' from env")
            .parse::<u8>()
            .expect("'DAY' var must be a valid u8");

        let data = get_input(day, &config).unwrap();

        (get_day(day), data)
    }

    #[bench]
    pub fn bench_parse(b: &mut Bencher) {
        let (day, data) = load_bench_data();
        b.iter(|| day.parse_bench(&data))
    }

    #[bench]
    pub fn bench_part_1(b: &mut Bencher) {
        let (day, data) = load_bench_data();
        b.iter(|| day.part_1(&data))
    }

    #[bench]
    pub fn bench_part_2(b: &mut Bencher) {
        let (day, data) = load_bench_data();
        b.iter(|| day.part_2(&data))
    }
}