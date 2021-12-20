use std::{collections::HashMap, hash::Hash};

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/14/input";


fn get_inputs(text: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut parts = text.split("\n\n");
    let template: Vec<char> = parts.next().unwrap().chars().collect();
    let rules: HashMap<(char, char), char> = parts.next().unwrap().lines().map(|l| {
        let mut from_to = l.split(" -> ");
        let mut from_iter = from_to.next().unwrap().chars();
        let from = (from_iter.next().unwrap(), from_iter.next().unwrap());
        let to = from_to.next().unwrap().chars().last().unwrap();
        (from, to)
    }).collect();

    (template, rules)
}

fn add_to<K: Eq + Hash>(map: &mut HashMap<K, u64>, key: K, n: u64) {
    let e = map.entry(key).or_insert(0);
    *e += n;
}

fn char_counts(
    pairs: &HashMap<(char, char), u64>,
    last_char: &char,
) -> HashMap<char, u64> {
    let mut counts = HashMap::new();

    pairs.iter().for_each(|((ch, _), count)| {
        add_to(&mut counts, *ch, *count);
    });
    add_to(&mut counts, *last_char, 1);

    counts
}

fn solve_for(text: &str, steps: usize) -> u64 {
    let (template, rules) = get_inputs(text);
    let last_char = template.last().unwrap();

    let mut pairs = HashMap::new();
    template.windows(2).for_each(|chars| {
        add_to(&mut pairs, (chars[0], chars[1]), 1);
    });

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        pairs.into_iter().for_each(|(pair, count)| {
            match rules.get(&pair) {
                None => {
                    add_to(&mut new_pairs, (pair.0, pair.1), count);
                },
                Some(&insertion) => {
                    add_to(&mut new_pairs, (pair.0, insertion), count);
                    add_to(&mut new_pairs, (insertion, pair.1), count);
                }
            }
        });
        pairs = new_pairs;
    }

    let counts = char_counts(&pairs, last_char);
    let most_common = counts.iter().max_by_key(|e| e.1).unwrap();
    let least_common = counts.iter().min_by_key(|e| e.1).unwrap();
    most_common.1 - least_common.1
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    solve_for(text, 10)
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    solve_for(text, 40)
}

