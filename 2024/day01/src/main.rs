use std::collections::HashMap;
use std::fs;

fn p1(input: &str) -> u32 {
    let (mut left_nums, mut right_nums): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok());

            (
                nums.next().expect("expected left number"),
                nums.next().expect("expected right number"),
            )
        })
        .unzip();

    left_nums.sort();
    right_nums.sort();

    left_nums
        .into_iter()
        .zip(right_nums.into_iter())
        .map(|(left_num, right_num)| left_num.abs_diff(right_num))
        .sum()
}

fn p2(input: &str) -> u32 {
    let mut left_nums: Vec<u32> = Vec::new();
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok());

        left_nums.push(nums.next().expect("expected left number"));

        *frequency_map
            .entry(nums.next().expect("expected right number"))
            .or_insert(0) += 1;
    }

    left_nums
        .into_iter()
        .map(|num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = fs::read_to_string("../../input.txt").expect("error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
