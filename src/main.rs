use adventofcode::client;
use adventofcode::day21 as day;

fn main() {
    let text = client::fetch(day::URL);
    //     let text = "Player 1 starting position: 4
    // Player 2 starting position: 8";

    println!("res1: {}", day::solve1(&text));
    println!("res2: {}", day::solve2(&text));
}
