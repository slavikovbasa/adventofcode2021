use std::collections::HashMap;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/21/input";

#[derive(PartialEq, Eq, Hash, Clone)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Player { pos, score: 0 }
    }

    fn go(&self, n: usize) -> Player {
        let target = (self.pos + n) % 10;
        Player {
            pos: target,
            score: self.score + target + 1,
        }
    }

    fn roll(&mut self, die: &mut impl Iterator<Item = usize>, n: usize) -> Player {
        self.go(die.take(n).sum::<usize>())
    }

    fn roll_dirac(&self) -> Vec<Player> {
        let mut new_players = Vec::new();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    new_players.push(self.go(i + j + k))
                }
            }
        }
        new_players
    }
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> usize {
    let mut players = text.trim().lines().map(|l| {
        l.split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });
    let mut p1 = Player::new(players.next().unwrap() - 1);
    let mut p2 = Player::new(players.next().unwrap() - 1);

    let die = &mut (1..=100).cycle();

    let mut roll_count = 0;

    let losing = loop {
        p1 = p1.roll(die, 3);
        roll_count += 3;
        if p1.score >= 1000 {
            break p2.score;
        }

        p2 = p2.roll(die, 3);
        roll_count += 3;
        if p2.score >= 1000 {
            break p1.score;
        }
    };

    losing * roll_count
}

fn player_counters(
    first: &Player,
    second: &Player,
    counters: &mut HashMap<(Player, Player), (u64, u64)>,
) -> (u64, u64) {
    let key = (first.clone(), second.clone());
    if let Some(&v) = counters.get(&key) {
        return v;
    }

    if first.score >= 21 {
        counters.insert(key, (1, 0));
        return (1, 0);
    }

    if second.score >= 21 {
        counters.insert(key, (0, 1));
        return (0, 1);
    }

    let mut counts = (0, 0);

    for p in first.roll_dirac() {
        let next_counts = player_counters(second, &p, counters);
        counts.0 += next_counts.1;
        counts.1 += next_counts.0;
    }

    counters.insert(key, counts);
    counts
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> String {
    let mut players = text.trim().lines().map(|l| {
        l.split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });
    let p1 = Player::new(players.next().unwrap() - 1);
    let p2 = Player::new(players.next().unwrap() - 1);

    let (p1_wins, p2_wins) = player_counters(&p1, &p2, &mut HashMap::new());

    format!("{} {}", p1_wins, p2_wins)
}
