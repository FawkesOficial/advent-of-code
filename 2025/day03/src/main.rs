fn part1(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|bank_str| {
            let bank = bank_str
                .chars()
                .map(|battery| battery.to_digit(10).expect("battery should be a digit") as u64);

            let left = bank.clone().take(bank_str.len() - 1); // take all but the last digit

            // note: we have to use manual fold here since in rust's .max() implementation,
            //       "if several elements are equally maximum, the last element is returned",
            //       and we're actually interested in the very first occurence of the max digit
            let (max_idx, max_first_digit) = left
                .enumerate()
                .fold(None, |best, (idx, battery)| match best {
                    None => Some((idx, battery)),
                    Some((_best_idx, best_battery)) if battery > best_battery => {
                        Some((idx, battery))
                    }
                    Some(b) => Some(b),
                })
                .expect("expected at least one battery");

            // search for second digit in the remaining digits/baterries
            let right = bank.skip(max_idx + 1);
            let max_second_digit = right.max().expect("expected at least one battery");

            let max_voltage = max_first_digit * 10 + max_second_digit;

            max_voltage
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
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.trim()), Some(357));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.trim()), None);
    }

    #[test]
    fn test_part1_01() {
        let example = r#"2222254221221322323251221422122222222913213312413322232222233312242241212222211222221231522123222321"#;
        assert_eq!(part1(example.trim()), Some(95));
    }

    #[test]
    fn test_part1_02() {
        let example = r#"998"#;
        assert_eq!(part1(example.trim()), Some(99));
    }
}
