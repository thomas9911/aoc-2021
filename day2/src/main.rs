use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day2/src/input.txt"
    }
}

type LineItem = Result<Result<Direction, String>, io::Error>;

#[derive(Debug)]
pub enum Direction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_i64(input: &str) -> Result<i64, String> {
    input.parse::<i64>().map_err(|x| x.to_string())
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        use Direction::*;

        let direction = match input.split_once(' ') {
            Some(("forward", amount)) => Forward(parse_i64(amount)?),
            Some(("down", amount)) => Down(parse_i64(amount)?),
            Some(("up", amount)) => Up(parse_i64(amount)?),
            _ => panic!("invalid line"),
        };

        Ok(direction)
    }
}

pub trait State {
    fn score(&self) -> i64;
    fn update(&mut self, direction: Direction);
}

#[derive(Debug, Default)]
pub struct PartOneState {
    position: i64,
    depth: i64,
}

impl State for PartOneState {
    fn update(&mut self, direction: Direction) {
        use Direction::*;

        match direction {
            Forward(x) => self.position += x,
            Down(x) => self.depth += x,
            Up(x) => self.depth -= x,
        };
    }

    fn score(&self) -> i64 {
        self.depth * self.position
    }
}

#[derive(Debug, Default)]
pub struct PartTwoState {
    position: i64,
    depth: i64,
    aim: i64,
}

impl State for PartTwoState {
    fn update(&mut self, direction: Direction) {
        use Direction::*;

        match direction {
            Forward(x) => {
                self.position += x;
                self.depth += self.aim * x;
            }
            Down(x) => self.aim += x,
            Up(x) => self.aim -= x,
        };
    }

    fn score(&self) -> i64 {
        self.depth * self.position
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("part one: {}", parts(true)?);
    println!("part two: {}", parts(false)?);

    Ok(())
}

fn parts(is_one: bool) -> Result<i64, Box<dyn std::error::Error>> {
    let mut state: Box<dyn State> = if is_one {
        Box::new(PartOneState::default())
    } else {
        Box::new(PartTwoState::default())
    };

    for line in line_iterator()? {
        state.update(line??);
    }

    Ok(state.score())
}

fn line_iterator() -> Result<Box<dyn Iterator<Item = LineItem>>, Box<dyn std::error::Error>> {
    let input_path = fetch_file_path();
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    let lines = reader
        .lines()
        .map(|line| line.map(|x| Direction::from_str(&x)));

    Ok(Box::new(lines))
}

#[test]
fn part_one() {
    assert_eq!(Ok(1868935), parts(true))
}

#[test]
fn part_two() {
    assert_eq!(Ok(1965970888), parts(false))
}
