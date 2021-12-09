use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::collections::BTreeMap;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day9/src/input.txt"
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
    let map = HeightMap::from_bufreader(buffer)?;

    let minimal_points = map.find_minimum_points();
    let score = minimal_points.values().map(|x| (x + 1) as usize).sum();

    Ok(score)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);
    let map = BasinMap::from_bufreader(buffer)?;

    let mut basin_sizes = map.find_chunks();
    basin_sizes.sort_unstable();

    let a = basin_sizes.pop().ok_or("zero basins found")?;
    let b = basin_sizes.pop().ok_or("only one basins found")?;
    let c = basin_sizes.pop().ok_or("only two basins found")?;

    Ok(a * b * c)
}

#[derive(Debug)]
pub struct HeightMap {
    data: Vec<Vec<u8>>,
}

impl HeightMap {
    pub fn from_bufreader(input: BufReader<File>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut data = Vec::new();

        for line in input.lines() {
            let mut line_data = Vec::new();
            for char in line?.chars() {
                line_data.push(char.to_digit(10).ok_or(String::from("invalid digit"))? as u8)
            }
            data.push(line_data)
        }

        Ok(HeightMap { data })
    }

    pub fn find_minimum_points(&self) -> BTreeMap<(usize, usize), u8> {
        let mut points = BTreeMap::new();

        for (i, row) in self.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let a = self.fetch_point(i.checked_sub(1), Some(j));
                let b = self.fetch_point(Some(i), j.checked_sub(1));
                let c = self.fetch_point(i.checked_add(1), Some(j));
                let d = self.fetch_point(Some(i), j.checked_add(1));

                let min = [a, b, c, d]
                    .into_iter()
                    .filter_map(|x| x)
                    .min()
                    .expect("minimum cannot be found");
                if min > cell {
                    points.insert((i, j), *cell);
                }
            }
        }

        points
    }

    fn fetch_point<'a>(&'a self, x: Option<usize>, y: Option<usize>) -> Option<&'a u8> {
        let x = x?;
        let y = y?;
        self.data.get(x).map(|rows| rows.get(y)).flatten()
    }
}

#[derive(Debug)]
pub struct BasinMap {
    data: Vec<Vec<bool>>,
}

impl BasinMap {
    pub fn from_bufreader(input: BufReader<File>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut data = Vec::new();

        for line in input.lines() {
            let mut line_data = Vec::new();
            for char in line?.chars() {
                line_data.push(char == '9')
            }
            data.push(line_data)
        }

        Ok(BasinMap { data })
    }

    pub fn find_chunks(mut self) -> Vec<usize> {
        // we will change the entire self.data lets just move it here.

        let mut chunks = Vec::new();

        while let Some((x, y)) = self.find_next_basin_start() {
            let amount_of_cells = self.spill(x, y);
            chunks.push(amount_of_cells);
        }

        chunks
    }

    fn find_next_basin_start(&self) -> Option<(usize, usize)> {
        self.data.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, cell)| if cell == &false { Some(j) } else { None })
                .map(|j| (i, j))
        })
    }

    fn spill(&mut self, start_x: usize, start_y: usize) -> usize {
        let mut counter: usize = 0;

        for (point_x, point_y) in [
            (start_x.checked_sub(1), Some(start_y)),
            (Some(start_x), start_y.checked_sub(1)),
            (start_x.checked_add(1), Some(start_y)),
            (Some(start_x), start_y.checked_add(1)),
        ] {
            if let Some(score) = self
                .fetch_point(point_x, point_y)
                .map(|c| self.spill_recursive(c))
            {
                counter += score
            }
        }

        counter
    }

    fn spill_recursive(&mut self, coordinates: (usize, usize)) -> usize {
        let x = coordinates.0;
        let y = coordinates.1;
        self.data[x][y] = true;
        self.spill(x, y) + 1
    }

    fn fetch_point<'a>(&'a self, x: Option<usize>, y: Option<usize>) -> Option<(usize, usize)> {
        let x = x?;
        let y = y?;
        let cell = self.data.get(x).map(|rows| rows.get(y)).flatten()?;
        if cell == &false {
            Some((x, y))
        } else {
            None
        }
    }
}

#[test]
fn day9_part_one() {
    assert_eq!(633, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day9_part_two() {
    assert_eq!(1050192, part_two(fetch_file_path()).unwrap())
}
