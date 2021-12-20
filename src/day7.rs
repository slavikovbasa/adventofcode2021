#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/7/input";


fn get_fuel_cost(d: &[i32]) -> i32 {
    d.iter().map(|d| (d.abs() * (d.abs() + 1)) / 2).sum()
}

fn min_fuel_cost_recursive(
    dest: usize,
    distances: Vec<i32>,
    fuel_cost: i32,
    total: usize,
    max_position: usize,
) -> (usize, i32) {
    if dest == 0 || dest == max_position {
        return (dest, fuel_cost);
    }
    
    // try to decrease dest
    let distances: Vec<i32> = distances.into_iter().map(|d| d + 1).collect();
    let decr_fuel_cost = get_fuel_cost(&distances);
    if decr_fuel_cost < fuel_cost {
        return min_fuel_cost_recursive(
            dest - 1,
            distances,
            decr_fuel_cost,
            total,
            max_position,
        );
    }

    // try to increase dest
    let distances: Vec<i32> = distances.iter().map(|&d| d - 2).collect();
    let incr_fuel_cost = get_fuel_cost(&distances);
    if incr_fuel_cost < fuel_cost {
        return min_fuel_cost_recursive(
            dest + 1,
            distances,
            incr_fuel_cost,
            total,
            max_position,
        );
    }
    return (dest, fuel_cost);
}


fn min_fuel_cost(
    positions: &[usize],
    behind_or_at: &[usize],
    max_position: usize,
) -> (usize, i32) {
    let total = positions.len();
    let mut dest = positions[total / 2];
    let mut fuel_cost = positions.iter()
        .map(|&p| (p as i32 - dest as i32).abs()).sum();
    loop {
        if dest == 0 || dest == max_position {
            break (dest, fuel_cost);
        }
        
        // try to decrease dest
        let behind = behind_or_at[dest-1];
        let after = total - behind;
        let fuel_diff = after as i32 - behind as i32;
        if fuel_diff < 0 {
            dest -= 1;
            fuel_cost += fuel_diff;
            continue;
        }

        // try to increase dest
        let behind = behind_or_at[dest];
        let after = total - behind;
        let fuel_diff = behind as i32 - after as i32;
        if fuel_diff < 0 {
            dest += 1;
            fuel_cost += fuel_diff;
            continue;
        }
        break (dest, fuel_cost);
    }
}


#[allow(dead_code)]
pub fn solve1(text: &str) -> String {
    let mut positions: Vec<usize> = text.trim().split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    positions.sort_unstable();

    let len = positions.len();
    let &max_position = positions.iter().max().unwrap();

    let mut behind_or_at = Vec::with_capacity(max_position + 1);
    let mut idx = 0;
    for i in 0..=max_position {
        while idx < len && positions[idx] == i {
            idx += 1;
        }
        behind_or_at.push(idx);
    }


    let (dest, fuel_cost) = min_fuel_cost(&positions, &behind_or_at, max_position);
    format!("({}, {})", dest, fuel_cost)
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> String {
    let mut positions: Vec<usize> = text.trim().split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    positions.sort_unstable();

    let &max_position = positions.iter().max().unwrap();

    let total = positions.len();
    let dest = positions[total / 2];
    let distances: Vec<i32> = positions.iter()
        .map(|&p| p as i32 - dest as i32).collect();

    let fuel_cost = get_fuel_cost(&distances);
    let (dest, fuel_cost) = min_fuel_cost_recursive(dest, distances, fuel_cost, total, max_position);
    format!("({}, {})", dest, fuel_cost)
}
