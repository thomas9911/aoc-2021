use std::collections::BTreeMap;

use std::path::Path;

pub mod structs;
pub use structs::{InputIter, Pixel, Range2D};

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day5/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = fetch_file_path();
    println!("{:?}", part_one(input_path)?);
    println!("{:?}", part_two(input_path)?);
    Ok(())
}

fn part_one(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    part(path, part_one_filter)
}

fn part_two(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    part(path, part_two_filter)
}

fn part<F: Fn(&Range2D) -> bool>(
    path: &str,
    filter: F,
) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = InputIter::from_file(path)?;
    let reader = reader.filter(|line| {
        match line {
            Ok(x) => filter(x),
            // dont filter out the errors
            Err(_) => false,
        }
    });

    let mut points: BTreeMap<Pixel, usize> = BTreeMap::new();
    for range in reader {
        for item in range? {
            *points.entry(item).or_insert(0) += 1
        }
    }

    points.retain(|_, count| count != &1);

    Ok(points.keys().count())
}

fn part_one_filter(range: &Range2D) -> bool {
    range.0 .0 == range.1 .0 || range.0 .1 == range.1 .1
}

fn part_two_filter(range: &Range2D) -> bool {
    part_one_filter(range) || {
        let dir = range.direction();
        dir.0.abs() == dir.1.abs()
    }
}

#[test]
fn day5_part_one() {
    assert_eq!(6841, part_one(fetch_file_path()).unwrap());
}

#[test]
fn day5_part_two() {
    assert_eq!(19258, part_two(fetch_file_path()).unwrap());
}
