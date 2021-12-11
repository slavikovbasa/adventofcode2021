mod client;
mod puzzle11;
use puzzle11 as puzzle;

fn main() {
    let text = client::fetch(puzzle::URL);
//     let text = "5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526";

    println!("res1: {:?}", puzzle::solve1(&text));
    println!("res2: {:?}", puzzle::solve2(&text));
}
