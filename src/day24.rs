// Reverse-engineered algorithm by hand

use std::str::FromStr;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/24/input";

#[derive(Debug, Clone)]
enum Arg {
    Var(String),
    Val(i64),
}

impl FromStr for Arg {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.parse::<i64>() {
            Ok(v) => Ok(Arg::Val(v)),
            _ => Ok(Arg::Var(String::from(s))),
        };
    }
}

#[derive(Debug)]
enum Op {
    Inp(Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Div(Arg, Arg),
    Mod(Arg, Arg),
    Eql(Arg, Arg),
}

impl Op {
    fn get_arg2(&self) -> Arg {
        match self {
            Op::Inp(_) => panic!("unexpected to call get_arg2 on inp op"),
            Op::Add(_, arg2)
            | Op::Mul(_, arg2)
            | Op::Div(_, arg2)
            | Op::Mod(_, arg2)
            | Op::Eql(_, arg2) => arg2.clone(),
        }
    }
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next().ok_or("empty instruction")? {
            "inp" => {
                let arg = parts.next().ok_or("no args for inp")?;
                Ok(Op::Inp(Arg::Var(String::from(arg))))
            }
            op @ ("add" | "mul" | "div" | "mod" | "eql") => {
                let arg1 = parts.next().ok_or("no args")?.parse()?;
                let arg2 = parts.next().ok_or("no args")?.parse()?;
                if let Arg::Val(_) = arg1 {
                    return Err("can't write into value");
                }

                match op {
                    "add" => Ok(Op::Add(arg1, arg2)),
                    "mul" => Ok(Op::Mul(arg1, arg2)),
                    "div" => Ok(Op::Div(arg1, arg2)),
                    "mod" => Ok(Op::Mod(arg1, arg2)),
                    "eql" => Ok(Op::Eql(arg1, arg2)),
                    _ => unreachable!(),
                }
            }
            _ => Err("unknown instruction"),
        }
    }
}

fn get_inputs(text: &str) -> Vec<Vec<Op>> {
    text.split("inp w")
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .map(|l| l.parse().unwrap())
                .collect()
        })
        .collect()
}

