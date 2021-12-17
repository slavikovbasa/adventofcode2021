mod client;
mod puzzle16;
use puzzle16 as puzzle;

fn main() {
    let text = client::fetch(puzzle::URL);
    // let text = "C0015000016115A2E0802F182340";

    println!("res1: {}", puzzle::solve1(&text));
    println!("res2: {}", puzzle::solve2(&text));
}
