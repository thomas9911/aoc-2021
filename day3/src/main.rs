use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fs::read_to_string;
use std::path::Path;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day3/src/input.txt"
    }
}

fn parse_input(path: &str) -> Option<Vec<VecDeque<u8>>> {
    let text = read_to_string(path).ok()?;

    let data: Vec<VecDeque<u8>> = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| {
                    u8::try_from(digit.to_digit(2).expect("invalid binary"))
                        .expect("one or zero always fits in u8")
                })
                .collect()
        })
        .collect();

    Some(data)
}

fn transpose_input(mut input: Vec<VecDeque<u8>>) -> Vec<Vec<u8>> {
    let mut transposed_vec = Vec::new();
    let size = input.len();

    for _ in 0..input[0].len() {
        let mut tmp = Vec::with_capacity(size);
        for line in &mut input {
            tmp.push((*line).pop_front().expect("invalid input columns"));
        }
        transposed_vec.push(tmp)
    }

    transposed_vec
}

fn most_common_bits(data: &[Vec<u8>]) -> Vec<u8> {
    data.iter().map(|x| most_common_bit(x)).collect()
}

fn most_common_bit(list: &[u8]) -> u8 {
    let sum = list.iter().map(|x| *x as u64).sum::<u64>() as f32;
    let avg = sum / list.len() as f32;
    avg.round() as u8
}

fn invert_bits(input: Vec<u8>) -> Vec<u8> {
    input.into_iter().map(|x| 1 - x).collect()
}

fn binary_to_number(input: &[u8]) -> i64 {
    let mut number = 0;
    for (position, bit) in input.iter().rev().enumerate() {
        number += (*bit as i64) * 2_i64.pow(position as u32);
    }
    number
}

fn find_rating<F, G>(mut input: Vec<VecDeque<u8>>, mut filter0: F, mut filter1: G) -> i64
where
    F: FnMut(&VecDeque<u8>, usize) -> bool,
    G: FnMut(&VecDeque<u8>, usize) -> bool,
{
    let length = input[0].len();

    for i in 0..length {
        let mut ones = 0;
        let mut zeroes = 0;

        for line in input.clone() {
            if line[i] == 1 {
                ones += 1
            } else {
                zeroes += 1
            }
        }

        input = if ones >= zeroes {
            input.into_iter().filter(|x| filter0(x, i)).collect()
        } else {
            input.into_iter().filter(|x| filter1(x, i)).collect()
        };

        if input.len() == 1 {
            break;
        }
    }

    binary_to_number(&input[0].make_contiguous())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);
    Ok(())
}

fn part_one(input_file: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let input = parse_input(input_file).ok_or(String::from("invalid input"))?;
    let input = transpose_input(input);
    let most_common_bits = most_common_bits(&input);
    let gamma = binary_to_number(&most_common_bits);
    let least_common_bits = invert_bits(most_common_bits);
    let epsilon = binary_to_number(&least_common_bits);

    Ok(gamma * epsilon)
}

fn part_two(input_file: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let input = parse_input(input_file).ok_or(String::from("invalid input"))?;
    let oxygen_generator_rating = find_rating(input.clone(), |x, i| x[i] == 1, |x, i| x[i] == 0);
    let co2_scrubber_rating = find_rating(input.clone(), |x, i| x[i] != 1, |x, i| x[i] != 0);

    Ok(oxygen_generator_rating * co2_scrubber_rating)
}

#[test]
fn day3_one() {
    assert_eq!(1997414, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day3_two() {
    assert_eq!(1032597, part_two(fetch_file_path()).unwrap())
}
