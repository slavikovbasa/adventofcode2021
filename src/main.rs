use adventofcode::client;
use adventofcode::day22 as day;

fn main() {
    let text = client::fetch(day::URL);
    // let text = "";

    println!("res1: {}", day::solve1(&text));
    println!("res2: {}", day::solve2(&text));
}
