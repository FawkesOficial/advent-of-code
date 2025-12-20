fn part1(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let bank = line
                .chars()
                .map(|battery| battery.to_digit(10).expect("battery should be a digit") as u64);

            let left = bank.clone().take(line.len() - 1); // take all but the last digit

            // [note]: we have to use manual fold here since in rust's .max() implementation,
            //         "if several elements are equally maximum, the last element is returned",
            //         and we're actually interested in the very first occurence of the max digit
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

fn part2(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let bank = line
                .chars()
                .map(|battery| battery.to_digit(10).expect("battery should be a digit") as u64);

            let mut max_voltage_digits: Vec<u64> = Vec::new();

            let mut max_idx: i64 = -1;
            for i in 0..12 {
                max_idx += 1;
                let left = bank
                    .clone()
                    .take(line.len() - (11 - i))
                    .skip(max_idx as usize);

                // [note]: for debug purposes
                // let left_str = left
                //     .clone()
                //     .into_iter()
                //     .map(|battery| battery.to_string())
                //     .collect::<String>();
                // println!("exploring: {}", left_str);

                // [note]: we have to use manual fold here since in rust's .max() implementation,
                //         "if several elements are equally maximum, the last element is returned",
                //         and we're actually interested in the very first occurence of the max digit
                let (new_max_idx, new_digit) = left
                    .enumerate()
                    .fold(None, |best, (idx, battery)| match best {
                        None => Some((idx, battery)),
                        Some((_best_idx, best_battery)) if battery > best_battery => {
                            Some((idx, battery))
                        }
                        Some(b) => Some(b),
                    })
                    .expect("expected at least one battery");

                max_idx = max_idx + new_max_idx as i64;
                max_voltage_digits.push(new_digit);
            }

            assert_eq!(max_voltage_digits.len(), 12); // sanity check

            let max_voltage = max_voltage_digits
                .iter()
                .enumerate()
                .map(|(i, digit)| digit * 10_u64.pow(12 - (i as u32 + 1)))
                .sum::<u64>();

            max_voltage
        })
        .sum();

    Some(result)
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
        assert_eq!(part2(EXAMPLE.trim()), Some(3121910778619));
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
