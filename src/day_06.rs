// Advent of Code 2022: Day 6
// Full prompt can be found here: https://adventofcode.com/2022/day/6

/// Prompt: How many characters need to be processed before the first
/// start-of-packet marker is detected?
pub fn puzzle_one(input: String) -> i32 {
    use std::collections::HashSet;

    let input = input.chars().collect::<Vec<char>>();
    let mut processed = 4;
    for window in input.windows(4) {
        let unique = window.iter().collect::<HashSet<&char>>();
        if unique.len() == 4 {
            return processed
        }
        processed += 1;
    }
    processed
}

/// Prompt: How many characters need to be processed before the first
/// start-of-message marker is detected?
pub fn puzzle_two(input: String) -> i32 {
    use std::collections::HashSet;

    let input = input.chars().collect::<Vec<char>>();
    let mut processed = 14;
    for window in input.windows(14) {
        let unique = window.iter().collect::<HashSet<&char>>();
        if unique.len() == 14 {
            return processed
        }
        processed += 1;
    }
    processed
}
