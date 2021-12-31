use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub enum Direction {
    East,
    South,
}

impl Direction {
    pub fn from_char(ch: char) -> Result<Option<Direction>, &'static str> {
        match ch {
            '>' => Ok(Some(Direction::East)),
            'v' => Ok(Some(Direction::South)),
            '.' => Ok(None),
            _ => Err("invalid char"),
        }
    }

    pub fn is_east(&self) -> bool {
        match self {
            Direction::East => true,
            _ => false,
        }
    }

    pub fn is_south(&self) -> bool {
        match self {
            Direction::South => true,
            _ => false,
        }
    }

    pub fn check_direction(&self, east: bool) -> bool {
        if east {
            self.is_east()
        } else {
            self.is_south()
        }
    }

    pub fn next_position(
        &self,
        coordinate: &(usize, usize),
        width: usize,
        height: usize,
    ) -> (usize, usize) {
        match self {
            Direction::South => ((coordinate.0 + 1) % height, coordinate.1),
            Direction::East => (coordinate.0, (coordinate.1 + 1) % width),
        }
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    // because im cool, the first is the height and second is the width coordinate
    field: BTreeMap<(usize, usize), Direction>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Grid, Box<dyn std::error::Error>> {
        let mut grid = Grid::default();

        let mut width = None;
        let mut height = None;
        for (i, line) in reader.lines().enumerate() {
            for (j, ch) in line?.chars().enumerate() {
                if let Some(sea_cucumber) = Direction::from_char(ch)? {
                    grid.field.insert((i, j), sea_cucumber);
                }
                width = Some(j)
            }
            height = Some(i)
        }

        grid.width = width.map(|x| x + 1).unwrap_or(0);
        grid.height = height.map(|x| x + 1).unwrap_or(0);
        Ok(grid)
    }

    pub fn get(&self, coordinate: &(usize, usize)) -> Option<&Direction> {
        self.field.get(coordinate)
    }

    pub fn update_step(&mut self, mutations: &[((usize, usize), (usize, usize))]) {
        for (from, to) in mutations {
            if let Some(direction) = self.field.remove(from) {
                self.field.insert(to.clone(), direction);
            }
        }
    }

    pub fn move_south_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), (usize, usize))> + 'a> {
        self.move_iter_factory(Direction::South)
    }

    pub fn move_east_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), (usize, usize))> + 'a> {
        self.move_iter_factory(Direction::East)
    }

    pub fn move_iter_factory<'a>(
        &'a self,
        direction: Direction,
    ) -> Box<dyn Iterator<Item = ((usize, usize), (usize, usize))> + 'a> {
        let iter = self
            .iterator_factory(direction.is_east())
            .filter_map(move |pos| {
                let next_pos = direction.next_position(pos, self.width, self.height);
                if self.get(&next_pos).is_none() {
                    Some((pos.clone(), next_pos))
                } else {
                    None
                }
            });

        Box::new(iter)
    }

    pub fn east_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &(usize, usize)> + 'a> {
        self.iterator_factory(true)
    }

    pub fn south_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &(usize, usize)> + 'a> {
        self.iterator_factory(false)
    }

    fn iterator_factory<'a>(
        &'a self,
        east: bool,
    ) -> Box<dyn Iterator<Item = &(usize, usize)> + 'a> {
        let iter = self.field.iter().filter_map(move |(k, v)| {
            if v.check_direction(east) {
                Some(k)
            } else {
                None
            }
        });

        Box::new(iter)
    }

    pub fn print(&self) -> String {
        let mut buffer = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let ch = match self.get(&(i, j)) {
                    Some(Direction::South) => 'v',
                    Some(Direction::East) => '>',
                    None => '.',
                };
                buffer.push(ch)
            }
            buffer.push('\n')
        }
        buffer
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day25/src/input.txt"
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut grid = Grid::from_reader(reader)?;

    let mut counter = 0;
    loop {
        counter += 1;
        let mut no_movement;
        let positions: Vec<_> = grid.move_east_iter().collect();
        grid.update_step(&positions);
        no_movement = positions.is_empty();

        let positions: Vec<_> = grid.move_south_iter().collect();
        grid.update_step(&positions);
        no_movement &= positions.is_empty();

        if no_movement {
            break;
        }
    }

    Ok(counter)
}

// only fast enough if you run in release mode
#[cfg(not(debug_assertions))]
#[test]
fn day_25_part_one() {
    assert_eq!(321, part_one(fetch_file_path()).unwrap())
}

#[cfg(test)]
mod grid_test {
    use super::Grid;
    use std::io::{BufReader, Cursor};

    #[test]
    fn print() {
        let input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
        let grid = Grid::from_reader(BufReader::new(Cursor::new(input))).unwrap();

        assert_eq!(grid.print(), input);
    }
}
