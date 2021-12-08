use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use derive_more::{Deref, DerefMut};

#[derive(Debug, PartialEq, Deref, DerefMut)]
pub struct Counter {
    data: BTreeMap<i64, i64>,
}

impl Counter {
    pub fn from_bufreader(reader: BufReader<File>) -> Result<Counter, Box<dyn std::error::Error>> {
        let data = reader
            .split(b',')
            .try_fold::<_, _, Result<_, Box<dyn std::error::Error>>>(
                BTreeMap::new(),
                |mut acc, x| match x {
                    Ok(x) => {
                        let number: i64 = String::from_utf8(x)?.parse()?;
                        *acc.entry(number).or_insert(0i64) += 1;
                        Ok(acc)
                    }
                    Err(e) => Err(Box::new(e)),
                },
            )?;

        Ok(Counter { data })
    }

    pub fn max(&self) -> i64 {
        // BTreeMap's are sorted on key, so the maximum value should be at the end.
        if let Some(max) = self.keys().next_back() {
            *max
        } else {
            0
        }
    }

    pub fn score_with<F: Fn(&i64) -> i64>(&self, scoring_function: F) -> i64 {
        self.iter()
            .map(|(value, amount)| scoring_function(value) * amount)
            .sum()
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day7/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    let counter = Counter::from_bufreader(reader)?;

    let minimum = (0..counter.max())
        .map(|guess| counter.score_with(|x| (x - guess).abs()))
        .min()
        .unwrap_or(i64::MAX);

    Ok(minimum)
}

fn part_two(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    let counter = Counter::from_bufreader(reader)?;

    let minimum = (0..counter.max())
        .map(|guess| {
            counter.score_with(|x| {
                let difference = (x - guess).abs();
                // using ye old sum formula here: (n+1)n/2
                ((1 + difference) * difference) / 2
            })
        })
        .min()
        .unwrap_or(i64::MAX);

    Ok(minimum)
}

#[test]
fn day7_part_one() {
    assert_eq!(337833, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day7_part_two() {
    assert_eq!(96678050, part_two(fetch_file_path()).unwrap())
}
