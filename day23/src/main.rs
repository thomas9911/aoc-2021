use maplit::{btreemap, btreeset};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, strum::Display, strum::EnumString, strum::IntoStaticStr)]
pub enum Amphipod {
    #[strum(to_string = "A")]
    Amber = 1,
    #[strum(to_string = "B")]
    Bronze = 10,
    #[strum(to_string = "C")]
    Copper = 100,
    #[strum(to_string = "D")]
    Desert = 1000,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Interior {
    Room(Amphipod),
    Hall,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pub spots: BTreeMap<(usize, usize), Option<Amphipod>>,
    pub connections: BTreeMap<(usize, usize), BTreeSet<Direction>>,
    pub interior: BTreeMap<(usize, usize), Interior>,
}

impl Default for Board {
    /// ...........
    ///   x x x x
    ///   x x x x
    fn default() -> Board {
        use Direction::*;
        use Interior::*;

        Board {
            spots: btreemap! {
                (0,0) => None,
                (0,1) => None,
                (0,2) => None,
                (0,3) => None,
                (0,4) => None,
                (0,5) => None,
                (0,6) => None,
                (0,7) => None,
                (0,8) => None,
                (0,9) => None,
                (0,10) => None,
                (1,2) => None,
                (2,2) => None,
                (1,4) => None,
                (2,4) => None,
                (1,6) => None,
                (2,6) => None,
                (1,8) => None,
                (2,8) => None,
            },
            connections: btreemap! {
                (0,0) => btreeset!{Right},
                (0,1) => btreeset!{Left, Right},
                (0,2) => btreeset!{Left, Right, Down},
                (0,3) => btreeset!{Left, Right},
                (0,4) => btreeset!{Left, Right, Down},
                (0,5) => btreeset!{Left, Right},
                (0,6) => btreeset!{Left, Right, Down},
                (0,7) => btreeset!{Left, Right},
                (0,8) => btreeset!{Left, Right, Down},
                (0,9) => btreeset!{Left, Right},
                (0,10) => btreeset!{Left},
                (1,2) => btreeset!{Down, Up},
                (2,2) =>  btreeset!{Up},
                (1,4) => btreeset!{Down, Up},
                (2,4) => btreeset!{Up},
                (1,6) => btreeset!{Down, Up},
                (2,6) => btreeset!{Up},
                (1,8) => btreeset!{Down, Up},
                (2,8) => btreeset!{Up},
            },
            interior: btreemap! {
                (0,0) => Hall,
                (0,1) => Hall,
                (0,2) => Hall,
                (0,3) => Hall,
                (0,4) => Hall,
                (0,5) => Hall,
                (0,6) => Hall,
                (0,7) => Hall,
                (0,8) => Hall,
                (0,9) => Hall,
                (0,10) => Hall,
                (1,2) => Room(Amphipod::Amber),
                (2,2) => Room(Amphipod::Amber),
                (1,4) => Room(Amphipod::Bronze),
                (2,4) => Room(Amphipod::Bronze),
                (1,6) => Room(Amphipod::Copper),
                (2,6) => Room(Amphipod::Copper),
                (1,8) => Room(Amphipod::Desert),
                (2,8) => Room(Amphipod::Desert),
            },
        }
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day23/src/input.txt"
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    // println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let mut board = Board::default();

    for (i, line) in data.lines().skip(2).take(2).enumerate() {
        for (j, x) in line
            .chars()
            .filter(|ch| ch != &'#' && ch != &' ')
            .map(|ch| ch.to_string())
            .map(|s| s.parse())
            .enumerate()
        {
            let key = board
                .spots
                .get_mut(&(i + 1, 2 * j + 2))
                .expect("the board is invalid");
            *key = Some(x?);
        }
    }

    dbg!(board);

    Ok(0)
}

// fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
//     let data = read_to_string(input_path)?;

//     Ok(0)
// }

// #[test]
// fn day_23_part_one() {
//     assert_eq!(658691, part_one(fetch_file_path(), false).unwrap())
// }

// #[test]
// fn day_23_part_two() {
//     assert_eq!(1228699515783640, part_two(fetch_file_path()).unwrap())
// }

