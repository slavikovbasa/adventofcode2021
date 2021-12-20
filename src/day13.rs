use std::collections::HashSet;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/13/input";

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

fn fold(coords: HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    coords
        .iter()
        .map(|p| match fold {
            Fold::X(n) if p.0 > *n => Point(2 * n - p.0, p.1),
            Fold::Y(n) if p.1 > *n => Point(p.0, 2 * n - p.1),
            _ => Point(p.0, p.1),
        })
        .collect()
}

fn get_inputs(text: &str) -> (HashSet<Point>, Vec<Fold>) {
    let mut parts = text.split("\n\n");
    let coords: HashSet<Point> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut coords = l.split(',');
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            Point(x, y)
        })
        .collect();
    let folds: Vec<Fold> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut instr = l.trim_start_matches("fold along ").split('=');
            let dir = instr.next().unwrap();
            let n = instr.next().unwrap().parse().unwrap();
            match dir {
                "x" => Fold::X(n),
                "y" => Fold::Y(n),
                _ => panic!("bad fold instruction"),
            }
        })
        .collect();

    (coords, folds)
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let (coords, folds) = get_inputs(text);
    fold(coords, folds.first().unwrap()).iter().count()
}

fn coords_to_str(coords: &HashSet<Point>) -> String {
    let height = coords.iter().max_by_key(|p| p.1).unwrap().1 + 1;
    let width = coords.iter().max_by_key(|p| p.0).unwrap().0 + 1;

    let mut res = String::new();
    res.push('\n');
    for i in 0..height {
        for j in 0..width {
            if coords.contains(&Point(j, i)) {
                res.push('#');
            } else {
                res.push('.');
            }
        }
        res.push('\n');
    }
    res
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> String {
    let (mut coords, folds) = get_inputs(text);

    for f in folds {
        coords = fold(coords, &f);
    }
    coords_to_str(&coords)
}
