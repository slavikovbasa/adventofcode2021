use std::{collections::BinaryHeap, cmp::Ordering};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/15/input";


type Point = (usize, usize);

#[derive(PartialEq, Eq)]
struct Node {
    p: Point,
    dist: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist).then_with(|| other.p.cmp(&self.p))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(p: &Point, last_point: &Point) -> Vec<Point> {
    let mut ns = Vec::new();
    if p.0 as i32 - 1 >= 0 {
        ns.push((p.0 - 1, p.1))
    }

    if p.0 + 1 <= last_point.0 {
        ns.push((p.0 + 1, p.1))
    }

    if p.1 as i32 - 1 >= 0 {
        ns.push((p.0, p.1 - 1))
    }

    if p.1 + 1 <= last_point.1 {
        ns.push((p.0, p.1 + 1))
    }

    ns
}

fn dijkstra(
    weights: &Vec<Vec<usize>>,
    first: &Point,
    last: &Point,
) -> Result<Node, String> {
    let mut distances: Vec<Vec<_>> = (0..=last.0).map(|_| {
        (0..=last.1).map(|_| usize::MAX).collect()
    }).collect();
    distances[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Node { p: *first, dist: distances[first.0][first.1] });

    while let Some(curr) = heap.pop() {
        if curr.p == *last {
            return Ok(curr);
        }
        if curr.dist > distances[curr.p.0][curr.p.1] {
            continue;
        }
        for next in neighbors(&curr.p, &last) {
            let next_dist = distances[next.0][next.1];
            let curr_next_dist = curr.dist + weights[next.0][next.1];
            if curr_next_dist < next_dist {
                heap.push(Node { p: next, dist: curr_next_dist });
                distances[next.0][next.1] = curr_next_dist;
            }
        }
    }

    Err(String::from("didnt find finish"))
}


#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let height = text.lines().count();
    let width = text.lines().next().unwrap().len();
    let weights: Vec<Vec<_>> = text.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
    }).collect();

    let mut distances: Vec<Vec<_>> = (0..height).map(|_| {
        (0..width).map(|_| usize::MAX).collect()
    }).collect();

    distances[0][0] = 0;

    let first = (0, 0);
    let last = (height - 1, width - 1);
    let finished_node = dijkstra(&weights, &first, &last).unwrap();
    finished_node.dist
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    let height = text.lines().count();
    let width = text.lines().next().unwrap().len();

    let weights: Vec<Vec<_>> = text.lines().cycle().take(5 * height).enumerate().map(
        |(i, l)| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .cycle()
                .take(5 * width)
                .enumerate()
                .map(|(j, e)| {
                    match (e + i / height + j / width) % 9 {
                        0 => 9,
                        v => v,
                    }
                }).collect()
        }
    ).collect();

    let height = height * 5;
    let width = width * 5;

    let first = (0, 0);
    let last = (height - 1, width - 1);
    let finished_node = dijkstra(&weights, &first, &last).unwrap();
    finished_node.dist
}

