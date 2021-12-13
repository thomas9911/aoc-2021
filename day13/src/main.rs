use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
pub struct Sheet {
    max_x: usize,
    max_y: usize,
    points: Vec<Point>,
}

pub type Point = (usize, usize);
pub type Instruction = (Axis, usize);

#[derive(Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
}

impl Sheet {
    pub fn from_points(points: Vec<Point>) -> Sheet {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in points.iter() {
            if x > &max_x {
                max_x = *x
            }

            if y > &max_y {
                max_y = *y
            }
        }

        Sheet {
            points,
            max_x,
            max_y,
        }
    }

    fn init_drawing(&self) -> Vec<Vec<bool>> {
        (0..=self.max_y)
            .into_iter()
            .map(|_| (0..=self.max_x).into_iter().map(|_| false).collect())
            .collect()
    }

    pub fn mark(&self) -> Vec<Vec<bool>> {
        let mut out = self.init_drawing();

        for (x, y) in self.points.iter() {
            out[*y][*x] = true
        }

        out
    }

    pub fn draw(&self) -> String {
        let mut buffer = String::with_capacity(self.max_x * self.max_y);
        for line in self.mark() {
            for dot in line {
                if dot {
                    buffer.push('█');
                } else {
                    buffer.push(' ');
                }
            }
            buffer.push('\n');
        }

        buffer
    }

    pub fn fold_y(&mut self, at: usize) -> Option<()> {
        let mut max = self.max_y;
        let res = self.inner_fold(&mut max, Sheet::inner_fold_y, at);
        self.max_y = max;
        res
    }

    pub fn fold_x(&mut self, at: usize) -> Option<()> {
        let mut max = self.max_x;
        let res = self.inner_fold(&mut max, Sheet::inner_fold_x, at);
        self.max_x = max;
        res
    }

    pub fn visible_points(&self) -> usize {
        self.points.len()
    }

    fn inner_fold<F: Fn(&mut Point, &usize)>(
        &mut self,
        max: &mut usize,
        func: F,
        at: usize,
    ) -> Option<()> {
        if *max < at {
            return None;
        }
        self.points.iter_mut().for_each(|point| func(point, &at));
        self.points.sort_unstable();
        self.points.dedup();
        *max = at - 1;
        Some(())
    }

    fn inner_fold_y((_, y): &mut Point, at: &usize) {
        if &*y > &at {
            *y = at - (*y - at);
        }
    }

    fn inner_fold_x((x, _): &mut Point, at: &usize) {
        if &*x > &at {
            *x = at - (*x - at);
        }
    }
}

fn parse_points(input: &str) -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(|x| match x.split_once(',') {
            Some((x, y)) => Ok((x.parse()?, y.parse()?)),
            None => Err(String::from("invalid line").into()),
        })
        .collect()
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(
            |x| match x.trim_start_matches("fold along ").split_once('=') {
                Some(("x", right)) => Ok((Axis::X, right.parse()?)),
                Some(("y", right)) => Ok((Axis::Y, right.parse()?)),
                _ => Err(String::from("invalid line").into()),
            },
        )
        .collect()
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day13/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two:\n{}", part_two(input_file)?);
    Ok(())
}

fn preprocess(input_file: &str) -> Result<(Sheet, Vec<Instruction>), Box<dyn std::error::Error>> {
    let input = read_to_string(input_file)?
        .lines()
        .fold(String::new(), |mut acc, x| {
            acc.push_str(x);
            acc.push('\n');
            acc
        });

    let (points_data, instructions) = input
        .split_once("\n\n")
        .ok_or(String::from("invalid input data"))?;
    let points = parse_points(points_data)?;
    let instructions = parse_instructions(instructions)?;
    let sheet = Sheet::from_points(points);
    Ok((sheet, instructions))
}

fn part_one(input_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (mut sheet, instructions) = preprocess(input_file)?;
    for (axis, at) in instructions.into_iter().take(1) {
        match axis {
            Axis::X => sheet.fold_x(at),
            Axis::Y => sheet.fold_y(at),
        };
    }

    Ok(sheet.visible_points())
}

fn part_two(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut sheet, instructions) = preprocess(input_file)?;
    for (axis, at) in instructions {
        match axis {
            Axis::X => sheet.fold_x(at),
            Axis::Y => sheet.fold_y(at),
        };
    }

    Ok(sheet.draw())
}

#[test]
fn day13_one() {
    assert_eq!(747, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day13_two() {
    let expected = r#"
 ██  ███  █  █ ████ ███   ██  █  █ █  █ 
█  █ █  █ █  █    █ █  █ █  █ █  █ █  █ 
█  █ █  █ ████   █  █  █ █    █  █ ████ 
████ ███  █  █  █   ███  █    █  █ █  █ 
█  █ █ █  █  █ █    █    █  █ █  █ █  █ 
█  █ █  █ █  █ ████ █     ██   ██  █  █ 
"#;
    assert_eq!(
        expected.trim_start_matches("\n"),
        part_two(fetch_file_path()).unwrap()
    )
}
