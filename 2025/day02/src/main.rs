fn is_invalid(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }

    let (first, second) = id.split_at(id.len() / 2);

    first == second
}

fn part1(input: &str) -> Option<u64> {
    let result = input
        .split(",")
        .map(|range| {
            let (first_id, last_id) = range.split_once("-").expect("invalid range");
            let first_id: u64 = first_id.parse().expect("first id is not a number");
            let last_id: u64 = last_id.parse().expect("last id is not a number");

            (first_id..=last_id)
                .filter(|id| is_invalid(&id.to_string()))
                .sum::<u64>()
        })
        .sum();

    Some(result)
}

fn part2(_input: &str) -> Option<u64> {
    None
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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.trim()), Some(1227775554));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.trim()), None);
    }
}
