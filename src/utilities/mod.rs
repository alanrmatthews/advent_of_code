use std::env;
use std::path::PathBuf;

pub fn get_input_path(year: i32, day: i32) -> PathBuf {
    let root = env::var("AOC_INPUT_ROOT")
        .expect("AOC_INPUT_ROOT environment variable not set");

    PathBuf::from(root)
        .join(year.to_string())
        .join(format!("day{day:02}.txt"))
}

pub fn get_input_contents(year: i32, day: i32) -> String {
    let path = get_input_path(year, day);
    std::fs::read_to_string(path)
        .expect("Failed to read input file")
}