use std::collections::HashMap;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/8/input";

#[derive(Default)]
struct Segments {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
}

const A_BYTE: u8 = 0x61;

fn str_to_num(s: &str) -> u8 {
    s.bytes().map(|b| b - A_BYTE).map(|i| 1 << i).sum()
}

fn is_one_segment(n: u8) -> bool {
    format!("{:b}", n).chars().filter(|&c| c == '1').count() == 1
}

fn solve_for_line(inputs: Vec<&str>, outputs: Vec<&str>) -> usize {
    let mut len6 = Vec::with_capacity(3);

    let mut s: Segments = Default::default();
    let mut digits: [u8; 10] = [0; 10];

    for n in inputs {
        match n.len() {
            2 => {
                digits[1] = str_to_num(n);
            }
            3 => {
                digits[7] = str_to_num(n);
            }
            4 => {
                digits[4] = str_to_num(n);
            }
            5 => (),
            6 => {
                len6.push(n);
            }
            7 => {
                digits[8] = str_to_num(n);
            }
            _ => panic!("bad number of segments"),
        };
    }

    s.a = digits[1] ^ digits[7];
    for digits_0_6_9 in len6.iter().map(|&seq| str_to_num(seq)) {
        let g = digits_0_6_9 ^ s.a ^ digits[4];
        if is_one_segment(g) {
            s.g = g;
            digits[9] = digits_0_6_9;
            continue;
        }

        let b = digits_0_6_9 ^ digits[1] ^ digits[4] ^ digits[8];
        if is_one_segment(b) {
            s.b = b;
            digits[0] = digits_0_6_9;
            continue;
        }
        digits[6] = digits_0_6_9;
    }

    assert_ne!(digits[0], 0);
    assert_ne!(digits[6], 0);
    assert_ne!(digits[9], 0);
    assert_ne!(s.b, 0);
    assert_ne!(s.g, 0);

    digits[3] = digits[9] ^ s.b;
    s.c = digits[6] ^ digits[8];
    s.d = digits[0] ^ digits[8];
    s.e = digits[9] ^ digits[8];
    s.f = digits[1] ^ s.c;

    digits[2] = s.a ^ s.c ^ s.d ^ s.e ^ s.g;
    digits[5] = s.a ^ s.b ^ s.d ^ s.f ^ s.g;

    let num_to_digit: HashMap<_, _> = digits
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();

    outputs
        .iter()
        .map(|&s| str_to_num(s))
        .map(|num| num_to_digit.get(&num).unwrap())
        .rev()
        .enumerate()
        .map(|(i, v)| v * (10 as usize).pow(i as u32))
        .sum()
}

fn get_uniq(text: &str) -> usize {
    text.lines()
        .map(|l| {
            l.split('|')
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .filter(|w| w.len() == 2 || w.len() == 3 || w.len() == 4 || w.len() == 7)
                .count()
        })
        .sum()
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    get_uniq(&text)
}

fn line_to_tuple<'a>(line: &str) -> (Vec<&str>, Vec<&str>) {
    let mut line = line.split('|');
    let inputs = line.next().unwrap().trim().split_whitespace().collect();
    let outputs = line.next().unwrap().trim().split_whitespace().collect();

    (inputs, outputs)
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    text.lines()
        .map(line_to_tuple)
        .map(|(inputs, outputs)| solve_for_line(inputs, outputs))
        .sum()
}
