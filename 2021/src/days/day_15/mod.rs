use crate::days::common::Day;
use priority_queue::priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct World {
    map: Vec<u64>,
    width: u64,
    height: u64,
}

#[derive(Clone, Debug, Hash)]
struct Pos {
    x: u64,
    y: u64,
}

impl Pos {
    fn to_index(&self, world: &World) -> usize {
        (self.x + self.y * world.width) as usize
    }

    fn from_index(index: usize, world: &World) -> Pos {
        let x = index as u64 % world.width;
        let y = (index as u64 - x) / world.width;
        Pos { x, y }
    }
}

impl PartialEq<Self> for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Eq for Pos {}

fn parse(input: &str) -> World {
    let mut width = 0;
    let map = input
        .lines()
        .map(|l| {
            width = l.len() as u64;
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .fold(Vec::new(), |mut m, mut digits| {
            m.append(&mut digits);
            m
        });

    let height = (map.len() as u64 / width);

    World { map, width, height }
}

pub struct Day15 {}

impl Day for Day15 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let world = parse(input);
        let start = Pos { x: 0, y: 0 };
        let end = Pos {
            x: world.width - 1,
            y: world.height - 1,
        };

        let path = dijsktras(world.clone(), start.clone(), end.clone());
        let mut sum = 0;
        let mut curr = end;
        loop {
            sum += world.map[curr.to_index(&world)];
            curr = path.get(&curr).unwrap().clone();
            if curr.eq(&start) {
                break;
            }
        }

        sum.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let world = get_new_world(parse(input));

        // print_world(&world);

        let start = Pos { x: 0, y: 0 };
        let end = Pos {
            x: world.width - 1,
            y: world.height - 1,
        };
        let path = dijsktras(world.clone(), start.clone(), end.clone());

        let mut sum = 0;
        let mut curr = end;
        loop {
            sum += world.map[curr.to_index(&world)];
            curr = path.get(&curr).unwrap().clone();
            if curr.eq(&start) {
                break;
            }
        }

        sum.to_string()
    }
}

fn dijsktras(world: World, start: Pos, target: Pos) -> HashMap<Pos, Pos> {
    let mut dist: HashMap<Pos, u64> = HashMap::new();
    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    let mut q: PriorityQueue<Pos, u64> = PriorityQueue::new();

    dist.insert(start, 0);
    for (i, _) in world.map.clone().into_iter().enumerate() {
        let p = Pos::from_index(i, &world);
        if dist.contains_key(&p) == false {
            dist.insert(p.clone(), u32::MAX as u64);
        }
        q.push(p.clone(), u32::MAX as u64 - dist.get(&p).unwrap());
    }

    while q.is_empty() == false {
        let (u, _) = q.pop().unwrap();

        if u.eq(&target) {
            return prev;
        }

        let curr_dist = *dist.get(&u).unwrap();
        for neighbour in get_neighbours(u.clone(), &world) {
            let alt = curr_dist + world.map[neighbour.to_index(&world)];
            let alt_dist = *dist.get(&neighbour).unwrap();
            if alt < alt_dist {
                dist.insert(neighbour.clone(), alt);
                prev.insert(neighbour.clone(), u.clone());
                q.change_priority(&neighbour, (u32::MAX as u64) - alt);
            }
        }
    }

    prev
}

fn get_neighbours(pos: Pos, world: &World) -> Vec<Pos> {
    let mut neighbours = Vec::new();
    if pos.y > 0 {
        neighbours.push(Pos {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.x < world.width - 1 {
        neighbours.push(Pos {
            x: pos.x + 1,
            y: pos.y,
        })
    }
    if pos.y < world.height - 1 {
        neighbours.push(Pos {
            x: pos.x,
            y: pos.y + 1,
        })
    }
    if pos.x > 0 {
        neighbours.push(Pos {
            x: pos.x - 1,
            y: pos.y,
        })
    }

    neighbours
}

fn get_new_world(world: World) -> World {
    let new_width = world.width * 5;
    let new_height = world.height * 5;
    let mut new_map = Vec::new();

    for rep_y in 0..5 {
        for y in 0..world.height {
            for rep_x in 0..5 {
                for x in 0..world.width {
                    let orig_val = world.map[(x + y * world.width) as usize];
                    let new_val = ((orig_val - 1) + rep_x + rep_y) % 9 + 1;
                    new_map.push(new_val)
                }
            }
        }
    }

    World {
        width: new_width,
        height: new_height,
        map: new_map,
    }
}

fn print_path(path: HashMap<Pos, Pos>, last: &Pos) {
    let mut c = path.get(last);
    while let Some(pos) = c {
        println!("{:?}", pos);
        c = path.get(pos);
    }
}

fn print_world(world: &World) {
    for y in 0..world.height {
        let mut s = String::new();
        for x in 0..world.width {
            s.push_str(&world.map[(x + y * world.width) as usize].to_string());
        }
        println!("{}", s);
    }
}
