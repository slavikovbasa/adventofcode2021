#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/25/input";

#[derive(Debug, Clone)]
enum SeaCucumber {
    Right,
    Down,
}

impl SeaCucumber {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::Right),
            'v' => Some(Self::Down),
            '.' => None,
            _ => panic!("unexpected char"),
        }
    }
}

fn get_inputs(text: &str) -> Vec<Vec<Option<SeaCucumber>>> {
    text.lines()
        .map(|l| l.chars().map(SeaCucumber::from_char).collect())
        .collect()
}

fn proceed(floor: &Vec<Vec<Option<SeaCucumber>>>) -> (bool, Vec<Vec<Option<SeaCucumber>>>) {
    let height = floor.len();
    let width = floor.first().unwrap().len();

    let mut new_floor = floor.clone();
    let mut moved = false;
    for i in 0..height {
        for j in 0..width {
            if let Some(SeaCucumber::Right) = floor[i][j] {
                if let None = floor[i][(j + 1) % width] {
                    moved = true;
                    new_floor[i][j] = None;
                    new_floor[i][(j + 1) % width] = Some(SeaCucumber::Right);
                }
            }
        }
    }
    let floor = new_floor.clone();
    for i in 0..height {
        for j in 0..width {
            if let Some(SeaCucumber::Down) = floor[i][j] {
                if let None = floor[(i + 1) % height][j] {
                    moved = true;
                    new_floor[i][j] = None;
                    new_floor[(i + 1) % height][j] = Some(SeaCucumber::Down);
                }
            }
        }
    }

    (moved, new_floor)
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    let mut floor = get_inputs(text);

    let mut count = 0;
    let mut moved = true;
    while moved {
        let (new_moved, new_floor) = proceed(&floor);
        moved = new_moved;
        floor = new_floor;
        count += 1;
    }

    count
}

#[allow(dead_code)]
pub fn solve2(_text: &str) -> u64 {
    0
}
