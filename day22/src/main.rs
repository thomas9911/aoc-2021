use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::read_to_string;
use std::ops::{Bound, RangeBounds, RangeInclusive};
use std::path::Path;

pub type Range = RangeInclusive<i32>;
pub type Triple = (i32, i32, i32);

fn bound_to_options(b: Bound<&i32>) -> Option<&i32> {
    match b {
        Bound::Included(x) => Some(x),
        Bound::Excluded(x) => Some(x),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct ReactorCore {
    blocks: Vec<Block>,
}

impl ReactorCore {
    pub fn from_text(input: &str) -> Result<ReactorCore, Box<dyn std::error::Error>> {
        let blocks = input
            .trim()
            .lines()
            .map(Block::from_text)
            .collect::<Result<_, Box<dyn std::error::Error>>>()?;
        Ok(ReactorCore { blocks })
    }

    pub fn new(blocks: Vec<Block>) -> ReactorCore {
        ReactorCore { blocks }
    }

    pub fn on(&self, triple: &Triple) -> bool {
        let mut point = false;
        for block in self.blocks.iter() {
            if let Some(found) = block.found(triple) {
                point = found;
            }
        }

        point
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    range_x: Range,
    range_y: Range,
    range_z: Range,
    is_on: bool,
}

impl Block {
    pub fn from_text(input: &str) -> Result<Block, Box<dyn std::error::Error>> {
        let input = input.trim_end();
        let (input, is_on) = if let Some(input) = input.strip_prefix("on ") {
            (input, true)
        } else {
            (input.strip_prefix("off ").ok_or("invalid line")?, false)
        };

        let (x_range_txt, y_range_txt, z_range_txt) = if let Some(Some((x, y, z))) = input
            .split_once(",")
            .map(|(x, rest)| rest.split_once(",").map(|(y, z)| (x, y, z)))
        {
            let x = x.trim_start_matches("x=");
            let y = y.trim_start_matches("y=");
            let z = z.trim_start_matches("z=");
            (x, y, z)
        } else {
            return Err("invalid input".into());
        };

        let x_range = Self::range_txt_to_range(x_range_txt)?;
        let y_range = Self::range_txt_to_range(y_range_txt)?;
        let z_range = Self::range_txt_to_range(z_range_txt)?;
        Ok(Block::new(x_range, y_range, z_range, is_on))
    }

    pub fn new(range_x: Range, range_y: Range, range_z: Range, is_on: bool) -> Block {
        Block {
            range_x,
            range_y,
            range_z,
            is_on,
        }
    }

    pub fn on(&self, (x, y, z): &Triple) -> bool {
        if self.range_x.contains(&x) && self.range_y.contains(&y) && self.range_z.contains(&z) {
            self.is_on
        } else {
            false
        }
    }

    pub fn found(&self, (x, y, z): &Triple) -> Option<bool> {
        if self.range_x.contains(&x) && self.range_y.contains(&y) && self.range_z.contains(&z) {
            Some(self.is_on)
        } else {
            None
        }
    }

    fn range_txt_to_range(input: &str) -> Result<Range, Box<dyn std::error::Error>> {
        if let Some(range) = input.split_once("..").map(
            |(start, end)| -> Result<Range, Box<dyn std::error::Error>> {
                let start: i32 = start.parse()?;
                let end: i32 = end.parse()?;
                Ok(start..=end)
            },
        ) {
            range
        } else {
            Err("invalid range".into())
        }
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day22/src/input.txt"
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let mut core = ReactorCore::from_text(&data)?;

    // remove blocks we dont care about
    core.blocks.retain(|block| {
        let upper_bound = 50;
        let lower_bound = -50;
        if (bound_to_options(block.range_x.end_bound()).unwrap() < &lower_bound)
            || (bound_to_options(block.range_y.end_bound()).unwrap() < &lower_bound)
            || (bound_to_options(block.range_z.end_bound()).unwrap() < &lower_bound)
        {
            return false;
        }

        if (bound_to_options(block.range_x.start_bound()).unwrap() > &upper_bound)
            || (bound_to_options(block.range_y.start_bound()).unwrap() > &upper_bound)
            || (bound_to_options(block.range_z.start_bound()).unwrap() > &upper_bound)
        {
            return false;
        }

        true
    });

    // this can perfectly be done in parallel
    let range3d = (-50..=50).into_par_iter().flat_map(|i| {
        (-50..=50)
            .into_par_iter()
            .flat_map(move |j| (-50..=50).into_par_iter().map(move |k| (i, j, k)))
    });
    let counter = range3d.filter(|x| core.on(x)).count();

    Ok(counter)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let _core = ReactorCore::from_text(&data)?;

    Ok(usize::MAX)
}

#[test]
fn day_22_part_one() {
    assert_eq!(658691, part_one(fetch_file_path()).unwrap())
}

#[test]
fn reactor_core_from_text() {
    let expected = ReactorCore::new(vec![
        Block::new(10..=12, 10..=12, 10..=12, true),
        Block::new(11..=13, 11..=13, 11..=13, true),
        Block::new(9..=11, 9..=11, 9..=11, false),
        Block::new(10..=10, 10..=10, 10..=10, true),
    ]);

    let input = "
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
";
    let out = ReactorCore::from_text(input).unwrap();

    assert_eq!(expected, out)
}

#[test]
fn reactor_core_on_test() {
    let core = ReactorCore::new(vec![
        Block::new(10..=12, 10..=12, 10..=12, true),
        Block::new(11..=13, 11..=13, 11..=13, true),
        Block::new(9..=11, 9..=11, 9..=11, false),
        Block::new(10..=10, 10..=10, 10..=10, true),
    ]);

    // turned on
    assert!(core.on(&(13, 12, 13)));
    assert!(core.on(&(13, 12, 11)));

    // last turned on
    assert!(core.on(&(10, 10, 10)));
    // turned off
    assert!(!core.on(&(9, 9, 9)));

    // never set
    assert!(!core.on(&(8, 9, 9)));
    assert!(!core.on(&(0, 0, 0)));
    assert!(!core.on(&(2, -9, -4)));
}

#[test]
fn block_on_test() {
    let always_false = Block::new(9..=11, 9..=11, 9..=11, false);
    assert!(!always_false.on(&(9, 9, 9)));
    assert!(!always_false.on(&(18, 9, 9)));

    let normal_block = Block::new(9..=11, 9..=11, 9..=11, true);
    assert!(normal_block.on(&(9, 10, 9)));
    assert!(normal_block.on(&(9, 9, 11)));
    assert!(!normal_block.on(&(18, 9, 9)));
}
