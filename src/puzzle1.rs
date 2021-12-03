pub const URL: &str = "https://adventofcode.com/2021/day/1/input";

pub fn solve(text: &str) -> u32 {
    let arr: Vec<u32> = text.lines().map(|x| x.trim().parse().unwrap()).collect();

    let mut total = 0;
    let mut prev = arr[0] + arr[1] + arr[2];
    for i in 1..arr.len()-2 {
        let curr = arr[i] + arr[i+1] + arr[i+2];
        if curr > prev {
            total += 1;
        }
        prev = curr;
    }

    return total;
}
