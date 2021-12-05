pub mod structs;

use std::fs::read_to_string;
use std::str::FromStr;

pub use structs::{Bingo, Board};
pub type Number = usize;

use std::path::Path;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day4/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {}", part_one(&input_file)?);
    println!("part two: {}", part_two(&input_file)?);

    Ok(())
}

fn part_one(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let text = read_to_string(path)?;
    let mut bingo = Bingo::from_str(&text)?;
    Ok(bingo.play_winning())
}

fn part_two(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let text = read_to_string(path)?;
    let mut bingo = Bingo::from_str(&text)?;
    Ok(bingo.play_losing())
}

#[test]
fn day3_one() {
    assert_eq!(46920, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day3_two() {
    assert_eq!(12635, part_two(fetch_file_path()).unwrap())
}
