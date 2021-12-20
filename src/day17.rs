use std::collections::HashSet;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/17/input";

pub fn launch((mut v_x, mut v_y): (i32, i32), target: ((i32, i32), (i32, i32))) -> bool {
    let (x_range, y_range) = target;
    let is_before = |x, y| x < x_range.0 || y > y_range.1;
    let is_after = |x, y| x > x_range.1 || y < y_range.0;

    let mut x = 0;
    let mut y = 0;
    while is_before(x, y) {
        x += v_x;
        y += v_y;
        v_x -= v_x.signum();
        v_y -= 1;
    }

    !is_after(x, y)
}

fn get_inputs(text: &str) -> ((i32, i32), (i32, i32)) {
    let mut parts = text.trim().trim_start_matches("target area: ").split(", ");
    let mut x = parts.next().unwrap().trim_start_matches("x=").split("..");
    let x_range = (
        x.next().unwrap().parse().unwrap(),
        x.next().unwrap().parse().unwrap(),
    );

    let mut y = parts.next().unwrap().trim_start_matches("y=").split("..");
    let y_range = (
        y.next().unwrap().parse().unwrap(),
        y.next().unwrap().parse().unwrap(),
    );

    (x_range, y_range)
}

fn min_x_vel(x_range: (i32, i32)) -> i32 {
    assert!(x_range.0 < x_range.1);
    assert!(x_range.0 > 0);

    let mut vel = 0;
    while (vel * (vel + 1)) / 2 < x_range.0 {
        vel += 1;
    }
    vel
}

fn max_x_vel(x_range: (i32, i32)) -> i32 {
    assert!(x_range.0 < x_range.1);
    assert!(x_range.1 > 0);

    x_range.1
}

fn min_y_vel(y_range: (i32, i32)) -> i32 {
    assert!(y_range.0 < y_range.1);
    assert!(y_range.0 < 0);

    y_range.0
}

fn max_y_vel(y_range: (i32, i32)) -> i32 {
    assert!(y_range.0 < y_range.1);
    assert!(y_range.0 < 0);

    y_range.0.abs() - 1
}

#[allow(dead_code)]
pub fn solve1(text: &str) -> i32 {
    let (_, y_range) = get_inputs(text);
    (y_range.0.abs() * (y_range.0.abs() - 1)) / 2
}

#[allow(dead_code)]
pub fn solve2(text: &str) -> usize {
    let mut velocities = HashSet::new();

    let target = get_inputs(text);

    for vel_x in min_x_vel(target.0)..=max_x_vel(target.0) {
        for vel_y in min_y_vel(target.1)..=max_y_vel(target.1) {
            if launch((vel_x, vel_y), target) {
                velocities.insert((vel_x, vel_y));
            }
        }
    }

    velocities.len()
}
