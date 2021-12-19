use std::fs::read_to_string;
use std::ops::{Bound, RangeBounds, RangeInclusive};
use std::path::Path;

use derive_more::{Deref, DerefMut, From};

pub type Range = RangeInclusive<i32>;
pub type Pair = (i32, i32);

fn bound_to_options(b: Bound<&i32>) -> Option<&i32> {
    match b {
        Bound::Included(x) => Some(x),
        Bound::Excluded(x) => Some(x),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct Area {
    range_x: Range,
    range_y: Range,
}

impl Area {
    pub fn from_txt(input: &str) -> Result<Area, Box<dyn std::error::Error>> {
        let (x_range_txt, y_range_txt) = if let Some((x, y)) = input
            .trim_start_matches("target area: ")
            .trim_end()
            .split_once(", ")
        {
            let x = x.trim_start_matches("x=");
            let y = y.trim_start_matches("y=");
            (x, y)
        } else {
            return Err("invalid input".into());
        };

        let x_range = Self::range_txt_to_range(x_range_txt)?;
        let y_range = Self::range_txt_to_range(y_range_txt)?;
        Ok(Area::new(x_range, y_range))
    }

    pub fn new(range_x: Range, range_y: Range) -> Area {
        Area { range_x, range_y }
    }

    // on target
    pub fn on(&self, pair: &Pair) -> bool {
        self.range_x.contains(&pair.0) && self.range_y.contains(&pair.1)
    }

    /// area under and after the 'area'
    pub fn through(&self, pair: &Pair) -> bool {
        &pair.0 >= bound_to_options(self.range_x.start_bound()).unwrap()
            && &pair.1 <= bound_to_options(self.range_y.end_bound()).unwrap()
            && !self.on(pair)
    }

    /// area up and before the 'area'
    pub fn before(&self, pair: &Pair) -> bool {
        &pair.0 <= bound_to_options(self.range_x.end_bound()).unwrap()
            && &pair.1 >= bound_to_options(self.range_y.start_bound()).unwrap()
            && !self.on(pair)
    }

    /// area under and/or before, and after and/or up the 'area'
    pub fn out(&self, pair: &Pair) -> bool {
        &pair.0 > bound_to_options(self.range_x.end_bound()).unwrap()
            || &pair.1 < bound_to_options(self.range_y.start_bound()).unwrap()
    }

    pub fn get_target_route(&self, acceleration: Pair) -> Option<Vec<Pair>> {
        let mut route = Vec::new();
        for next_probe in Probe::new(acceleration) {
            if self.out(&next_probe.position) {
                return None;
            }

            if self.on(&next_probe.position) {
                route.push(next_probe.position);
                return Some(route);
            }
            route.push(next_probe.position);
        }

        None
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

#[derive(Debug, PartialEq, Clone, Deref, DerefMut, From)]
pub struct Probe {
    #[deref]
    #[deref_mut]
    pub position: Pair,
    pub acceleration: Pair,
    done: bool,
}

impl Probe {
    pub fn new(acceleration: Pair) -> Probe {
        Probe {
            position: (0, 0),
            acceleration,
            done: false,
        }
    }
}

impl Iterator for Probe {
    type Item = Probe;

    fn next<'a>(&'a mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        };

        let (mut current_x_acc, mut current_y_acc) = self.acceleration;
        // if current_x_acc == 0 && current_y_acc <= 0 {
        //     self.done = true;
        //     return None;
        // };

        self.position.0 += current_x_acc;
        self.position.1 += current_y_acc;

        current_y_acc -= 1;
        if current_x_acc > 0 {
            current_x_acc -= 1;
        };
        if current_x_acc < 0 {
            current_x_acc += 1;
        };

        self.acceleration = (current_x_acc, current_y_acc);

        Some(self.clone())
    }
}

fn bound_to_search_range(bound: Bound<&i32>) -> i32 {
    bound_to_options(bound).unwrap().abs() + 1
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day17/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let area = Area::from_txt(&data)?;

    let mut routes = Vec::new();

    let a = bound_to_search_range(area.range_x.end_bound());
    let b = bound_to_search_range(area.range_y.start_bound());

    for (i, j) in (0..a).flat_map(|x| (0..b).map(move |y| (x, y))) {
        if let Some(route) = area.get_target_route((i, j)) {
            routes.push(route)
        }
    }

    let max_route = routes
        .iter()
        .max_by_key(|x| x.iter().max_by_key(|y| y.1))
        .expect("routes not found");
    let max_y_position = max_route
        .iter()
        .max_by_key(|y| y.1)
        .expect("max route is empty");
    let max_y = max_y_position.1;

    Ok(max_y)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let area = Area::from_txt(&data)?;

    let mut routes = Vec::new();

    let a = bound_to_search_range(area.range_x.end_bound());
    let b = bound_to_search_range(area.range_y.start_bound());

    for (i, j) in (-a..a).flat_map(|x| (-b..b).map(move |y| (x, y))) {
        if let Some(route) = area.get_target_route((i, j)) {
            routes.push(route)
        }
    }

    Ok(routes.len())
}

#[cfg(test)]
mod area_test {
    use super::Area;

    #[test]
    fn on() {
        let area = Area::new(0..=3, 8..=11);

        assert!(area.on(&(1, 11)));
        assert!(area.on(&(0, 8)));
        assert!(!area.on(&(4, 8)));
        assert!(!area.on(&(2, 19)));
    }

    #[test]
    fn through() {
        let area = Area::new(0..=3, 8..=11);

        assert!(!area.through(&(1, 11)));
        assert!(!area.through(&(0, 8)));
        assert!(!area.through(&(3, 12)));
        assert!(area.through(&(4, 8)));
        assert!(area.through(&(2, 7)));
        assert!(area.through(&(15, 4)));
    }

    #[test]
    fn before() {
        let area = Area::new(0..=3, 8..=11);

        assert!(!area.before(&(1, 11)));
        assert!(!area.before(&(0, 8)));
        assert!(area.before(&(3, 13)));
        assert!(area.before(&(-4, 8)));
        assert!(area.before(&(-2, 19)));
        assert!(!area.before(&(15, 19)));
    }

    #[test]
    fn get_target_route_on_target() {
        let area = Area::new(20..=30, -10..=-5);

        assert!(area.get_target_route((7, 2)).is_some());
        assert!(area.get_target_route((6, 3)).is_some());
        assert!(area.get_target_route((9, 0)).is_some());
        assert!(area.get_target_route((17, -4)).is_none());
    }

    #[test]
    fn get_target_route() {
        let area = Area::new(20..=30, -10..=-5);

        assert_eq!(
            Some(vec![
                (7, 2),
                (13, 3),
                (18, 3),
                (22, 2),
                (25, 0),
                (27, -3),
                (28, -7)
            ]),
            area.get_target_route((7, 2))
        );
    }
}

#[test]
fn day17_part_one() {
    assert_eq!(2850, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day17_part_two() {
    assert_eq!(1117, part_two(fetch_file_path()).unwrap())
}
