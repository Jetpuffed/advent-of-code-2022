// Advent of Code 2022: Day 4
// Full prompt can be found here: https://adventofcode.com/2022/day/4

fn parse_ranges(input: String) -> ((i32, i32), (i32, i32)) {
    let (range_1, range_2) = input.split_once(',').unwrap();
    let (range_1_start, range_1_end) = range_1.split_once('-').unwrap();
    let (range_2_start, range_2_end) = range_2.split_once('-').unwrap();
    let (range_1_start, range_1_end) = (
        range_1_start.parse::<i32>().unwrap(),
        range_1_end.parse::<i32>().unwrap(),
    );
    let (range_2_start, range_2_end) = (
        range_2_start.parse::<i32>().unwrap(),
        range_2_end.parse::<i32>().unwrap(),
    );
    ((range_1_start, range_1_end), (range_2_start, range_2_end))
}

/// Prompt: In how many assignment pairs does one range fully contain the other?
pub fn puzzle_one(input: String) -> i32 {
    let mut contained_ranges = 0;
    for line in input.lines() {
        let ((range_1_start, range_1_end), (range_2_start, range_2_end)) =
            parse_ranges(line.to_owned());

        if ((range_1_start >= range_2_start) && (range_1_end <= range_2_end))
            || ((range_2_start >= range_1_start) && (range_2_end <= range_1_end))
        {
            contained_ranges += 1;
        }
    }
    contained_ranges
}

/// Prompt: In how many assignment pairs do the ranges overlap?
pub fn puzzle_two(input: String) -> i32 {
    use std::collections::HashSet;

    let mut overlapped_ranges = 0;
    for line in input.lines() {
        let ((range_1_start, range_1_end), (range_2_start, range_2_end)) =
            parse_ranges(line.to_owned());

        let mut seen = HashSet::new();
        for n in (range_1_start..=range_1_end).chain(range_2_start..=range_2_end) {
            if seen.contains(&n) {
                overlapped_ranges += 1;
                break
            }
            seen.insert(n);
        }
    }
    overlapped_ranges
}
