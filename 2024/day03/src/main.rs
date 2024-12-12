use regex::Regex;
use std::fs;

fn parse_mul_capture(capture: regex::Captures<'_>) -> u64 {
    let [x, y]: [u64; 2] = capture
        .iter()
        .skip(1)
        .map(|group| {
            group
                .expect("expected a match")
                .as_str()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();

    x * y
}

fn p1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|cap| parse_mul_capture(cap))
        .sum::<u64>() as u64
}

fn p2(input: &str) -> u64 {
    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut enabled = true;

    re.captures_iter(input)
        .filter_map(|cap| match cap.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                None
            }
            "don't()" => {
                enabled = false;
                None
            }
            _ => {
                if enabled {
                    Some(parse_mul_capture(cap))
                } else {
                    None
                }
            }
        })
        .sum::<u64>() as u64
}

fn main() {
    let input = fs::read_to_string("../../input.txt").expect("error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