fn params_from_inputs(inputs: Vec<Vec<Op>>) -> Vec<(i64, i64)> {
    let mut params = vec![];
    for block in inputs {
        match block[4].get_arg2() {
            Arg::Val(v1) => match block[14].get_arg2() {
                Arg::Val(v2) => {
                    params.push((v1, v2));
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    params
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    let stages = get_inputs(text);
    assert_eq!(stages.len(), 14);

    let mut chars = ['0'; 14];
    let mut stack = vec![];

    let mut i = 0;
    for (v1, v2) in params_from_inputs(stages) {
        if v1 > 0 {
            stack.push((i, v2));
        } else {
            let (prev_i, prev_v2) = stack.pop().unwrap();
            let delta = prev_v2 + v1;
            let (n_i, prev_n_i) = if delta > 0 {
                (9, 9 - delta)
            } else {
                (9 + delta, 9)
            };
            assert!(n_i > 0 && n_i < 10 && prev_n_i > 0 && prev_n_i < 10);
            chars[i] = ('0' as u8 + n_i as u8) as char;
            chars[prev_i] = ('0' as u8 + prev_n_i as u8) as char;
        }
        i += 1;
    }

    String::from_iter(chars.into_iter()).parse().unwrap()
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let stages = get_inputs(text);
    assert_eq!(stages.len(), 14);

    let mut chars = ['0'; 14];
    let mut stack = vec![];

    let mut i = 0;
    for (v1, v2) in params_from_inputs(stages) {
        if v1 > 0 {
            stack.push((i, v2));
        } else {
            let (prev_i, prev_v2) = stack.pop().unwrap();
            let delta = prev_v2 + v1;
            let (n_i, prev_n_i) = if delta > 0 {
                (1 + delta, 1)
            } else {
                (1, 1 - delta)
            };
            assert!(n_i > 0 && n_i < 10 && prev_n_i > 0 && prev_n_i < 10);
            chars[i] = ('0' as u8 + n_i as u8) as char;
            chars[prev_i] = ('0' as u8 + prev_n_i as u8) as char;
        }
        i += 1;
    }

    String::from_iter(chars.into_iter()).parse().unwrap()
}

// Bruteforce

// use std::{collections::HashMap, str::FromStr, ops::RangeInclusive, time::Instant};

// #[allow(dead_code)]
// pub const URL: &str = "https://adventofcode.com/2021/day/24/input";

// #[derive(Debug)]
// struct State {
//     vars: HashMap<String, i64>,
//     input: [i64; 14],
//     curr_input_idx: usize,
// }

// impl State {
//     fn new(input: [i64; 14]) -> Self {
//         let mut vars = HashMap::new();
//         vars.insert("w".to_string(), 0);
//         vars.insert("x".to_string(), 0);
//         vars.insert("y".to_string(), 0);
//         vars.insert("z".to_string(), 0);

//         State {
//             vars,
//             input,
//             curr_input_idx: 0,
//         }
//     }

//     fn input(&mut self, var: String) {
//         if self.curr_input_idx >= self.input.len() {
//             panic!("trying to read out of bounds")
//         }
//         self.vars.insert(var, self.input[self.curr_input_idx]);
//         self.curr_input_idx += 1;
//     }

//     fn get(&self, var: String) -> &i64 {
//         self.vars.get(&var).unwrap_or(&0)
//     }

//     fn apply(&mut self, var1: String, var2: String, func: fn(i64, i64) -> i64) {
//         let n2 = *self.get(var2);
//         let n1 = self.vars.entry(var1).or_default();
//         *n1 = func(*n1, n2);
//     }

//     fn apply_val(&mut self, var: String, val: i64, func: fn(i64, i64) -> i64) {
//         let n1 = self.vars.entry(var).or_default();
//         *n1 = func(*n1, val);
//     }
// }

// #[derive(Debug)]
// enum Arg {
//     Var(String),
//     Val(i64),
// }

// impl FromStr for Arg {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         return match s.parse::<i64>() {
//             Ok(v) => Ok(Arg::Val(v)),
//             _ => Ok(Arg::Var(String::from(s))),
//         };
//     }
// }

// #[derive(Debug)]
// enum Op {
//     Inp(Arg),
//     Add(Arg, Arg),
//     Mul(Arg, Arg),
//     Div(Arg, Arg),
//     Mod(Arg, Arg),
//     Eql(Arg, Arg),
// }

// impl Op {
//     fn func(&self) -> fn(i64, i64) -> i64 {
//         match self {
//             Op::Inp(_) => panic!("no func associated with <inp> instruction"),
//             Op::Add(_, _) => |n1, n2| n1 + n2,
//             Op::Mul(_, _) => |n1, n2| n1 * n2,
//             Op::Div(_, _) => |n1, n2| {
//                 if n2 == 0 {
//                     panic!("cant divide by 0");
//                 }
//                 n1 / n2
//             },
//             Op::Mod(_, _) => |n1, n2| {
//                 if n1 < 0 || n2 <= 0 {
//                     panic!("cant mod with nagative numbers");
//                 }
//                 n1 % n2
//             },
//             Op::Eql(_, _) => |n1, n2| if n1 == n2 { 1 } else { 0 },
//         }
//     }

//     fn eval(&self, s: &mut State) {
//         match self {
//             Op::Inp(arg) => {
//                 match arg {
//                     Arg::Val(_) => panic!("cant write into value"),
//                     Arg::Var(v) => { s.input(v.to_string()); },
//                 };
//             }
//             op @ (
//                 Op::Add(arg1, arg2)
//                 | Op::Mul(arg1, arg2)
//                 | Op::Div(arg1, arg2)
//                 | Op::Mod(arg1, arg2)
//                 | Op::Eql(arg1, arg2)
//             ) => {
//                 match arg1 {
//                     Arg::Val(_) => panic!("cant write into value"),
//                     Arg::Var(v1) => match arg2 {
//                         Arg::Val(n2) => s.apply_val(v1.to_string(), *n2, op.func()),
//                         Arg::Var(v2) => s.apply(v1.to_string(), v2.to_string(), op.func()),
//                     }
//                 };
//             }
//         }
//     }
// }

// impl FromStr for Op {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut parts = s.split_whitespace();
//         match parts.next().ok_or("empty instruction")? {
//             "inp" => {
//                 let arg = parts.next().ok_or("no args for inp")?;
//                 Ok(Op::Inp(Arg::Var(String::from(arg))))
//             },
//             op @ ("add" | "mul" | "div" | "mod" | "eql") => {
//                 let arg1 = parts.next().ok_or("no args")?.parse()?;
//                 let arg2 = parts.next().ok_or("no args")?.parse()?;
//                 if let Arg::Val(_) = arg1 {
//                     return Err("can't write into value");
//                 }

//                 match op {
//                     "add" => Ok(Op::Add(arg1, arg2)),
//                     "mul" => Ok(Op::Mul(arg1, arg2)),
//                     "div" => Ok(Op::Div(arg1, arg2)),
//                     "mod" => Ok(Op::Mod(arg1, arg2)),
//                     "eql" => Ok(Op::Eql(arg1, arg2)),
//                     _ => unreachable!(),
//                 }
//             }
//             o => {
//                 println!("{}", o);
//                 Err("unknown instruction")
//             },
//         }
//     }
// }

// fn get_inputs(text: &str) -> Vec<Op> {
//     text.lines().map(|l| l.parse().unwrap()).collect()
// }

// fn next_input_num(
//     digit_gens: &mut Vec<RangeInclusive<i64>>,
//     mut num: [i64; 14],
// ) -> Option<[i64; 14]> {
//     for i in 0..14 {
//         let cur_iter = &mut digit_gens[i];
//         match cur_iter.next() {
//             None => {
//                 if i == 13 {
//                     return None
//                 }
//                 digit_gens[i] = 2..=9;
//                 num[i] = 1;
//                 continue;
//             }
//             Some(v) => {
//                 num[i] = v;
//                 break;
//             }
//         }
//     }
//     Some(num)
// }

// #[allow(dead_code)]
// pub fn solve1(text: &str) -> u64 {
//     let ops = get_inputs(text);
//     let mut digit_gens: Vec<RangeInclusive<i64>> = (0..13).map(|_| (2..=9)).collect();
//     digit_gens.insert(0, 1..=9);
//     let mut prev_num = [1; 14];
//     prev_num[0] = 0;

//     let mut c = 0 as u64;
//     let mut good_nums = Vec::new();
//     let now = Instant::now();
//     while let Some(num) = next_input_num(&mut digit_gens, prev_num) {
//         if c % 100000 == 0 {
//             println!("{} {}", c, now.elapsed().as_secs());
//         }
//         let mut state = State::new(num);
//         ops.iter().for_each(|op| {
//             op.eval(&mut state);
//         });
//         if *state.get("z".to_string()) == 0 {
//             good_nums.push(num);
//             println!("{:?}", num)
//         }
//         prev_num = num;
//         c += 1;
//     }
//     0
// }
