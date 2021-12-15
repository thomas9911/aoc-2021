use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Rules = BTreeMap<(char, char), char>;
pub type Template = Vec<char>;

pub mod part1;

#[derive(Debug, Default)]
struct Counter {
    data: BTreeMap<char, usize>,
}

impl Counter {
    pub fn put(&mut self, ch: char) {
        self.put_amount(ch, 1)
    }

    pub fn put_amount(&mut self, ch: char, amount: usize) {
        *self.data.entry(ch).or_insert(0) += amount;
    }

    pub fn max(&self) -> Option<(char, usize)> {
        self.data
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(ch, count)| (*ch, *count))
    }

    pub fn min(&self) -> Option<(char, usize)> {
        self.data
            .iter()
            .min_by_key(|(_, count)| *count)
            .map(|(ch, count)| (*ch, *count))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TemplateCollection {
    starting: char,
    ending: char,
    pairs: BTreeMap<(char, char), usize>,
}

impl std::fmt::Display for TemplateCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer: String = self.clone().into_iter().collect();
        write!(f, "{}", buffer)
    }
}

impl TemplateCollection {
    pub fn apply(&mut self, rules: &Rules) {
        let mut new_pairs = BTreeMap::new();
        for (pair, count) in self.pairs.iter() {
            if let Some(x) = rules.get(&pair).copied() {
                *new_pairs.entry((pair.0, x)).or_insert(0) += count;
                *new_pairs.entry((x, pair.1)).or_insert(0) += count;
            } else {
                *new_pairs.entry(*pair).or_insert(0) += count;
            }
        }

        self.pairs = new_pairs;
    }
}

impl From<&str> for TemplateCollection {
    fn from(input: &str) -> TemplateCollection {
        if input.len() < 3 {
            TemplateCollection::default()
        } else {
            let chars: Vec<_> = input.chars().collect();
            let starting = chars.first().copied().unwrap();
            let ending = chars.last().copied().unwrap();

            let mut pairs = BTreeMap::new();
            for pair in chars.windows(2) {
                *pairs.entry((pair[0], pair[1])).or_insert(0) += 1
            }
            TemplateCollection {
                starting,
                ending,
                pairs,
            }
        }
    }
}

impl IntoIterator for TemplateCollection {
    type Item = <TemplateIter as Iterator>::Item;
    type IntoIter = TemplateIter;

    fn into_iter(self) -> Self::IntoIter {
        TemplateIter {
            pairs: self.pairs,
            previous_char: Some(self.starting),
        }
    }
}

pub struct TemplateIter {
    pairs: BTreeMap<(char, char), usize>,
    previous_char: Option<char>,
}

impl Iterator for TemplateIter {
    type Item = char;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // sadly it does not work, but sometimes it does :D
        // also this was not needed for the solution

        let pair = match self.pairs.iter().find_map(|((a, b), _)| {
            if Some(a) == self.previous_char.as_ref() && a == b {
                Some((*a, *b))
            } else {
                None
            }
        }) {
            Some(pair) => pair,
            None => {
                match self.pairs.iter().find_map(|((a, b), _)| {
                    if Some(a) == self.previous_char.as_ref() {
                        Some((*a, *b))
                    } else {
                        None
                    }
                }) {
                    Some(pair) => pair,
                    None => return self.previous_char.take(),
                }
            }
        };

        self.pairs.get_mut(&pair).map(|x| *x -= 1);
        if let Some(0) = self.pairs.get(&pair) {
            self.pairs.remove(&pair);
        };

        self.previous_char = Some(pair.1);

        Some(pair.0)
    }
}

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day14/src/input.txt"
    }
}

