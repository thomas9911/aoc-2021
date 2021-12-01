use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day1/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {}", part_one(input_file)?);
    println!("part two: {}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines().peekable();

    let mut difference = Vec::new();

    while let Some(line) = lines.next() {
        let parsed_line: i64 = line?.parse()?;
        if let Some(next_line) = lines.peek() {
            let parsed_next_line: i64 = next_line.as_ref().expect("is valid").parse()?;
            difference.push(parsed_next_line - parsed_line)
        }
    }

    Ok(difference.iter().filter(|x| x.is_positive()).count())
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    // we skip the first two because these contain zeroes
    let windows = WindowMaker::new(Box::new(reader.lines())).skip(2);
    let mut windows = windows.peekable();
    let mut difference = Vec::new();

    while let Some(parsed_line) = windows.next() {
        if let Some(parsed_next_line) = windows.peek() {
            difference.push(parsed_next_line - parsed_line)
        }
    }

    Ok(difference.iter().filter(|x| x.is_positive()).count())
}

pub struct Window(i64, i64, i64);

impl Default for Window {
    fn default() -> Window {
        Window(0, 0, 0)
    }
}

impl Window {
    pub fn sum(&self) -> i64 {
        self.0 + self.1 + self.2
    }

    pub fn push(&mut self, item: i64) -> &mut Self {
        self.2 = self.1;
        self.1 = self.0;
        self.0 = item;

        self
    }
}

pub struct WindowMaker<'a> {
    iter: Box<dyn Iterator<Item = Result<String, io::Error>> + 'a>,
    window: Window,
}

impl<'a> WindowMaker<'a> {
    pub fn new(iter: Box<dyn Iterator<Item = Result<String, io::Error>> + 'a>) -> WindowMaker<'a> {
        WindowMaker {
            iter,
            window: Window::default(),
        }
    }
}

impl<'a> Iterator for WindowMaker<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.iter.next() {
            if let Some(Ok(parsed_line)) = line.map(|x| x.parse::<i64>()).ok() {
                self.window.push(parsed_line);
                return Some(self.window.sum());
            }

            None
        } else {
            None
        }
    }
}


#[test]
fn day1_one() {
    assert_eq!(1559, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day1_two() {
    assert_eq!(1600, part_two(fetch_file_path()).unwrap())
}