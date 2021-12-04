use std::collections::HashSet;

pub const URL: &str = "https://adventofcode.com/2021/day/4/input";

const BOARD_SIZE: usize = 5;

struct Board {
    rows: Vec<HashSet<u32>>,
    cols: Vec<HashSet<u32>>,
    is_winner: bool,
}

impl Board {
    fn from_lines<'a, T>(lines: &mut T) -> Board
    where
        T: Iterator<Item=&'a str>
    {
        let mut rows = vec![HashSet::new(); BOARD_SIZE];
        let mut cols = vec![HashSet::new(); BOARD_SIZE];
        lines.take(BOARD_SIZE).map(
            |l| l.trim().split_whitespace().map(|s| s.parse().unwrap())
        ).enumerate().for_each(
            |(i, l)| l.enumerate().for_each(
                |(j, num)| {
                    rows[i].insert(num);
                    cols[j].insert(num);
                }
            )
        );
        Board { rows, cols, is_winner: false }
    }

    fn cross_out(&mut self, num: u32) -> bool {
        for row in self.rows.iter_mut() {
            row.remove(&num);
            if row.is_empty() {
                self.is_winner = true;
                return true;
            }
        }
        for col in self.cols.iter_mut() {
            col.remove(&num);
            if col.is_empty() {
                self.is_winner = true;
                return true;
            }
        }

        false
    }
    
    fn sum(&self) -> u32 {
        self.rows.iter().flatten().sum()
    }
}


fn get_inputs(text: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = text.lines();
    let drawn_nums: Vec<u32> = lines.next().unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        boards.push(Board::from_lines(&mut lines))
    }
    return (drawn_nums, boards);
}

fn play_game1(drawn_nums: Vec<u32>, boards: &mut Vec<Board>) -> Option<(u32, usize)> {
    for num in drawn_nums {
        for (i, board) in boards.iter_mut().enumerate() {
            let is_winner = board.cross_out(num);
            if is_winner {
                return Some((num, i));
            }
        }
    }
    return None;
}

pub fn solve1(text: &str) -> u32 {
    let (drawn_nums, mut boards) = get_inputs(text);
    let (winner_num, winner_idx) = play_game1(drawn_nums, &mut boards).unwrap();
    let winner_board = &boards[winner_idx];

    winner_num * winner_board.sum()
}


fn play_game2(drawn_nums: Vec<u32>, boards: &mut Vec<Board>) -> (u32, usize) {
    let boards_len = boards.len();
    let mut winners = Vec::new();
    for num in drawn_nums {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.is_winner {
                continue;
            }
            if board.cross_out(num) {
                winners.push((num, i));
                if winners.len() == boards_len {
                    return *winners.last().unwrap();
                }
            }
        }
    }
    return *winners.last().unwrap();
}

pub fn solve2(text: &str) -> u32 {
    let (drawn_nums, mut boards) = get_inputs(text);
    let (winner_num, winner_idx) = play_game2(drawn_nums, &mut boards);
    let winner_board = &boards[winner_idx];

    winner_num * winner_board.sum()
}
