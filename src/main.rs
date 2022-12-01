use advent_of_code_2022::*;

fn main() {
    let day_one_input_one = get_input(1, 1).expect("Input is invalid");
    println!("{}", day_01::puzzle_one(day_one_input_one.clone()));
    println!("{}", day_01::puzzle_two(day_one_input_one)); // input hasn't changed
}
