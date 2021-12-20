use std::{fmt::Display, collections::{HashSet, HashMap}, ops::{Sub, Add}};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/19/input";


fn cos_disc(n: u32) -> i32 {
    if n % 2 == 1 {
        0
    } else {
        if n % 4 == 0 {
            1
        } else {
            -1
        }
    }
}

fn sin_disc(n: u32) -> i32 {
    cos_disc(n + 1)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(i32, i32, i32);

impl Point {
    fn sum_abs(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
 
    fn rotate_x(&self, n: u32) -> Self {
        Point(
            self.0,
            cos_disc(n) * self.1 - sin_disc(n) * self.2,
            sin_disc(n) * self.1 + cos_disc(n) * self.2,
        )
    }

    fn rotate_y(&self, n: u32) -> Self {
        Point(
            cos_disc(n) * self.0 + sin_disc(n) * self.2,
            self.1,
            -sin_disc(n) * self.0 + cos_disc(n) * self.2,
        )
    }

    fn rotate_z(&self, n: u32) -> Self {
        Point(
            cos_disc(n) * self.0 - sin_disc(n) * self.1,
            sin_disc(n) * self.0 + cos_disc(n) * self.1,
            self.2,
        )
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}


#[derive(Clone, Debug)]
struct Scanner {
    id: usize,
    points: HashSet<Point>,
    pos: Point,
}

fn vectors(points: &HashSet<Point>) -> HashMap<Point, (Point, Point)> {
    let mut vectors: HashMap<Point, (Point, Point)> = HashMap::new();

    for p1 in points {
        for p2 in points {
            if p1 == p2 {
                continue;
            }
            let p1 = p1.clone();
            let p2 = p2.clone();
            vectors.insert(p1 - p2, (p1, p2));
        }
    }

    vectors
}

impl Scanner {
    fn try_adjust_for(
        &self,
        base: &Scanner,
        threshold: usize,
    ) -> Option<(HashSet<Point>, Point)> {
        let base_points = &base.points;
        let base_vectors = vectors(base_points);
        for rot_x in 0..4 {
            for rot_y in 0..4 {
                for rot_z in 0..4 {
                    let rotated = self.points.iter()
                        .map(|p| p.rotate_x(rot_x).rotate_y(rot_y).rotate_z(rot_z))
                        .collect();
                    let vectors = vectors(&rotated);
                    let common_vectors: HashSet<_> = base_vectors.keys()
                        .filter(|k| vectors.contains_key(*k))
                        .collect();

                    if common_vectors.len() < threshold {
                        continue;
                    }
                    let matched_vec = common_vectors.iter().next().unwrap();
                    let base_point = base_vectors.get(matched_vec).unwrap().0;
                    let point = vectors.get(matched_vec).unwrap().0;

                    let diff = base_point - point;

                    let translated_points: HashSet<_> = rotated.into_iter().map(|p| p + diff).collect();
                    return Some((translated_points, diff));
                }
            }
        }
        None
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("--- scanner {} ---\n", self.id));
        for p in self.points.iter() {
            result.push_str(&format!("{}\n", p));
        }
        write!(f, "{}", result)
    }
}

fn get_inputs(text: &str) -> Vec<Scanner> {
    text.split("\n\n").enumerate().map(|(id, scanner)| {
        let points = scanner.lines().skip(1).map(|point| {
            let mut parts = point.split(',')
                .map(|coord| coord.parse::<i32>().unwrap());
            Point(parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap())
        }).collect();
        Scanner { id, points, pos: Point(0, 0, 0) }
    }).collect()
}

fn adjust_scanners(scanners: &mut Vec<Scanner>) {
    let mut stack = vec![scanners[0].clone()];
    let mut visited = HashSet::new();
    visited.insert(0);

    while !stack.is_empty() {
        let base_scanner = stack.pop().unwrap();
        for s in scanners.iter_mut() {
            if visited.contains(&s.id) {
                continue;
            }
            match s.try_adjust_for(&base_scanner, 12) {
                None => continue,
                Some((v, pos)) => {
                    s.points = v;
                    s.pos = pos;
                    stack.push(s.clone());
                    visited.insert(s.id);
                },
            }
        }
    }
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let mut scanners: Vec<Scanner> = get_inputs(text);

    adjust_scanners(&mut scanners);

    let all_points = scanners.into_iter().fold(
        HashSet::new(),
        |acc, s| acc.union(&s.points).cloned().collect()
    );
    let mut sorted_points: Vec<Point> = all_points.into_iter().collect();
    sorted_points.sort_unstable_by_key(|p| p.0);
    for p in sorted_points.iter() {
        println!("{}", p);
    }
    sorted_points.len()
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> i32 {
    let mut scanners: Vec<Scanner> = get_inputs(text);

    adjust_scanners(&mut scanners);
    scanners.iter().flat_map(
        |s1| scanners.iter().map(|s2| (s1.pos - s2.pos).sum_abs()).collect::<Vec<_>>()
    ).max().unwrap()
}

