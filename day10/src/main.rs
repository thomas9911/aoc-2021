use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub enum Direction {
    Open,
    Close,
}

impl Direction {
    pub fn is_open(&self) -> bool {
        match self {
            Direction::Open => true,
            Direction::Close => false,
        }
    }

    pub fn is_close(&self) -> bool {
        !self.is_open()
    }
}

#[derive(Debug)]
pub enum Bracket {
    Round(Direction),
    Square(Direction),
    Curly(Direction),
    Angle(Direction),
}

const INCOMPLETE_MULTIPLIER: usize = 5;

impl Bracket {
    pub fn is_open(&self) -> bool {
        use Bracket::*;
        match self {
            Round(dir) | Square(dir) | Curly(dir) | Angle(dir) => dir.is_open(),
        }
    }

    pub fn is_close(&self) -> bool {
        !self.is_open()
    }

    pub fn error_score(&self) -> usize {
        use Bracket::*;

        match self {
            Round(..) => 3,
            Square(..) => 57,
            Curly(..) => 1197,
            Angle(..) => 25137,
        }
    }

    pub fn incomplete_score(&self) -> usize {
        use Bracket::*;

        match self {
            Round(..) => 1,
            Square(..) => 2,
            Curly(..) => 3,
            Angle(..) => 4,
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Bracket::*;
        use Direction::*;

        match value {
            '(' => Ok(Round(Open)),
            ')' => Ok(Round(Close)),
            '[' => Ok(Square(Open)),
            ']' => Ok(Square(Close)),
            '{' => Ok(Curly(Open)),
            '}' => Ok(Curly(Close)),
            '<' => Ok(Angle(Open)),
            '>' => Ok(Angle(Close)),
            _ => Err(String::from("invalid char")),
        }
    }
}

impl PartialEq for Bracket {
    fn eq(&self, other: &Bracket) -> bool {
        use Bracket::*;
        match (self, other) {
            (Round(..), Round(..)) => true,
            (Square(..), Square(..)) => true,
            (Curly(..), Curly(..)) => true,
            (Angle(..), Angle(..)) => true,
            _ => false,
        }
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day10/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);

    let mut score = 0;
    for line in buffer.lines() {
        if let Ok(line_score) = score_line_for_errors(&line?) {
            score += line_score;
        }
    }

    Ok(score)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);

    let mut scores = Vec::new();
    for line in buffer.lines() {
        if let Ok(line_score) = score_line_for_incomplete(&line?) {
            scores.push(line_score);
        }
    }

    scores.sort_unstable();
    let middle_point = scores[scores.len() / 2];
    Ok(middle_point)
}

fn score_line_for_errors(input: &str) -> Result<usize, String> {
    let mut stack: Vec<Bracket> = Vec::new();

    for ch in input.chars() {
        let bracket: Bracket = ch.try_into()?;
        if bracket.is_open() {
            stack.push(bracket)
        } else {
            if bracket != stack.pop().ok_or(String::from("pop from empty stack"))? {
                return Ok(bracket.error_score());
            }
        }
    }

    Err(String::from("no errors found"))
}

fn score_line_for_incomplete(input: &str) -> Result<usize, String> {
    let mut stack: Vec<Bracket> = Vec::new();

    for ch in input.chars() {
        let bracket: Bracket = ch.try_into()?;
        if bracket.is_open() {
            stack.push(bracket)
        } else {
            if bracket != stack.pop().ok_or(String::from("pop from empty stack"))? {
                return Err(String::from("corrupt line"));
            }
        }
    }

    if stack.is_empty() {
        return Err(String::from("line not incomplete"));
    }

    // line incomplete
    let mut score = 0;
    while let Some(item) = stack.pop() {
        score *= INCOMPLETE_MULTIPLIER;
        score += item.incomplete_score();
    }

    Ok(score)
}

#[test]
fn day10_part_one() {
    assert_eq!(394647, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day10_part_two() {
    assert_eq!(2380061249, part_two(fetch_file_path()).unwrap())
}
