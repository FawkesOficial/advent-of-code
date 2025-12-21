fn parse_bank(bank: &str) -> impl Iterator<Item = u64> + Clone {
    bank.chars()
        .map(|battery| battery.to_digit(10).expect("battery should be a digit") as u64)
}

// [note]: we need this since in rust's .max()/.max_by_key() implementation,
//         "if several elements are equally maximum, the last element is returned",
//         and we're actually interested in the very first occurence of the max digit
fn first_max_by_key<I, T, K, F>(iter: I, key: F) -> Option<T>
where
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> K,
    K: Ord,
{
    iter.into_iter()
        .reduce(|a, b| if key(&b) > key(&a) { b } else { a })
}

fn part1(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let bank = parse_bank(line);

            // - [ 1. search for the first occurence of the max digit ]
            let left = bank.clone().take(line.len() - 1); // take all but the last digit

            let (max_idx, first_digit) =
                first_max_by_key(left.enumerate(), |&(_, battery)| battery)
                    .expect("expected at least one battery");

            // - [ 2. search for the second digit in the remaining digits/baterries ] -
            let right = bank.skip(max_idx + 1);
            let second_digit = right.max().expect("expected at least one battery");

            // - [ 3. build the final voltage number ] -
            let max_voltage = first_digit * 10 + second_digit;

            max_voltage
        })
        .sum();

    Some(result)
}

fn part2(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let bank = parse_bank(line);

            let mut max_voltage_digits: Vec<u64> = Vec::new();

            // - [ 1. iteratively search for the first max digit while moving forward ] -
            //
            // ex: bank = 234234234234278
            //
            //     1st iteration: search only *2342* for a first max digit,        *2342* | 34234234278 --> 11 spare digits
            //                    the rest (34234234278) will be searched later
            //                    max is 4 and it is at index 2, so next time we will skip the first 2+1 digits
            //
            //     2nd iteration: skip the first 2+1 digits (234).   skiped from previous iteration <-- 234 | *23* | 4234234278 --> 10 spare digits
            //                    search *23* for a first max digit,
            //                    the rest (4234234278) will be searched later
            //                    max is 3 and it is at index 1 (this is relative)
            //
            //     ...
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

                let (relative_max_idx, new_digit) =
                    first_max_by_key(left.enumerate(), |&(_, battery)| battery)
                        .expect("expected at least one battery");

                max_idx = max_idx + relative_max_idx as i64;
                max_voltage_digits.push(new_digit);
            }

            assert_eq!(max_voltage_digits.len(), 12); // sanity check

            // - [ 2. build the final voltage number ] -
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
