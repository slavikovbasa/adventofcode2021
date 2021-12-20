use std::{ops::Add, str::{FromStr, Chars}, fmt::Display};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/18/input";


#[derive(Clone)]
enum Number {
    Simple(u32),
    Pair{ left: Box<Number>, right: Box<Number> },
}


impl Number {
    fn add_left(&mut self, n: u32) {
        match self {
            Self::Simple(value) => {
                *value += n;
            },
            Self::Pair{ left, .. } => {
                left.add_left(n);
            }
        }
    }

    fn add_right(&mut self, n: u32) {
        match self {
            Self::Simple(value) => {
                *value += n;
            },
            Self::Pair{ right, .. } => {
                right.add_right(n);
            }
        }
    }

    fn reduce(&mut self) {
        let _ = self.explode(0);

        while self.split() {
            let _ = self.explode(0);
        }
    }

    fn explode(&mut self, depth: u32) -> (u32, u32) {
        match self {
            Self::Simple(_) => (0, 0),
            Self::Pair{ left, right } => {
                if let Self::Simple(l_value) = **left {
                    if let Self::Simple(r_value) = **right {
                        if depth >= 4 {
                            *self = Self::Simple(0);
                            return (l_value, r_value)
                        }
                        return (0, 0)
                    }
                }
                let (lv_left, lv_right) = left.explode(depth + 1);
                right.add_left(lv_right);

                let (rv_left, rv_right) = right.explode(depth + 1);
                left.add_right(rv_left);

                (lv_left, rv_right)
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Simple(n) => {
                if *n > 9 {
                    *self = Self::Pair {
                        left: Box::new(Number::Simple(*n / 2)),
                        right: Box::new(Number::Simple((*n + 1) / 2)),
                    };
                    return true;
                }
                false
            },
            Self::Pair{ left, right } => {
                left.split() || right.split()
            },
        }
    } 

    fn magnitude(&self) -> u32 {
        match self {
            Self::Simple(val) => *val,
            Self::Pair{ left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match &self {
            Self::Simple(_) => panic!("Cant add to simple number"),
            Self::Pair{..} => {
                let mut res = Number::Pair{ left: Box::new(self), right: Box::new(other) };
                res.reduce();
                res
            }
        }
    }
}


impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(n) => write!(f, "{}", n),
            Self::Pair{ left, right } => {
                write!(f, "[{}, {}]", left, right)
            }
        }
    }
}

fn parse_number(s: &mut Chars) -> Result<Number, String> {
    let c = s.next().ok_or("expected a char")?;
    if c == '[' {
        let left = parse_number(s)?;
        assert_eq!(s.next().unwrap(), ',');
        let right = parse_number(s)?;
        assert_eq!(s.next().unwrap(), ']');
        return Ok(Number::Pair{ left: Box::new(left), right: Box::new(right) });
    }
    let num = c.to_digit(10)
        .ok_or("expected a digit")?;
    Ok(Number::Simple(num))
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_number(&mut s.chars())
    }
}


#[allow(dead_code)]
pub fn solve1(text: &str) -> u32 {
    let nums: Vec<Number> = text.lines().map(|l| l.trim().parse().unwrap()).collect();

    nums.into_iter().reduce(|acc, n| acc + n).unwrap().magnitude()
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> u32 {
    let nums: Vec<Number> = text.lines().map(|l| l.trim().parse().unwrap()).collect();
    
    let mut max = 0;
    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            let sum = n1.clone() + n2.clone();
            let magnitude = sum.magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }
    }
    max
}

