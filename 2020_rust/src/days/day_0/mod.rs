
pub fn parse(input_name: &str) -> () {
    todo!(fmt!("Implement parsing of input {}", input_name))
}

pub fn part_1(input_name: &str) -> Option<()> {
    let data = parse(input_name);
    todo!("Implement part 1")
}

pub fn part_2(input_name: &str) -> Option<()> {
    let data = parse(input_name);
    todo!("Implement part 2")
}

pub fn run(input: AOCInput) -> () {
    let data = match input {
        AOCInput::Input(input) => input,
        AOCInput::Test => include_str!("test_input.txt"),
    };

    let part_one = part_1(data);
    println!("Solution to part 1: {:?}", part_one);
    let part_two = part_2(data);
    println!("Solution to part 2: {:?}", part_two);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_parse(b: &mut Bencher) {
        let data = include_str!("input.txt");
        b.iter(|| parse(data))
    }

    #[bench]
    pub fn bench_part_1(b: &mut Bencher) {
        let data = include_str!("input.txt");
        b.iter(|| part_1(data))
    }

    #[bench]
    pub fn bench_part_2(b: &mut Bencher) {
        let data = include_str!("input.txt");
        b.iter(|| part_2(data))
    }
}