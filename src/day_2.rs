use std::fs::read_to_string;

struct SubmarineTelemetry {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl SubmarineTelemetry {
    fn new() -> SubmarineTelemetry {
        SubmarineTelemetry {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }

    fn position(&self) -> i32 {
        self.horizontal * self.depth
    }
}

enum Move {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn get_input() -> Vec<Move> {
    read_to_string("./inputs/day_2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let move_tokens = line.split(" ").collect::<Vec<&str>>();
            let amount = move_tokens.get(1).unwrap().parse().unwrap();

            match move_tokens.get(0).unwrap() {
                &"forward" => Move::Forward(amount),
                &"up" => Move::Up(amount),
                &"down" => Move::Down(amount),
                _ => panic!(),
            }
        })
        .collect()
}

pub fn solve_all_parts() -> (i32, i32) {
    (solve_first_part(), solve_second_part())
}

pub fn solve_first_part() -> i32 {
    let mut submarine_telemetry = SubmarineTelemetry::new();
    let moves = get_input();

    for sub_move in moves.iter() {
        match sub_move {
            Move::Forward(amount) => submarine_telemetry.horizontal += amount,
            Move::Up(amount) => submarine_telemetry.depth -= amount,
            Move::Down(amount) => submarine_telemetry.depth += amount,
        }
    }
    submarine_telemetry.position()
}

pub fn solve_second_part() -> i32 {
    let mut submarine_telemetry = SubmarineTelemetry::new();
    let moves = get_input();

    for sub_move in moves.iter() {
        match sub_move {
            Move::Forward(amount) => {
                submarine_telemetry.horizontal += amount;

                if submarine_telemetry.aim > 0 {
                    submarine_telemetry.depth += submarine_telemetry.aim * amount
                }
            }
            Move::Up(amount) => submarine_telemetry.aim -= amount,
            Move::Down(amount) => submarine_telemetry.aim += amount,
        }
    }
    submarine_telemetry.position()
}
