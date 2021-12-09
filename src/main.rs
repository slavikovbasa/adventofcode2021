mod client;
mod puzzle8;
use puzzle8 as puzzle;

fn main() {
    let text = client::fetch(puzzle::URL);
    // let text = "";

    println!("res1: {:?}", puzzle::solve1(&text));
    println!("res2: {:?}", puzzle::solve2(&text));
}
