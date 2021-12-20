#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/6/input";

const RESET_DAYS: usize = 7;
const MAX_DAYS: usize = 9;

fn init_population(text: &str, population: &mut [u64]) {
    assert_eq!(population.len(), MAX_DAYS);
    for f in text.trim().split(',').map(|i| i.parse::<usize>().unwrap()) {
        if f > MAX_DAYS - 1 {
            panic!("a weird fish")
        }
        population[f] += 1
    }
}

fn another_day(population: &mut [u64]) {
    let gonna_bear = population[0];
    population.rotate_left(1);
    population[RESET_DAYS - 1] += gonna_bear;
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    let mut population = [0 as u64; MAX_DAYS];
    init_population(text, &mut population);

    for _ in 0..80 {
        another_day(&mut population);
    }

    population.iter().sum()
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let mut population = [0 as u64; MAX_DAYS];
    init_population(text, &mut population);

    for _ in 0..256 {
        another_day(&mut population);
    }

    population.iter().sum()
}
