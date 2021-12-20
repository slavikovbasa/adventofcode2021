#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/10/input";


const OPENING_BRACES: &str = "([{<";

fn get_closing_bracket(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected symbol")
    }
}

fn bracket_value(closing: &char) -> u64 {
    match closing {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unexpected symbol")
    }
}

fn first_corrupted(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if OPENING_BRACES.contains(c) {
            stack.push(c)
        } else {
            let v = stack.pop().unwrap();
            if get_closing_bracket(v) != c {
                return Some(c);
            }
        }
    }
    return None
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u32 {
    text.lines().map(|l| first_corrupted(l)).map(|v| {
        match v {
            None => 0,
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => panic!("unexpected symbol")
        }
    }).sum()
}


fn closing_seq(line: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if OPENING_BRACES.contains(c) {
            stack.push(c)
        } else {
            let v = stack.pop().unwrap();
            if get_closing_bracket(v) != c {
                return None;
            }
        }
    }
    let seq = stack.into_iter().rev().map(|c| get_closing_bracket(c)).collect();
    Some(seq)
}

fn seq_score(seq: Vec<char>) -> u64 {
    seq.iter().fold(0, |acc, c| acc * 5 + bracket_value(c))
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let mut scores: Vec<u64> = text.lines()
        .map(closing_seq)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(seq_score)
        .collect();
    
    scores.sort_unstable();
    scores[scores.len() / 2]
}