pub fn parse_template_and_rules(
    mut iter: Box<dyn Iterator<Item = Result<String, std::io::Error>>>,
) -> Result<(String, Rules), Box<dyn std::error::Error>> {
    let template = iter.next().ok_or("first line should contain template")??;
    iter.next().ok_or("second line should be empty")??;

    let rules: Result<Rules, Box<dyn std::error::Error>> = iter
        .map(|line| {
            if let Some(res) = line?.split_once(" -> ").map(|(left, right)| {
                if left.len() == 2 && right.len() == 1 {
                    let mut chars = left.chars();
                    // we already checked in the if statement if this will succeed
                    let pair_a = chars.next().unwrap();
                    let pair_b = chars.next().unwrap();
                    let insert = right.chars().next().unwrap();
                    Ok(((pair_a, pair_b), insert))
                } else {
                    Err("invalid line".into())
                }
            }) {
                res
            } else {
                Err("invalid line".into())
            }
        })
        .collect();

    Ok((template, rules?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part1::part_one(input_file)?);
    println!("part one again: {:?}", part_one_compact(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one_compact(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);

    let lines = Box::new(buffer.lines());
    let (template, rules) = parse_template_and_rules(lines)?;

    let mut template = TemplateCollection::from(&*template);

    for _ in 0..10 {
        template.apply(&rules);
    }

    let mut counter = Counter::default();
    counter.put(template.ending);
    for ((left, _), amount) in template.pairs.iter() {
        counter.put_amount(*left, *amount);
    }

    Ok(counter.max().unwrap().1 - counter.min().unwrap().1)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let buffer = BufReader::new(file);

    let lines = Box::new(buffer.lines());
    let (template, rules) = parse_template_and_rules(lines)?;

    let mut template = TemplateCollection::from(&*template);

    for _ in 0..40 {
        template.apply(&rules);
    }

    let mut counter = Counter::default();
    counter.put(template.ending);
    for ((left, _), amount) in template.pairs.iter() {
        counter.put_amount(*left, *amount);
    }

    Ok(counter.max().unwrap().1 - counter.min().unwrap().1)
}

#[test]
fn day14_part_one() {
    assert_eq!(2937, part1::part_one(fetch_file_path()).unwrap())
}

#[test]
fn day14_part_one_compact() {
    assert_eq!(2937, part_one_compact(fetch_file_path()).unwrap())
}

#[test]
fn day12_part_two() {
    assert_eq!(3390034818249, part_two(fetch_file_path()).unwrap())
}

#[test]
fn template_collection_into_iter() {
    let t = TemplateCollection::from("ABCDABE");
    assert_eq!("ABCDABE", t.into_iter().collect::<String>())
}

#[test]
fn template_collection_into_iter2() {
    let t = TemplateCollection::from("NNCB");
    assert_eq!("NNCB", t.into_iter().collect::<String>())
}

#[test]
fn template_apply_steps() {
    let rules = BTreeMap::from_iter([
        (('B', 'B'), 'N'),
        (('B', 'C'), 'B'),
        (('B', 'H'), 'H'),
        (('B', 'N'), 'B'),
        (('C', 'B'), 'H'),
        (('C', 'C'), 'N'),
        (('C', 'H'), 'B'),
        (('C', 'N'), 'C'),
        (('H', 'B'), 'C'),
        (('H', 'C'), 'B'),
        (('H', 'H'), 'N'),
        (('H', 'N'), 'C'),
        (('N', 'B'), 'B'),
        (('N', 'C'), 'B'),
        (('N', 'H'), 'C'),
        (('N', 'N'), 'C'),
    ]);
    let mut t = TemplateCollection::from("NNCB");
    t.apply(&rules);
    assert_eq!(TemplateCollection::from("NCNBCHB"), t);
    t.apply(&rules);
    assert_eq!(TemplateCollection::from("NBCCNBBBCBHCB"), t);
    t.apply(&rules);
    assert_eq!(TemplateCollection::from("NBBBCNCCNBBNBNBBCHBHHBCHB"), t);
    t.apply(&rules);
    assert_eq!(
        TemplateCollection::from("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"),
        t
    );
}
