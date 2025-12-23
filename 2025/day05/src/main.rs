use std::cmp::max;
use std::{collections::HashSet, ops::RangeInclusive};

fn parse_range(range: &str) -> RangeInclusive<u64> {
    let (first_id, last_id) = range.split_once("-").expect("invalid range");

    let first_id: u64 = first_id.parse().expect("first id is not a number");
    let last_id: u64 = last_id.parse().expect("last id is not a number");

    first_id..=last_id
}

fn parse_database(input: &str) -> (Vec<RangeInclusive<u64>>, HashSet<u64>) {
    let (fresh_ranges, available_ingredients) =
        input.split_once("\n\n").expect("invalid database format");

    let fresh_ranges: Vec<RangeInclusive<u64>> = fresh_ranges.lines().map(parse_range).collect();
    let available_ingredients: HashSet<u64> = available_ingredients
        .lines()
        .map(|id| id.parse().expect("id is not a number"))
        .collect();

    (fresh_ranges, available_ingredients)
}

fn is_fresh(ingredient_id: u64, fresh_ranges: &[RangeInclusive<u64>]) -> bool {
    fresh_ranges
        .iter()
        .any(|range| range.contains(&ingredient_id))
}

fn ranges_union(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return ranges.to_vec();
    }

    let mut ranges = ranges.to_vec().clone();
    ranges.sort_by_key(|range| *range.start());

    let mut result: Vec<RangeInclusive<u64>> = Vec::new();
    for range in ranges {
        match result.last_mut() {
            Some(last) => {
                // overlapping or adjacent (inclusive)
                if *range.start() <= *last.end() + 1 {
                    *last = *last.start()..=max(*last.end(), *range.end());
                } else {
                    result.push(range);
                }
            }
            None => result.push(range),
        }
    }

    result
}

fn part1(input: &str) -> Option<u64> {
    let (fresh_ranges, available_ingredients) = parse_database(input);

    let fresh_count: u64 = available_ingredients
        .into_iter()
        .filter(|&ingredient_id| is_fresh(ingredient_id, &fresh_ranges))
        .count() as u64;

    Some(fresh_count)
}

fn part2(input: &str) -> Option<u64> {
    let (fresh_ranges, _) = parse_database(input);

    let fresh_count: u64 = ranges_union(&fresh_ranges)
        .into_iter()
        .map(|range| range.count() as u64)
        .sum();

    Some(fresh_count)
}

fn main() {
    let input = include_str!("../../../input.txt").trim();

    if let Some(p1_result) = part1(&input) {
        println!("p1: {}", p1_result);
    }
    if let Some(p2_result) = part2(&input) {
        println!("p2: {}", p2_result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.trim()), Some(3));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.trim()), Some(14));
    }
}
