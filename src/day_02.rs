// Advent of Code 2022: Day 2
// Full prompt can be found here: https://adventofcode.com/2022/day/2

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn decide_round(p1: &str, p2: &str) -> i32 {
    match (p1, p2) {
        ("A", "X") => 0,  // draw
        ("A", "Y") => 1,  // p2 wins
        ("A", "Z") => -1, // p1 wins
        ("B", "X") => -1,
        ("B", "Y") => 0,
        ("B", "Z") => 1,
        ("C", "X") => 1,
        ("C", "Y") => -1,
        ("C", "Z") => 0,
        _ => unreachable!()
    }
}

fn decide_move<'a>(p1: &str, p2: &str) -> &'a str {
    match (p1, p2) {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => unreachable!()
    }
}

fn move_score(mov: &str) -> i32 {
    match mov {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => unreachable!()
    }
}

/// Prompt: What would your total score be if everything goes exactly according
/// to your strategy guide?
pub fn puzzle_one(input: String) -> i32 {
    let mut total_score = 0;
    for line in input.lines() {
        if let Some((p1, p2)) = line.split_once(' ') {
            let result = decide_round(p1, p2);
            total_score += match result {
                -1 => LOSS,
                0 => DRAW,
                1 => WIN,
                _ => unreachable!(),
            };
            total_score += move_score(p2);
        }
    }
    total_score
}

/// Prompt: Following the Elf's instructions for the second column, what would
/// your total score be if everything goes exactly according to your strategy
/// guide?
pub fn puzzle_two(input: String) -> i32 {
    let mut total_score = 0;
    for line in input.lines() {
        if let Some((p1, p2)) = line.split_once(' ') {
            let move_to_play = decide_move(p1, p2);
            let result = decide_round(p1, move_to_play);
            total_score += match result {
                -1 => LOSS,
                0 => DRAW,
                1 => WIN,
                _ => unreachable!()
            };
            total_score += move_score(move_to_play);
        }
    }
    total_score
}
