// Advent of Code 2022: Day 1
// Full prompt can be found here: https://adventofcode.com/2022/day/1

/// Prompt: Find the Elf carrying the most Calories. How many total Calories is
/// that Elf carrying?
pub fn puzzle_one(input: String) -> i32 {
    let mut max_calories = i32::MIN;
    let mut curr_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            max_calories = max_calories.max(curr_calories);
            curr_calories = 0;
            continue
        }
        let value = line.parse::<i32>().expect("Unable to convert value to i32");
        curr_calories += value;
    }
    max_calories
}

/// Prompt: Find the top three Elves caryying the most Calories. How many
/// Calories are those Elves carrying in total?
pub fn puzzle_two(input: String) -> i32 {
    use std::collections::BTreeSet;

    let mut elf_calories = BTreeSet::new();
    let mut curr_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            elf_calories.insert(curr_calories);
            curr_calories = 0;
            continue
        }
        let value = line.parse::<i32>().expect("Unable to convert value to i32");
        curr_calories += value;
    }
    elf_calories.iter().rev().take(3).sum()
}
