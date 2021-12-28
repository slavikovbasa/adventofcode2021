use std::time::Instant;

use adventofcode::client;
use adventofcode::day24 as day;

fn main() {
    let text = client::fetch(day::URL);
    // let text = "";
    let now = Instant::now();
    let res1 = day::solve1(&text);
    println!("res1({}s): {}", now.elapsed().as_secs(), res1);

    let now = Instant::now();
    let res2 = day::solve2(&text);
    println!("res2({}s): {}", now.elapsed().as_secs(), res2);
}
