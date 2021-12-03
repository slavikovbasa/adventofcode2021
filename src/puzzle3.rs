pub const URL: &str = "https://adventofcode.com/2021/day/3/input";


enum BitFilter {
    Majority,
    Minority,
}

impl BitFilter {
    fn cmp(&self, z: isize, o: isize) -> isize {
        match self {
            BitFilter::Majority => z - o,
            BitFilter::Minority => o - z,
        }
    }
}

pub fn solve1(text: &str) -> u32 {
    let line_len = text.lines().next().unwrap().len();

    let mut total_len = 0;
    let mut counts = vec![0; line_len];
    for binary in text.lines().map(|l| l.trim()) {
        total_len += 1;
        if binary.len() != line_len {
            panic!("not all rows are the same size")
        }
        for (j, ch) in binary.chars().enumerate() {
            counts[j] += match ch {
                '0' => 0,
                '1' => 1,
                _ => panic!("not a binary number ('-_-)"),
            };
        }
    }
    let gamma_rate: String = counts.iter()
        .map(|x| if *x > total_len / 2 {'1'} else {'0'})
        .collect();
    let gamma = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon = !gamma & ((1 << line_len) - 1);
    gamma * epsilon
}


fn bit_partition<'a>(lines: &Vec<&'a str>, idx: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    lines.iter().partition(|s| s.chars().nth(idx).unwrap() == '0')
}


fn find_one(
    lines: Vec<&str>,
    idx: usize,
    bit_filter: BitFilter,
) -> Vec<&str> {
    if lines.len() <= 1 || idx > lines.iter().next().unwrap().len() {
        return lines;
    }

    let (zeroes, ones) = bit_partition(&lines, idx);
    let zeroes_len = zeroes.len() as isize;
    let ones_len = ones.len() as isize;

    let cmp = bit_filter.cmp(zeroes_len, ones_len);
    let subset = if cmp > 0  {
        zeroes
    } else if cmp < 0 {
        ones
    } else {
        match bit_filter {
            BitFilter::Minority => zeroes,
            BitFilter::Majority => ones,
        }
    };

    find_one(subset, idx + 1, bit_filter)
}

pub fn solve2(text: &str) -> u32 {
    let lines: Vec<&str> = text.lines().map(|l| l.trim()).collect();
    let lines_copy = lines.clone();

    let oxygen_rate = find_one(
        lines,
        0,
        BitFilter::Majority,
    );
    if oxygen_rate.len() != 1 {
        panic!("oxygen - something went wrong")
    }
    let oxygen_rate = oxygen_rate[0];

    let co2_rate = find_one(
        lines_copy,
        0,
        BitFilter::Minority,
    );
    if co2_rate.len() != 1 {
        panic!("co2 - something went wrong")
    }
    let co2_rate = co2_rate[0];


    let oxygen_rate = u32::from_str_radix(oxygen_rate, 2).unwrap();
    let co2_rate = u32::from_str_radix(co2_rate, 2).unwrap();
    oxygen_rate * co2_rate
}
