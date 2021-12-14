use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/12/input";


const START: &str = "start";
const END: &str = "end";


fn is_small(cave: &str) -> bool {
    cave.chars().any(|c| c.is_lowercase())
}

fn get_inputs(text: &str) -> HashMap<&str, HashSet<&str>> {
    let mut caves = HashMap::new();

    for line in text.lines() {
        let mut parts = line.split('-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        if from != END && to != START {
            caves.entry(from)
                .or_insert(HashSet::new())
                .insert(to);
        }
        if from != START && to != END {
            caves.entry(to)
                .or_insert(HashSet::new())
                .insert(from);
        }
    }
    caves
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let caves = get_inputs(text);

    let mut stack = vec![vec![START]];
    let mut visited: Vec<&str> = Vec::new();
    let mut paths: Vec<Vec<&str>> = Vec::new();
    while !stack.is_empty() {
        let curr = match stack.last_mut().unwrap().pop() {
            None => {
                stack.pop();
                visited.pop();
                continue;
            },
            Some(v) => v,
        };
        if curr == END {
            paths.push(visited.clone());
            continue;
        }
        if let Some(next_caves) = caves.get(curr) {
            visited.push(curr);
            let next_stack: Vec<&str> = next_caves.iter()
                .filter(|c| !is_small(c) || !visited.contains(&c))
                .map(|&c| c)
                .collect();
            stack.push(next_stack);
        }
    }
    paths.iter().count()
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    let caves = get_inputs(text);

    let mut stack = vec![vec![START]];
    let mut visited: Vec<&str> = Vec::new();
    let mut paths: Vec<Vec<&str>> = Vec::new();
    let mut visited_small_twice = None;
    while !stack.is_empty() {
        let curr = match stack.last_mut().unwrap().pop() {
            None => {
                let prev = visited.pop();
                if let Some((v, p)) = visited_small_twice.zip(prev) {
                    if v == p {
                        visited_small_twice = None;
                    }
                }
                stack.pop();
                continue;
            },
            Some(v) => v,
        };
        if curr == END {
            paths.push(visited.clone());
            continue;
        }
        if is_small(curr) && visited.contains(&curr) {
            visited_small_twice = Some(curr);
        }
        if let Some(next_caves) = caves.get(curr) {
            visited.push(curr);
            let next_stack: Vec<&str> = next_caves.iter()
                .filter(|c| !is_small(c) || !visited.contains(&c) || visited_small_twice.is_none())
                .map(|&c| c)
                .collect();
            stack.push(next_stack);
        }
    }
    paths.iter().count()
}


// fn solve_recursive<'a>(
//     curr: &'a str,
//     caves: &HashMap<&'a str, HashSet<&'a str>>,
//     visited: Vec<&'a str>,
//     paths: &mut Vec<Vec<&'a str>>,
//     visited_small_twice: bool,
// ) {
//     if curr == END {
//         paths.push(visited.clone());
//         return;
//     }
//     if let Some(next_caves) = caves.get(curr) {
//         let mut visited = visited.clone();
//         visited.push(curr);
//         for cave in next_caves {
//             let mut visited_small_twice = visited_small_twice;
//             if is_small(cave) && visited.contains(cave) {
//                 if visited_small_twice {
//                     continue;
//                 }
//                 visited_small_twice = true;
//             }
//             solve_recursive(cave, caves, visited.clone(), paths, visited_small_twice);
//         }
//     }
// }
