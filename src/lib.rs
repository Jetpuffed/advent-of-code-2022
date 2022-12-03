pub mod day_01;
pub mod day_02;
pub mod day_03;

use std::fs;
use std::io;
use std::path::PathBuf;

const INPUT_ROOT_PATH: &str = "./inputs/";

pub fn get_input(day: i32, puzzle: i32) -> Result<String, io::Error> {
    let mut path = PathBuf::from(INPUT_ROOT_PATH);
    let day_dir = format!("day_{:02}/", day);
    path.push(day_dir);

    match puzzle {
        1 => path.push("puzzle_1"),
        2 => path.push("puzzle_2"),
        _ => unreachable!(),
    }
    fs::read_to_string(path)
}
