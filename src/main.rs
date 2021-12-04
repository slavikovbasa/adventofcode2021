mod client;
mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;

fn main() {
    let text = client::fetch(puzzle4::URL);

    println!("res: {}", puzzle4::solve2(&text));
}
