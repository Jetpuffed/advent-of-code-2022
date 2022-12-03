// Advent of Code 2022: Day 3
// Full prompt can be found here: https://adventofcode.com/2022/day/3

use std::collections::HashMap;

fn priority_map() -> HashMap<char, i32> {
    let item_types = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority_map = HashMap::new();
    for (i, ch) in item_types.char_indices() {
        priority_map.insert(ch, (i + 1) as i32);
    }
    priority_map
}

/// Prompt: Find the item type that appears in both compartments of each
/// rucksack. What is the sum of the priorities of those item types?
pub fn puzzle_one(input: String) -> i32 {
    use std::collections::HashSet;

    let priority_map = priority_map();
    let mut priority_total = 0;
    for line in input.lines() {
        let rucksack_size = line.len() / 2;
        let mut seen = HashSet::new();
        for (i, ch) in line.char_indices() {
            if i < rucksack_size {
                seen.insert(ch);
            } else if seen.contains(&ch) {
                priority_total += priority_map[&ch];
                break
            }
        }
    }
    priority_total
}

/// Prompt: Find the item type that corresponds to the badges of each three-Elf
/// group. What is the sum of the priorities of those item types?
pub fn puzzle_two(input: String) -> i32 {
    use std::collections::HashSet;

    let priority_map = priority_map();
    let mut priority_total = 0;
    let mut seen = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        if i % 3 == 0 {
            if !seen.is_empty() {
                let ch = seen.drain().next().unwrap();
                priority_total += priority_map[&ch];
            }
            for ch in line.chars() {
                seen.insert(ch);
            }
            continue
        }
        seen = line.chars().filter(|ch| seen.contains(ch)).collect::<HashSet<char>>();
    }
    let ch = seen.drain().next().unwrap();
    priority_total += priority_map[&ch];
    priority_total
}
