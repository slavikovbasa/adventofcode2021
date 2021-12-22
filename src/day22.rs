use std::{cmp, num::ParseIntError, str::FromStr};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/22/input";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Range(i32, i32);

impl Range {
    fn is_valid(&self) -> bool {
        self.1 > self.0
    }

    fn clamp(&self, limits: (i32, i32)) -> Self {
        Range(cmp::max(self.0, limits.0), cmp::min(self.1, limits.1))
    }

    fn intersection(&self, r: &Range) -> Option<Range> {
        let inner_left = cmp::max(self.0, r.0);
        let inner_right = cmp::min(self.1, r.1);
        if inner_left >= inner_right {
            None
        } else {
            Some(Range(inner_left, inner_right))
        }
    }

    fn len(&self) -> u64 {
        (self.1 - self.0).abs() as u64
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('=');
        let mut parts = parts.skip(1).next().unwrap().split("..");
        let left: i32 = parts.next().unwrap().parse()?;
        let right: i32 = parts.next().unwrap().parse()?;
        Ok(Range(left, right + 1))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cuboid(Range, Range, Range);

impl Cuboid {
    fn is_valid(&self) -> bool {
        self.0.is_valid() && self.1.is_valid() && self.2.is_valid()
    }

    fn clamp(&self, limits: (i32, i32)) -> Self {
        Cuboid(
            self.0.clamp(limits),
            self.1.clamp(limits),
            self.2.clamp(limits),
        )
    }

    fn intersection(&self, r: &Cuboid) -> Option<Cuboid> {
        if let Some(x_ranges) = self.0.intersection(&r.0) {
            if let Some(y_ranges) = self.1.intersection(&r.1) {
                if let Some(z_ranges) = self.2.intersection(&r.2) {
                    return Some(Cuboid(x_ranges, y_ranges, z_ranges));
                }
            }
        }
        None
    }

    fn volume(&self) -> u64 {
        self.0.len() * self.1.len() * self.2.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    On,
    Off,
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err("bad action"),
        }
    }
}

#[derive(Debug)]
struct Step {
    action: Action,
    cuboid: Cuboid,
}

impl FromStr for Step {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let action = parts.next().unwrap().parse().unwrap();
        let mut ranges = parts.next().unwrap().split(',').map(|s| s.parse().unwrap());

        let cuboid = Cuboid(
            ranges.next().unwrap(),
            ranges.next().unwrap(),
            ranges.next().unwrap(),
        );
        Ok(Step { action, cuboid })
    }
}

fn get_inputs(text: &str) -> Vec<Step> {
    text.lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn solve12(text: &str) -> u64 {
    let steps = get_inputs(text);
    let mut cubes = vec![vec![vec![Action::Off; 100]; 100]; 100];

    for s in steps {
        for x in s.cuboid.0 .0..s.cuboid.0 .1 {
            if x < -50 || x > 50 {
                continue;
            }
            for y in s.cuboid.1 .0..s.cuboid.1 .1 {
                if y < -50 || y > 50 {
                    continue;
                }
                for z in s.cuboid.2 .0..s.cuboid.2 .1 {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    let i = (x + 50) as usize;
                    let j = (y + 50) as usize;
                    let k = (z + 50) as usize;
                    cubes[i][j][k] = s.action;
                }
            }
        }
    }

    let mut count = 0;
    for sq in cubes {
        for row in sq {
            for i in row {
                if let Action::On = i {
                    count += 1;
                }
            }
        }
    }
    count
}

fn volume_after_steps(steps: Vec<Step>) -> u64 {
    let mut visited: Vec<Step> = Vec::new();
    let mut total_vol = 0;
    for s in steps {
        let mut intersections = Vec::new();
        for visited_step in visited.iter() {
            match s.cuboid.intersection(&visited_step.cuboid) {
                None => continue,
                Some(intersection) => {
                    match visited_step.action {
                        Action::Off => {
                            total_vol += intersection.volume();
                            intersections.push(Step {
                                action: Action::On,
                                cuboid: intersection,
                            });
                        }
                        Action::On => {
                            total_vol -= intersection.volume();
                            intersections.push(Step {
                                action: Action::Off,
                                cuboid: intersection,
                            });
                        }
                    };
                }
            }
        }
        visited.extend(intersections);
        if let Action::On = s.action {
            total_vol += s.cuboid.volume();
            visited.push(s);
        }
    }
    total_vol
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    let steps: Vec<Step> = get_inputs(text)
        .iter()
        .map(|s| Step {
            action: s.action,
            cuboid: s.cuboid.clamp((-50, 51)),
        })
        .filter(|s| s.cuboid.is_valid())
        .collect();

    volume_after_steps(steps)
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let steps = get_inputs(text);

    volume_after_steps(steps)
}
