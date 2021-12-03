mod puzzle3;
mod client;

fn main() {
    let text = client::fetch(puzzle3::URL);

    println!("res: {}", puzzle3::solve2(&text));
}
