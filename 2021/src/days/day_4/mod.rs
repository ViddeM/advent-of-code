use crate::days::common::Day;

type Board = Vec<Vec<(u32, bool)>>;

fn parse(input: &str) -> (Vec<Board>, Vec<u32>) {
    let mut lines = input.lines();

    let nums = lines
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut board: Board = Vec::new();
    let mut boards: Vec<Board> = vec![];

    for l in lines {
        if l.is_empty() {
            if board.is_empty() == false {
                boards.push(board);
                board = Vec::new();
            }
        } else {
            board.push(
                l.split(" ")
                    .filter(|v| v.is_empty() == false)
                    .map(|v| (v.parse::<u32>().unwrap(), false))
                    .collect::<Vec<(u32, bool)>>(),
            );
        }
    }
    if board.is_empty() == false {
        boards.push(board);
    }

    (boards, nums)
}

pub struct Day4 {}

impl Day for Day4 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let (mut boards, nums) = parse(input);
        for num in nums {
            let mut new_boards = Vec::new();
            for mut board in boards.clone().into_iter() {
                let res = set_true(&mut board, num);
                if let Some(v) = res {
                    return (v * num).to_string();
                }
                new_boards.push(board);
            }
            boards = new_boards;
        }
        "failed_to_find_solution".to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (mut boards, nums) = parse(input);
        for num in nums {
            let mut new_boards = Vec::new();
            for mut board in boards.clone().into_iter() {
                let res = set_true(&mut board, num);
                match res {
                    None => new_boards.push(board),
                    Some(v) => {
                        if boards.len() == 1 {
                            return (v * num).to_string();
                        }
                    }
                }
            }
            boards = new_boards;
        }
        "No solution".to_string()
    }
}

fn set_true(board: &mut Board, num: u32) -> Option<u32> {
    for (r_i, row) in board.iter().enumerate() {
        for (c_i, (v, set)) in row.iter().enumerate() {
            if !set && v.eq(&num) {
                board[r_i][c_i] = (v.clone(), true);
                return check_finished(board);
            }
        }
    }

    None
}

fn check_finished(board: &mut Board) -> Option<u32> {
    if check_rows_finished(board) || check_cols_finished(board) {
        let mut sum = 0;
        for row in board {
            for (n, b) in row {
                if *b == false {
                    sum += *n;
                }
            }
        }
        return Some(sum);
    }
    None
}

fn check_rows_finished(board: &mut Board) -> bool {
    for row in board {
        if row.iter().fold(true, |b, (_, new_b)| {
            if b && new_b == &true {
                return true;
            }
            return false;
        }) {
            return true;
        }
    }

    false
}

fn check_cols_finished(board: &mut Board) -> bool {
    let mut cols_finished = Vec::new();
    for i in 0..5 {
        cols_finished.insert(i, true);
    }

    for row in board {
        for (i_c, (_, set)) in row.iter().enumerate() {
            if !set {
                cols_finished[i_c] = false;
            }
        }
    }
    return cols_finished.contains(&true);
}

#[allow(dead_code)]
fn print_board(board: &Board) -> () {
    for row in board {
        println!("{:?}", row)
    }
    println!()
}
