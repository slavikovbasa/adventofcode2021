use std::time::Instant;

use adventofcode::client;
use adventofcode::day23 as day;

fn main() {
    let text = client::fetch(day::URL);
    // let text = "";
    let now = Instant::now();
    println!("res1({}s): {}", now.elapsed().as_secs(), day::solve1(&text));

    let now = Instant::now();
    println!("res2({}s): {}", now.elapsed().as_secs(), day::solve2(&text));
}
