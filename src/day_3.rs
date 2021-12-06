use std::fs::read_to_string;

const BINARY_WORD_SIZE: usize = 12;

fn get_input() -> Vec<String> {
    read_to_string("./inputs/day_3.txt")
        .unwrap()
        .lines()
        .map(|line| str::to_string(line.trim()))
        .collect()
}

pub fn solve_all_parts() -> (usize, usize) {
    (solve_first_part(), solve_second_part())
}

pub fn solve_first_part() -> usize {
    0
}

pub fn solve_second_part() -> usize {
    0
}
