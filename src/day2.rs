#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/2/input";

enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
    None(String),
}

fn get_move_vec(text: &str) -> Vec<Move> {
    text.lines()
        .map(|x| {
            let mut split = x.trim().split_whitespace();
            let tuple = (
                split.next().unwrap(),
                split.next().unwrap().parse().unwrap(),
            );
            match tuple {
                ("forward", n) => Move::Forward(n),
                ("down", n) => Move::Down(n),
                ("up", n) => Move::Up(n),
                (s, _) => Move::None(s.to_string()),
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> i32 {
    let move_vec = get_move_vec(text);

    let mut horizontal = 0;
    let mut depth = 0;
    for mv in move_vec {
        match mv {
            Move::Forward(n) => horizontal += n,
            Move::Down(n) => depth += n,
            Move::Up(n) => depth -= n,
            Move::None(s) => println!("invalid move {}", &s),
        }
    }

    return horizontal * depth;
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> i64 {
    let move_vec = get_move_vec(text);

    let mut aim: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    for mv in move_vec {
        match mv {
            Move::Forward(n) => {
                horizontal += n as i64;
                depth += aim * n as i64;
            }
            Move::Down(n) => aim += n as i64,
            Move::Up(n) => aim -= n as i64,
            Move::None(s) => println!("invalid move {}", &s),
        }
    }

    return horizontal * depth;
}
