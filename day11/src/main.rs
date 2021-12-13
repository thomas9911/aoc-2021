use derive_more::{Deref, DerefMut, From};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day11/src/input.txt"
    }
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum Octopus {
    Charging(u8),
    Charged,
}

impl Default for Octopus {
    fn default() -> Self {
        Octopus::Charging(0)
    }
}

impl Octopus {
    pub fn as_option_mut(&mut self) -> Option<&mut u8> {
        use Octopus::*;
        match self {
            Charging(x) => Some(x),
            Charged => None,
        }
    }
}

impl TryFrom<char> for Octopus {
    type Error = String;

    fn try_from(ch: char) -> Result<Octopus, Self::Error> {
        Ok(Octopus::from(
            ch.to_digit(10).ok_or(String::from("invalid digit"))? as u8,
        ))
    }
}

#[derive(Debug, Default, Deref, DerefMut, Clone)]
pub struct Grid {
    #[deref]
    #[deref_mut]
    data: [[Octopus; 10]; 10],
    flashes: usize,
}

const MAX_CELL_AMOUNT: u8 = 9;

impl Grid {
    pub fn from_bufreader(buffer: BufReader<File>) -> Result<Grid, Box<dyn std::error::Error>> {
        let mut grid = Grid::default();
        for (i, line) in buffer.lines().enumerate() {
            for (j, ch) in line?.chars().enumerate() {
                grid.data[i][j] = Octopus::try_from(ch)?;
            }
        }

        Ok(grid)
    }

    pub fn step(&mut self) -> bool {
        self.cells_plus_one();
        self.cells_update();
        self.cap_and_count_flashes()
    }

    fn cells_plus_one(&mut self) {
        self.iter_mut().for_each(|x| {
            x.iter_mut().for_each(|y| match y {
                Octopus::Charging(z) => *z += 1,
                Octopus::Charged => (),
            })
        })
    }

    fn cells_update(&mut self) {
        let mut new_grid = self.clone();

        for (i, line) in self.data.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                if let Octopus::Charging(x) = cell {
                    if x > &MAX_CELL_AMOUNT {
                        new_grid.update_surrounding_cells(i, j);
                    }
                }
            }
        }

        self.data = new_grid.data;
    }

    fn update_surrounding_cells(&mut self, x: usize, y: usize) {
        let current = self.get_mut(x, y).expect("this cell exists");
        if *current == Octopus::Charged {
            return;
        }
        *current = Octopus::Charged;

        self.update_cell(x.checked_sub(1), y.checked_sub(1));
        self.update_cell(Some(x), y.checked_sub(1));
        self.update_cell(x.checked_add(1), y.checked_sub(1));

        self.update_cell(x.checked_sub(1), Some(y));
        self.update_cell(x.checked_add(1), Some(y));

        self.update_cell(x.checked_sub(1), y.checked_add(1));
        self.update_cell(Some(x), y.checked_add(1));
        self.update_cell(x.checked_add(1), y.checked_add(1));
    }

    fn update_cell(&mut self, x: Option<usize>, y: Option<usize>) -> Option<()> {
        let x = x?;
        let y = y?;

        if let Some(cell) = self.get_mut(x, y) {
            match cell {
                Octopus::Charging(amount) if *amount >= MAX_CELL_AMOUNT => {
                    self.update_surrounding_cells(x, y);
                }
                Octopus::Charging(amount) => *amount += 1,
                Octopus::Charged => (),
            }
        };

        Some(())
    }

    fn cap_and_count_flashes(&mut self) -> bool {
        let mut flashes_count = 0;
        self.iter_mut().for_each(|x| {
            x.iter_mut().for_each(|y| {
                if *y == Octopus::Charged {
                    flashes_count += 1;
                    *y = Octopus::Charging(0);
                }
            })
        });
        self.flashes += flashes_count;
        if flashes_count == 100 {
            return true;
        } else {
            false
        }
    }

    pub fn print(&self, sep: char) -> String {
        let mut out = String::new();

        for line in self.data.iter() {
            out.push(sep);
            for cell in line {
                match &cell {
                    Octopus::Charging(cell) => out.push_str(&cell.to_string()),
                    Octopus::Charged => out.push('x'),
                }
                out.push(sep);
            }
            out.push('\n')
        }

        out
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Octopus> {
        self.data.get_mut(x).map(|row| row.get_mut(y)).flatten()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Octopus> {
        self.data.get(x).map(|row| row.get(y)).flatten()
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
    let mut grid = Grid::from_bufreader(buffer)?;

    for _ in 0..100 {
        grid.step();
    }

    Ok(grid.flashes)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);
    let mut grid = Grid::from_bufreader(buffer)?;

    let mut round = 1;
    while !grid.step() {
        round += 1;
    }

    Ok(round)
}

#[test]
fn day11_part_one() {
    assert_eq!(1757, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day11_part_two() {
    assert_eq!(422, part_two(fetch_file_path()).unwrap())
}
