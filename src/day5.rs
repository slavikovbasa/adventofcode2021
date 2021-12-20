use std::{str::FromStr, num::ParseIntError, collections::HashMap};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/5/input";

fn get_change_rates(start: &Point, end: &Point) -> (i32, i32) {
    let x_diff = end.0 - start.0;
    let y_diff = end.1 - start.1;
    if x_diff != 0 && y_diff != 0 && x_diff.abs() != y_diff.abs() {
        panic!("something went wrong")
    }
    (x_diff.signum(), y_diff.signum())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut coords = text.split(',');
        let (x, y) = (coords.next(), coords.next());
        let x = x.unwrap().parse()?;
        let y = y.unwrap().parse()?;
        Ok(Point(x, y))
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    i: u32,
    dir_x: i32,
    dir_y: i32,
}

impl Line {
    fn is_grid(&self) -> bool{
        self.dir_x == 0 || self.dir_y == 0
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let add_x = self.i as i32 * self.dir_x;
        let add_y = self.i as i32 * self.dir_y;
        if add_x.abs() > (self.end.0 - self.start.0).abs()
            || add_y.abs() > (self.end.1 - self.start.1).abs() {
            return None
        }
        self.i += 1;
        Some(Point(self.start.0 + add_x, self.start.1 + add_y))
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut points = text.split(" -> ").take(2);
        let start = points.next().unwrap().parse()?;
        let end = points.next().unwrap().parse()?;
        let (rate_x, rate_y) = get_change_rates(&start, &end);
        Ok(Line{
            start,
            end,
            i: 0,
            dir_x: rate_x,
            dir_y: rate_y,
        })
    }
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let mut space = HashMap::new();
    for line in text.lines().map(|l| l.parse::<Line>().unwrap()).filter(|l| l.is_grid()) {
        for point in line {
            let count = space.entry(point).or_insert(0);
            *count += 1;
        }
    }
    space.values().filter(|&&v| v > 1).count()
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    let mut space = HashMap::new();
    for line in text.lines().map(|l| l.parse::<Line>().unwrap()) {
        for point in line {
            let count = space.entry(point).or_insert(0);
            *count += 1;
        }
    }
    space.values().filter(|&&v| v > 1).count()
}
