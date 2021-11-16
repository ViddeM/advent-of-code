
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
    if let Some(val) = part_one {
        println!("Solution to part 1: {}", val);
    } else {
        println!("Found no solution to part 1 :(");
    }
    let part_two = part_2(data);
    if let Some(val) = part_two {
        println!("Solution to part 2: {}", val);
    } else {
        println!("Found no solution to part 2 :(");
    }
}