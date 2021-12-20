use std::collections::{HashSet};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/9/input";

const ZERO_CODE: u8 = 0x30; 

#[allow(dead_code)]
pub fn solve1(text: &str) -> u32 {
    let map: Vec<Vec<u8>> = text.lines()
        .map(|l| l.bytes().map(|b| b - ZERO_CODE).collect())
        .collect();
    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &point) in row.iter().enumerate() {
            if i > 0 && map[i-1][j] <= point {
                continue;
            }
            if i < map.len() - 1 && map[i+1][j] <= point {
                continue;
            }
            if j > 0 && map[i][j-1] <= point {
                continue;
            }
            if j < row.len() - 1 && map[i][j+1] <= point {
                continue;
            }
            sum += point as u32 + 1;
        }
    }
    sum
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point(usize, usize);

impl Point {
    fn neighbors(&self, last_point: &Point) -> Vec<Point> {
        let mut ns = Vec::new();
        if self.0 > 0 {
            ns.push(Point(self.0 - 1, self.1));
        }
        if self.0 < last_point.0 {
            ns.push(Point(self.0 + 1, self.1));
        }
        if self.1 > 0 {
            ns.push(Point(self.0, self.1 - 1));
        }
        if self.1 < last_point.1 {
            ns.push(Point(self.0, self.1 + 1));
        }
        ns
    }
}


fn populate_basin_from(
    p: Point,
    basin: &mut HashSet<Point>,
    map: &Vec<Vec<u8>>,
    last_point: &Point,
) {
    let cur_value = map[p.0][p.1];
    if cur_value == 9 || basin.contains(&p) {
        return;
    }
    basin.insert(p.clone());
    p.neighbors(last_point).into_iter()
        .filter(|n| map[n.0][n.1] != cur_value)
        .for_each(|point| { populate_basin_from(point, basin, map, last_point); });
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    let map: Vec<Vec<u8>> = text.lines()
        .map(|l| l.bytes().map(|b| b - ZERO_CODE).collect())
        .collect();
    let last_point = Point(map.len() - 1, map.first().unwrap().len() - 1);
    let mut basins = Vec::new();
    let mut points = HashSet::new();
    for i in 0..last_point.0 {
        for j in 0..last_point.1 {
            let p = Point(i, j);
            if map[p.0][p.1] == 9 {
                points.insert(Point(i, j));
                continue;
            }
            if points.contains(&p) {
                continue;
            }
            let mut b = HashSet::new();
            populate_basin_from(p, &mut b, &map, &last_point);
            if !b.is_empty(){
                basins.push(b.len());
                points.extend(b);
            }
        }
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

