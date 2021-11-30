use crate::days::common::Day;

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl Box {
    fn calc_area(&self) -> u32 {
        let sides = [self.l * self.w, self.w * self.h, self.h * self.l];
        let mut smallest_side = u32::MAX;
        let mut area = 0;
        for side in sides.into_iter() {
            if side < smallest_side {
                smallest_side = side;
            }
            area += 2 * side;
        }
        area + smallest_side
    }

    fn calc_ribbon(&self) -> u32 {
        let mut smallest_sides = [self.l, self.w, self.h].into_iter().collect::<Vec<u32>>();
        smallest_sides.sort();
        smallest_sides[0] * 2 + smallest_sides[1] * 2 + (self.l * self.w * self.h)
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Box> + 'a {
    input
        .trim()
        .lines()
        .map(|text| {
            let parts = text.split("x").map(|p| p.parse::<u32>().expect("Failed to parse str to int")).collect::<Vec<u32>>();
            Box {
                l: parts[0],
                w: parts[1],
                h: parts[2],
            }
        })
}

pub struct Day2 {}

impl Day for Day2 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let boxes = parse(input);
        let mut area: u32 = 0;
        for b in boxes {
            area += b.calc_area();
        }

        area.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let boxes = parse(input);
        let mut dist: u32 = 0;
        for b in boxes {
            dist += b.calc_ribbon();
        }
        dist.to_string()
    }
}