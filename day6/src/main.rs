use derive_more::{Deref, DerefMut};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Lanternfish {
    days_left: usize,
}

impl Lanternfish {
    pub fn new(days_left: usize) -> Lanternfish {
        Lanternfish { days_left }
    }
    pub fn new_born_fish() -> Lanternfish {
        Lanternfish::new(8)
    }

    pub fn next_day(&mut self) -> Option<Lanternfish> {
        if let Some(days_left) = self.days_left.checked_sub(1) {
            self.days_left = days_left;
            None
        } else {
            self.reset();
            Some(Lanternfish::new_born_fish())
        }
    }

    pub fn reset(&mut self) {
        self.days_left = 6
    }

    pub fn has_new_born_fish(&self) -> bool {
        self.days_left.checked_sub(1).is_none()
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct School {
    #[deref]
    #[deref_mut]
    fishes: BTreeMap<Lanternfish, usize>,
}

impl School {
    pub fn simulate_days(&mut self, amount_of_days: usize) {
        for _ in 0..amount_of_days {
            let mut next_day = BTreeMap::new();
            for (mut fish, count) in self.fishes.clone() {
                if fish.has_new_born_fish() {
                    next_day.insert(Lanternfish::new_born_fish(), count);
                    fish.reset();
                    next_day.insert(fish, count);
                } else {
                    fish.days_left -= 1;
                    *next_day.entry(fish).or_insert(0) += count;
                };
            }

            self.fishes = next_day
        }
    }

    pub fn print_as_list(&self, day: &usize) {
        print!("{}: ", day);
        for (k, v) in self.fishes.iter() {
            for _ in 0..*v {
                print!("{}, ", k.days_left)
            }
        }
        println!()
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day6/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", parts(input_file, 80)?);
    println!("part two: {:?}", parts(input_file, 256)?);
    Ok(())
}

fn parts(input_path: &str, till: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    let mut school = School::default();
    for number in reader.split(',' as u8) {
        *school
            .entry(Lanternfish::new(String::from_utf8(number?)?.parse()?))
            .or_insert(0) += 1;
    }

    school.simulate_days(till);
    Ok(school.values().sum())
}

#[test]
fn day6_part_one() {
    assert_eq!(388419, parts(fetch_file_path(), 80).unwrap())
}

#[test]
fn day6_part_two() {
    assert_eq!(1740449478328, parts(fetch_file_path(), 256).unwrap())
}
