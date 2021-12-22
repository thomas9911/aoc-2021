use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::path::Path;

pub type Range = RangeInclusive<i32>;
pub type Triple = (i32, i32, i32);

#[derive(Debug, PartialEq)]
pub struct ReactorCore {
    blocks: Vec<Block>,
    is_combined: bool,
}

impl ReactorCore {
    pub fn from_text(input: &str) -> Result<ReactorCore, Box<dyn std::error::Error>> {
        let blocks = input
            .trim()
            .lines()
            .map(Block::from_text)
            .collect::<Result<_, Box<dyn std::error::Error>>>()?;
        Ok(ReactorCore {
            blocks,
            is_combined: false,
        })
    }

    pub fn new(blocks: Vec<Block>) -> ReactorCore {
        ReactorCore {
            blocks,
            is_combined: false,
        }
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

    pub fn reject_blocks_not_in_range(&mut self, range: Range) {
        let upper_bound = *range.end();
        let lower_bound = *range.start();

        self.blocks.retain(|block| {
            if (block.range_x.end() < &lower_bound)
                || (block.range_y.end() < &lower_bound)
                || (block.range_z.end() < &lower_bound)
            {
                return false;
            }

            if (block.range_x.start() > &upper_bound)
                || (block.range_y.start() > &upper_bound)
                || (block.range_z.start() > &upper_bound)
            {
                return false;
            }

            true
        });
    }

    pub fn combining(&mut self) {
        // current block on:
        // if on:
        //    add intersect off
        //    add block
        // if off:
        //    add intersect off

        // current block off:
        // if on:
        //    add block
        // if off:
        //    add block

        // reduced blocks contains blocks with on, and intersecting off blocks

        let mut reduced_blocks: Vec<Block> = Vec::new();

        for block in self.blocks.iter() {
            for i in 0..reduced_blocks.len() {
                let current_block = &reduced_blocks[i];
                match (current_block.is_on, block.is_on) {
                    (true, _) => {
                        // subtract intersection
                        reduced_blocks[i]
                            .intersect(block, false)
                            .map(|x| reduced_blocks.push(x));
                    }
                    (false, _) => {
                        // add intersection, to counter the other off block
                        reduced_blocks[i]
                            .intersect(block, true)
                            .map(|x| reduced_blocks.push(x));
                    }
                }
            }
            if block.is_on {
                reduced_blocks.push(block.clone())
            }
        }

        self.is_combined = true;
        self.blocks = reduced_blocks;
    }

    pub fn count_on_cubes(&self) -> usize {
        if !self.is_combined {
            return 0;
        }

        let mut count: i64 = 0;
        for block in self.blocks.iter() {
            if block.is_on {
                count += block.volume()
            } else {
                count -= block.volume()
            }
        }

        if count < 0 {
            panic!("invalid volume")
        }
        count as usize
    }
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn volume(&self) -> i64 {
        // why do I need to do +1 here ?!
        (self.range_x.end() - self.range_x.start() + 1) as i64
            * (self.range_y.end() - self.range_y.start() + 1) as i64
            * (self.range_z.end() - self.range_z.start() + 1) as i64
    }

    pub fn intersect(&self, other: &Block, is_on: bool) -> Option<Block> {
        let mut coordinates = [
            (&self.range_x, &other.range_x),
            (&self.range_y, &other.range_y),
            (&self.range_z, &other.range_z),
        ]
        .into_iter()
        .map(|(left, right)| {
            let left_start = left.start();
            let left_end = left.end();
            let right_start = right.start();
            let right_end = right.end();
            let new_start = left_start.max(right_start);
            let new_end = left_end.min(right_end);
            if new_start <= new_end {
                Some(*new_start..=*new_end)
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()?;

        let z = coordinates.remove(2);
        let y = coordinates.remove(1);
        let x = coordinates.remove(0);
        Some(Block::new(x, y, z, is_on))
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
    // finished but with too much help from reddit
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file, false)?);
    println!("part one combined: {:?}", part_one(input_file, true)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str, combined: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let mut core = ReactorCore::from_text(&data)?;

    if combined {
        core.combining();
    }

    // remove blocks we dont care about
    core.reject_blocks_not_in_range(-50..=50);

    // this can perfectly be done in parallel
    let range3d = (-50..=50).into_par_iter().flat_map(|i| {
        (-50..=50)
            .into_par_iter()
            .flat_map(move |j| (-50..=50).into_par_iter().map(move |k| (i, j, k)))
    });
    Ok(range3d.filter(|x| core.on(x)).count())
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let mut core = ReactorCore::from_text(&data)?;
    core.combining();

    let count = core.count_on_cubes();

    Ok(count)
}

#[test]
fn day_22_part_one() {
    assert_eq!(658691, part_one(fetch_file_path(), false).unwrap())
}

// only fast enough to get if you run in release mode
#[cfg(not(debug_assertions))]
#[test]
fn day_22_part_two() {
    assert_eq!(1228699515783640, part_two(fetch_file_path()).unwrap())
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

#[test]
fn intersect_test() {
    let a = Block::new(10..=12, 10..=12, 10..=12, true);
    let b = Block::new(11..=13, 11..=13, 11..=13, true);

    let x = a.intersect(&b, false).unwrap();

    assert_eq!(
        Block {
            range_x: 11..=12,
            range_y: 11..=12,
            range_z: 11..=12,
            is_on: false,
        },
        x
    )
}

#[test]
fn volume_test() {
    let a = Block::new(10..=12, 10..=13, 11..=14, true);
    let b = Block::new(-10..=-7, 10..=13, 11..=14, true);

    assert_eq!(a.volume(), 3 * 4 * 4);
    assert_eq!(b.volume(), 4 * 4 * 4);
}
