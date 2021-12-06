mod day_1;
mod day_2;
mod day_3;

fn main() {
    let day = 3;
    let (first_part, second_part) = day_3::solve_all_parts();

    println!("Solutions for day {}", day);
    println!("Part 1: {}", first_part);
    println!("Part 2: {}", second_part);
}
