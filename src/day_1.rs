use itertools::izip;
use std::fs::read_to_string;

fn get_input() -> Vec<i32> {
    read_to_string("./inputs/day_1.txt")
        .unwrap()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

pub fn solve_all_parts() -> (usize, usize) {
    (solve_first_part(), solve_second_part())
}

pub fn solve_first_part() -> usize {
    get_input().windows(2).filter(|w| w[0] < w[1]).count()
}

pub fn solve_second_part() -> usize {
    let input = get_input();

    izip!(input.iter(), input.iter().skip(1), input.iter().skip(2))
        .map(|(a, b, c)| a + b + c)
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}
