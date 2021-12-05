mod client;
mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;

fn main() {
    let text = client::fetch(puzzle5::URL);

    println!("res: {}", puzzle5::solve2(&text));
}
