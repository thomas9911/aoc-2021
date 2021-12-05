use derive_more::{Add, From, Sub};
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, From, Add, Sub)]
pub struct Pixel(pub i64, pub i64);

impl FromStr for Pixel {
    type Err = String;
    fn from_str(s: &str) -> Result<Pixel, Self::Err> {
        if let Some((left, right)) = s.split_once(",") {
            let x = left.parse::<i64>().map_err(|e| e.to_string())?;
            let y = right.parse::<i64>().map_err(|e| e.to_string())?;
            Ok(Pixel::from((x, y)))
        } else {
            Err(String::from("invalid pixel input"))
        }
    }
}

impl Pixel {
    pub fn normalize(self) -> Self {
        let vertical_dir = self.0.signum();
        let horizontal_dir = self.1.signum();
        Pixel(vertical_dir, horizontal_dir)
    }
}

#[derive(Debug, PartialEq, From)]
pub struct Range2D(pub Pixel, pub Pixel);

impl FromStr for Range2D {
    type Err = String;
    fn from_str(s: &str) -> Result<Range2D, Self::Err> {
        if let Some((left, right)) = s.split_once(" -> ") {
            let start = Pixel::from_str(left)?;
            let end = Pixel::from_str(right)?;
            Ok(Range2D(start, end))
        } else {
            Err(String::from("invalid line input"))
        }
    }
}

impl From<((i64, i64), (i64, i64))> for Range2D {
    fn from(input: ((i64, i64), (i64, i64))) -> Range2D {
        Range2D(Pixel::from(input.0), Pixel::from(input.1))
    }
}

impl Range2D {
    pub fn direction(&self) -> Pixel {
        self.1.clone() - self.0.clone()
    }
}

impl IntoIterator for Range2D {
    type Item = Pixel;
    type IntoIter = Range2DIter;

    fn into_iter(self) -> Range2DIter {
        Range2DIter {
            next: Some(self.0.clone()),
            end: self.1.clone(),
            direction: self.direction().normalize(),
        }
    }
}

pub struct Range2DIter {
    end: Pixel,
    next: Option<Pixel>,
    direction: Pixel,
}

impl<'a> Iterator for Range2DIter {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.as_ref()?.clone();

        // find next;
        if &current == &self.end {
            self.next = None;
        } else {
            self.next = Some(current.clone() + self.direction.clone())
        };

        Some(current)
    }
}

pub struct InputIter<'a> {
    lines: Box<dyn Iterator<Item = Cow<'a, str>> + 'a>,
}

impl<'a> InputIter<'a> {
    pub fn new(input: &'a str) -> InputIter<'a> {
        InputIter {
            lines: Box::new(input.lines().map(Cow::from)),
        }
    }

    pub fn from_file(file_path: &str) -> Result<InputIter<'a>, std::io::Error> {
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        Ok(InputIter {
            lines: Box::new(
                reader
                    .lines()
                    .map(|x| Cow::from(x.expect("cannot read file"))),
            ),
        })
    }
}

impl<'a> Iterator for InputIter<'a> {
    type Item = Result<Range2D, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.lines.next() {
            Some(Range2D::from_str(&line))
        } else {
            None
        }
    }
}

#[test]
fn test_parse() {
    let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
"#;

    assert_eq!(
        vec![
            Range2D::from(((0, 9), (5, 9))),
            Range2D::from(((8, 0), (0, 8))),
            Range2D::from(((9, 4), (3, 4))),
        ],
        InputIter::new(input)
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
    )
}

#[test]
fn range_test_1() {
    let range = Range2D::from(((9, 9), (5, 9)));

    let mut points = Vec::new();
    for item in range {
        points.push(item)
    }

    assert_eq!(points.len(), 5)
}

#[test]
fn range_test_2() {
    let range = Range2D::from(((2, 9), (5, 9)));

    let mut points = Vec::new();
    for item in range {
        points.push(item)
    }

    assert_eq!(points.len(), 4)
}

#[test]
fn range_test_3() {
    let range = Range2D::from(((4, 9), (4, 3)));

    let mut points = Vec::new();
    for item in range {
        points.push(item)
    }

    assert_eq!(points.len(), 7)
}
