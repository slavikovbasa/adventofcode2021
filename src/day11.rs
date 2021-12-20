#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/11/input";

const STEPS: usize = 100;
const MAX_ENERGY: u32 = 9;

#[derive(Debug)]
struct Octopus(usize, usize);

impl Octopus {
    fn get_all(last_octopus: &Octopus) -> Vec<Octopus> {
        (0..=last_octopus.0)
            .map(|i| {
                (0..=last_octopus.1)
                    .map(|j| Octopus(i, j))
                    .collect::<Vec<Octopus>>()
            })
            .flatten()
            .collect()
    }

    fn neighbors(&self, last_octopus: &Octopus) -> Vec<Octopus> {
        let mut ns = Vec::new();
        let y = self.0 as i32;
        let x = self.1 as i32;
        let ver_ns = vec![y, y - 1, y + 1];
        let hor_ns = vec![x, x - 1, x + 1];

        for i in ver_ns {
            for j in hor_ns.iter() {
                if i == self.0 as i32 && *j == self.1 as i32 {
                    continue;
                }
                if i >= 0 && *j >= 0 && i <= last_octopus.0 as i32 && *j <= last_octopus.1 as i32 {
                    ns.push(Octopus(i as usize, *j as usize))
                }
            }
        }

        ns
    }

    fn energy<'a>(&self, energies: &'a mut Vec<Vec<u32>>) -> &'a mut u32 {
        &mut energies[self.0][self.1]
    }
}

fn proceed(energies: &mut Vec<Vec<u32>>) -> usize {
    let mut flashed = 0;
    let last_octopus = Octopus(energies.len() - 1, energies.first().unwrap().len() - 1);
    let mut affected_octopuses = Octopus::get_all(&last_octopus);

    while !affected_octopuses.is_empty() {
        let octopus = affected_octopuses.pop().unwrap();
        let energy = octopus.energy(energies);
        if *energy > MAX_ENERGY {
            continue;
        }
        if *energy == MAX_ENERGY {
            flashed += 1;
            affected_octopuses.extend(octopus.neighbors(&last_octopus))
        }
        *energy += 1;
    }

    energies.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|e| {
            if *e > MAX_ENERGY {
                *e = 0;
            }
        })
    });

    flashed
}

fn get_inputs(text: &str) -> Vec<Vec<u32>> {
    text.lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let mut grid = get_inputs(text);

    let mut total_flashed = 0;
    for _ in 0..STEPS {
        total_flashed += proceed(&mut grid);
    }
    total_flashed
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let mut grid: Vec<Vec<u32>> = get_inputs(text);

    let total_octopuses = grid.len() * grid.first().unwrap().len();

    let mut step = 0;
    loop {
        step += 1;
        let flashed = proceed(&mut grid);
        if flashed == total_octopuses {
            break step;
        }
    }
}
