use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::usize;

pub mod finder;

const _DRAWING: &'static str = r#"
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
"#;

const UNIQUE_LENGTH_DIGITS: &[usize] = &[2, 3, 4, 7];

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day8/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    let mut number = 0;
    for line in reader.lines() {
        if let Some((_, last_part)) = line?.split_once('|') {
            number += last_part
                .split(" ")
                .filter(|s| UNIQUE_LENGTH_DIGITS.contains(&s.len()))
                .count();
        };
    }

    Ok(number)
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    let mut number = 0;
    for line in reader.lines() {
        if let Some((first_part, remaining_digits)) = line?.split_once('|') {
            let info: BTreeMap<usize, Vec<BTreeSet<char>>> =
                first_part
                    .trim()
                    .split(' ')
                    .fold(BTreeMap::new(), |mut acc, x| {
                        acc.entry(x.len())
                            .or_insert(Vec::new())
                            .push(x.chars().collect());
                        acc
                    });

            let mapping = determine_mapping(&info);
            let score = parse_remaining_digits(remaining_digits)
                .map(|x| mapping.get(&x).expect("digit not found"))
                .rev()
                .enumerate()
                .fold(0, reduce_number_decimal);
            number += score;
        };
    }

    Ok(number)
}

fn reduce_number_decimal(acc: usize, item: (usize, &usize)) -> usize {
    acc + item.1 * 10usize.pow(item.0 as u32)
}

fn parse_remaining_digits<'a>(
    input: &'a str,
) -> Box<dyn DoubleEndedIterator<Item = BTreeSet<char>> + 'a> {
    Box::new(
        input
            .trim()
            .split(' ')
            .map(|x| x.chars().collect::<BTreeSet<_>>()),
    )
}

fn determine_mapping(
    data: &BTreeMap<usize, Vec<BTreeSet<char>>>,
) -> BTreeMap<&BTreeSet<char>, usize> {
    let mut digit_mapping = BTreeMap::new();
    let one = data[&2].get(0).expect("incomplete data");
    let seven = data[&3].get(0).expect("incomplete data");
    let four = data[&4].get(0).expect("incomplete data");
    let eight = data[&7].get(0).expect("incomplete data");

    digit_mapping.insert(one, 1);
    digit_mapping.insert(four, 4);
    digit_mapping.insert(seven, 7);
    digit_mapping.insert(eight, 8);

    let two_five_six = data
        .values()
        .flat_map(|x| x.iter())
        .filter(|x| !x.is_superset(one));

    let six = two_five_six
        .clone()
        .filter(|x| x.len() == 6)
        .next()
        .expect("six cannot be found");
    digit_mapping.insert(six, 6);

    let two_five: Vec<_> = two_five_six.filter(|x| x.len() == 5).collect();

    let mut five = None;
    let mut two = None;
    for (is_five, chars) in two_five.into_iter().map(|x| (x.is_subset(six), x)) {
        if is_five {
            five = Some(chars)
        } else {
            two = Some(chars)
        }
    }
    let five = five.expect("five cannot be found");
    let two = two.expect("two cannot be found");
    let three = data[&5]
        .iter()
        .filter(|x| *x != two && *x != five)
        .next()
        .expect("three cannot be found");

    digit_mapping.insert(two, 2);
    digit_mapping.insert(five, 5);
    digit_mapping.insert(three, 3);

    let middle_and_bottom: BTreeSet<_> = three.difference(seven).cloned().collect();
    let middle = middle_and_bottom
        .intersection(four)
        .next()
        .expect("middle not found");

    let nine = data[&6]
        .iter()
        .filter(|x| x != &six)
        .filter(|x| x.contains(middle))
        .next()
        .expect("nine cannot be found");
    let zero = data[&6]
        .iter()
        .filter(|x| x != &six && x != &nine)
        .next()
        .expect("zero cannot be found");

    digit_mapping.insert(zero, 0);
    digit_mapping.insert(nine, 9);

    digit_mapping
}

#[test]
fn day8_part_one() {
    assert_eq!(247, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day8_part_two() {
    assert_eq!(933305, part_two(fetch_file_path()).unwrap())
}
