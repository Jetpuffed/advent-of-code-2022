// Advent of Code 2022: Day 5
// Full prompt can be found here: https://adventofcode.com/2022/day/5

const COL_INDICES: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];

fn parse_stacks(input: String) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::new(); 9];
    'outer: for line in input.lines() {
        let crates = line.chars().collect::<Vec<char>>();
        for (col, i) in COL_INDICES.iter().enumerate() {
            // Stop parsing once all crates have been read
            if crates[*i].is_ascii_digit() {
                break 'outer
            }
            // Only consider columns with actual values
            if crates[*i].is_ascii_uppercase() {
                stacks[col].push(crates[*i]);
            }
        }
    }
    // Since input stacks were read top to bottom, reverse them for correct order
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

fn parse_command(line: &str) -> [i32; 3] {
    let mut values = [0, 0, 0];
    // Command syntax: move {quantity} from {source} to {destination}
    let command_args = line.split(' ').map(|s| s.parse::<i32>());
    let mut i = 0;
    for arg in command_args {
        if let Ok(arg) = arg {
            values[i] = arg;
            i += 1;
        }
    }
    values
}

/// Prompt: After the rearrangement procedure completes, what crate ends up on
/// top of each stack?
pub fn puzzle_one(input: String) -> String {
    let mut stacks = parse_stacks(input.clone());
    for line in input.lines() {
        // Ignore any line that isn't a command
        if !line.contains("move") {
            continue
        }
        let parsed_cmd = parse_command(line);
        let (qty, src, dst) = (parsed_cmd[0], (parsed_cmd[1] - 1) as usize, (parsed_cmd[2] - 1) as usize);
        for _ in 0..qty {
            if let Some(val) = stacks[src].pop() {
                stacks[dst].push(val);
            }
        }
    }
    let mut result = String::new();
    for stack in &mut stacks {
        if let Some(top) = stack.pop() {
            result.push(top);
        }
    }
    result
}

/// Prompt: After the rearrangement procedure completes, what crate ends up on
/// top of each stack?
pub fn puzzle_two(input: String) -> String {
    let mut stacks = parse_stacks(input.clone());
    for line in input.lines() {
        if !line.contains("move") {
            continue
        }
        let parsed_cmd = parse_command(line);
        let (qty, src, dst) = (parsed_cmd[0] as usize, (parsed_cmd[1] - 1) as usize, (parsed_cmd[2] - 1) as usize);
        let stack_size = stacks[src].len();
        let start = stack_size - qty;
        // Can't mutably borrow `stacks` more than once at a time, so create
        // an intermediary buffer
        let mut buf = Vec::with_capacity(qty);
        for item in stacks[src].drain(start..stack_size) {
            buf.push(item);
        }
        for item in buf {
            stacks[dst].push(item);
        }
    }
    let mut result = String::new();
    for stack in &mut stacks {
        if let Some(top) = stack.pop() {
            result.push(top);
        }
    }
    result
}
