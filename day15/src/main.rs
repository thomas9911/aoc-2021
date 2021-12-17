use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use derive_more::{Deref, DerefMut};
use std::collections::BTreeMap;

pub mod naive_finder;

#[derive(Debug, Default, PartialEq, Deref, DerefMut)]
pub struct Map {
    data: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(data: Vec<Vec<u8>>) -> Map {
        Map { data }
    }

    pub fn from_bufreader<R: Read>(
        reader: BufReader<R>,
    ) -> Result<Map, Box<dyn std::error::Error>> {
        let mut data = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let mut row = Vec::with_capacity(line.len());
            for point in line.chars() {
                row.push(point.to_digit(10).ok_or("invalid digit")? as u8);
            }
            data.push(row)
        }
        Ok(Map::new(data))
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.data.get(x).map(|row| row.get(y)).flatten()
    }

    pub fn endpoint(&self) -> (usize, usize) {
        (self.max_x() - 1, self.max_y() - 1)
    }

    pub fn max_x(&self) -> usize {
        self.data[0].len()
    }

    pub fn max_y(&self) -> usize {
        self.data.len()
    }

    pub fn paint(&self, route: &Route) {
        for (i, line) in self.data.iter().enumerate() {
            for (j, dot) in line.iter().enumerate() {
                if route.contains(&(i, j)) {
                    print!("x ");
                } else {
                    print!("{} ", dot);
                }
            }
            println!("");
        }
    }

    pub fn foldout(&mut self, amount: usize) {
        let mut new_data = Vec::new();
        for (i, line) in self.data.iter().cycle().enumerate() {
            if i == self.max_x() * amount {
                break;
            }
            let mut row = Vec::new();
            for (j, cell) in line.iter().cycle().enumerate() {
                if j == self.max_y() * amount {
                    break;
                }
                let extra_score = (j / self.max_y() + i / self.max_x()) as u8;
                row.push((*cell - 1 + extra_score) % 9 + 1);
            }
            new_data.push(row);
        }
        self.data = new_data;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Route {
    cost: usize,
    path: Vec<(usize, usize)>,
}

impl Route {
    pub fn new() -> Route {
        let mut route = Route::default();
        route.push((0, 0), 0);
        route
    }

    pub fn push(&mut self, coordinate: (usize, usize), cost: usize) {
        self.path.push(coordinate);
        self.cost += cost;
    }

    pub fn next_steps(&self) -> Vec<(usize, usize)> {
        let mut places = Vec::new();
        if let Some((x, y)) = self.path.last().copied() {
            let _left = x.checked_sub(1).map(|x| places.push((x, y)));
            let _right = x.checked_add(1).map(|x| places.push((x, y)));
            let _up = y.checked_sub(1).map(|y| places.push((x, y)));
            let _down = y.checked_add(1).map(|y| places.push((x, y)));
        }
        places
    }

    pub fn contains(&self, coordinate: &(usize, usize)) -> bool {
        self.path.contains(coordinate)
    }

    pub fn last(&self) -> Option<&(usize, usize)> {
        self.path.last()
    }
}

type Pair = (usize, usize);

#[derive(Debug, Default)]
pub struct Edges {
    data: BTreeMap<Pair, BTreeMap<Pair, u8>>,
}

impl Edges {
    pub fn set(&mut self, from: Pair, to: Pair, cost: u8) {
        let x = self.data.entry(from).or_insert(BTreeMap::default());
        *x.entry(to).or_insert(0) = cost;
    }

    pub fn get(&self, from: &Pair, to: &Pair) -> Option<&u8> {
        self.data.get(from).map(|inner| inner.get(to)).flatten()
    }

    pub fn connections(&self, from: &Pair) -> Option<&BTreeMap<Pair, u8>> {
        self.data.get(from)
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    nodes: BTreeMap<Pair, Option<usize>>,
    edges: Edges,
    endpoint: Pair,
}

impl Graph {
    pub fn from_map(map: &Map) -> Graph {
        let mut nodes = BTreeMap::new();
        let mut edges = Edges::default();
        for (y, line) in map.iter().enumerate() {
            for (x, cost) in line.iter().enumerate() {
                let current_pos = (x, y);
                nodes.insert(current_pos.clone(), None);
                let mut connections = Vec::new();
                let _left = x.checked_sub(1).map(|x| connections.push((x, y)));
                let _right = x.checked_add(1).map(|x| connections.push((x, y)));
                let _up = y.checked_sub(1).map(|y| connections.push((x, y)));
                let _down = y.checked_add(1).map(|y| connections.push((x, y)));
                for connection in connections {
                    edges.set(connection, current_pos.clone(), *cost)
                }
            }
        }
        Graph {
            nodes,
            edges,
            endpoint: map.endpoint(),
        }
    }

    /// just go over all points and check all neighbours
    pub fn start_bruteforce(&mut self) {
        self.nodes.insert((0, 0), Some(0));
        let (x, y) = self.endpoint;

        for _ in 0..3 {
            for i in 0..=x {
                for j in 0..=y {
                    let current_pos = (i, j);
                    if let Some(connections) = self.edges.connections(&current_pos) {
                        for (connection, cost) in connections {
                            let current_cost = self
                                .nodes
                                .get(&current_pos)
                                .unwrap_or(&None)
                                .unwrap_or(100000);
                            let new_cost = current_cost + *cost as usize;
                            match self.nodes.get_mut(&connection).expect("node should exist") {
                                Some(point) => {
                                    if *point > new_cost {
                                        *point = new_cost
                                    }
                                }
                                point => *point = Some(new_cost),
                            };
                        }
                    }
                }
            }
        }
    }

    #[allow(unreachable_code)]
    pub fn start_smart(&mut self) {
        // incomplete

        return;
        let mut current_pos = (0, 0);
        loop {
            if let Some(connections) = self.edges.connections(&current_pos) {
                let mut next_pos = None;
                for (connection, cost) in connections {
                    let current_cost = self
                        .nodes
                        .get(&current_pos)
                        .unwrap_or(&None)
                        .unwrap_or(100000);
                    println!("{:?}", cost);
                    println!("{:?}", connection);
                    let new_cost = current_cost + *cost as usize;
                    match self.nodes.get_mut(&connection).expect("node should exist") {
                        Some(x) => {
                            if *x > new_cost {
                                *x = new_cost
                            }
                        }
                        y => *y = Some(new_cost),
                    };
                    next_pos = Some(connection);
                }

                if Some(&self.endpoint) == next_pos {
                    break;
                }
                if let Some(next_pos) = next_pos {
                    current_pos = *next_pos;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn draw(&self) {
        let (x, y) = self.endpoint;

        for i in 0..=x {
            for j in 0..=y {
                if let Some(Some(point)) = self.nodes.get(&(i, j)) {
                    print!("{: ^6}", point)
                } else {
                    print!("{: ^6}", "x")
                }
                print!("|")
            }
            println!("")
        }
    }

    pub fn endpoint_score(&self) -> Option<&usize> {
        if let Some(Some(score)) = self.nodes.get(&self.endpoint) {
            Some(score)
        } else {
            None
        }
    }
}

// #[derive(Debug)]
// pub struct Node {
//     label: (usize, usize),
//     visted: Option<usize>,
// }

// impl Node {
//     pub fn new(pair: Pair) -> Node {
//         Node {
//             label: pair,
//             visted: None,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Edge<'a> {
//     left: &'a Pair,
//     right: &'a Pair,
//     cost: u8,
// }

// impl<'a> Edge<'a> {
//     pub fn new(left: &'a Pair, right: &'a Pair, cost: u8) -> Edge<'a> {
//         Edge { left, right, cost }
//     }
// }

#[derive(Debug)]
pub struct Finder<'a> {
    map: &'a Map,
    route: Route,
    scores: BTreeMap<(usize, usize), usize>,
}

impl<'a> Finder<'a> {
    pub fn new(map: &'a Map) -> Finder<'a> {
        Finder {
            map,
            route: Route::new(),
            scores: BTreeMap::new(),
        }
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day15/src/input.txt"
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

    let map = Map::from_bufreader(buffer)?;
    let mut graph = Graph::from_map(&map);
    graph.start_bruteforce();

    graph
        .endpoint_score()
        .copied()
        .ok_or("endpoint score not found".into())
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);
    let mut map = Map::from_bufreader(buffer)?;
    map.foldout(5);

    let mut graph = Graph::from_map(&map);
    graph.start_bruteforce();

    graph
        .endpoint_score()
        .copied()
        .ok_or("endpoint score not found".into())
}

#[test]
fn day15_part_one() {
    assert_eq!(508, part_one(fetch_file_path()).unwrap())
}

// only possible to get if you run in release mode
#[ignore]
#[test]
fn day15_part_two() {
    assert_eq!(2872, part_two(fetch_file_path()).unwrap())
}

#[cfg(test)]
mod map_test {
    use super::Map;
    use std::io::{BufReader, Cursor};

    #[test]
    fn from_bufreader() {
        let text = "123
456
789
222";
        let expected = Map::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![2, 2, 2],
        ]);
        let map = Map::from_bufreader(BufReader::new(Cursor::new(text))).unwrap();

        assert_eq!(map, expected)
    }

    #[test]
    fn get() {
        let map = Map::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(Some(&2), map.get(0, 1))
    }

    #[test]
    fn endpoint() {
        let map = Map::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![2, 2, 2],
        ]);
        assert_eq!((2, 3), map.endpoint())
    }

    #[test]
    fn foldout() {
        let mut map = Map::new(vec![vec![1, 2], vec![4, 5]]);

        let expected = Map::new(vec![
            vec![1, 2, 2, 3, 3, 4],
            vec![4, 5, 5, 6, 6, 7],
            vec![2, 3, 3, 4, 4, 5],
            vec![5, 6, 6, 7, 7, 8],
            vec![3, 4, 4, 5, 5, 6],
            vec![6, 7, 7, 8, 8, 9],
        ]);

        map.foldout(3);
        assert_eq!(map, expected)
    }

    #[test]
    fn foldout_wraps_around() {
        let mut map = Map::new(vec![vec![9, 7], vec![8, 9]]);
        let expected = Map::new(vec![
            vec![9, 7, 1, 8],
            vec![8, 9, 9, 1],
            vec![1, 8, 2, 9],
            vec![9, 1, 1, 2],
        ]);

        map.foldout(2);
        assert_eq!(map, expected)
    }
}

#[cfg(test)]
mod graph_test {
    use super::{Graph, Map};

    #[test]
    fn from_map() {
        let map = Map::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        let graph = Graph::from_map(&map);

        assert_eq!(&None, graph.nodes.get(&(0, 0)).unwrap());
        assert_eq!(&None, graph.nodes.get(&(2, 1)).unwrap());
        assert_eq!(None, graph.nodes.get(&(5, 5)));
        assert_eq!(&4, graph.edges.get(&(0, 0), &(0, 1)).unwrap());
        assert_eq!(&8, graph.edges.get(&(2, 2), &(1, 2)).unwrap());
        assert_eq!(None, graph.edges.get(&(1, 1), &(1, 1)));

        assert_eq!(
            vec![((0, 1), 4), ((1, 0), 2)],
            graph
                .edges
                .connections(&(0, 0))
                .cloned()
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![((0, 1), 4), ((1, 0), 2), ((1, 2), 8), ((2, 1), 6)],
            graph
                .edges
                .connections(&(1, 1))
                .cloned()
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>()
        );
    }
}
