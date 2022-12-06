use advent_of_code_2022::*;

fn main() {
    println!("\nDay One:");
    let day_one_input_one = get_input(1, 1).expect("Input is invalid");
    println!("{}", day_01::puzzle_one(day_one_input_one.clone()));
    println!("{}", day_01::puzzle_two(day_one_input_one)); // input hasn't changed

    println!("\nDay Two:");
    let day_two_input_one = get_input(2, 1).expect("Input is invalid");
    println!("{}", day_02::puzzle_one(day_two_input_one.clone()));
    println!("{}", day_02::puzzle_two(day_two_input_one)); // input hasn't changed

    println!("\nDay Three:");
    let day_three_input_one = get_input(3, 1).expect("Input is invalid");
    println!("{}", day_03::puzzle_one(day_three_input_one.clone()));
    println!("{}", day_03::puzzle_two(day_three_input_one)); // input hasn't changed

    println!("\nDay Four:");
    let day_four_input_one = get_input(4, 1).expect("Input is invalid");
    println!("{}", day_04::puzzle_one(day_four_input_one.clone()));
    println!("{}", day_04::puzzle_two(day_four_input_one)); // input hasn't changed

    println!("\nDay Five:");
    let day_five_input_one = get_input(5, 1).expect("Input is invalid");
    println!("{}", day_05::puzzle_one(day_five_input_one.clone()));
    println!("{}", day_05::puzzle_two(day_five_input_one)); // input hasn't changed

    println!("\nDay Six:");
    let day_six_input_one = get_input(6, 1).expect("Input is invalid");
    println!("{}", day_06::puzzle_one(day_six_input_one.clone()));
    println!("{}", day_06::puzzle_two(day_six_input_one)); // input hasn't changed
}
