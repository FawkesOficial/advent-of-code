const DIAL_START: i64 = 50;
const DIAL_SIZE: i64 = 100;

fn add_mod(x: i64, y: i64, n: i64) -> i64 {
    ((x % n + y % n) % n + n) % n
}

fn part1(input: &str) -> Option<u64> {
    let (_final_pos, zero_count) =
        input
            .lines()
            .fold((DIAL_START, 0), |(current_pos, zero_count), line| {
                let (turn, dist) = line.split_at(1);
                let dist: i64 = dist.parse().unwrap();

                let new_pos = match turn {
                    "L" => add_mod(current_pos, -dist, DIAL_SIZE),
                    "R" => add_mod(current_pos, dist, DIAL_SIZE),
                    _ => unreachable!(),
                };

                (new_pos, zero_count + (new_pos == 0) as u64)
            });

    Some(zero_count)
}

fn part2(input: &str) -> Option<u64> {
    let (_final_pos, zero_count) =
        input
            .lines()
            .fold((DIAL_START, 0), |(current_pos, zero_count), line| {
                let (turn, dist) = line.split_at(1);
                let dist: i64 = dist.parse().unwrap();

                let dir = match turn {
                    "L" => -1,
                    "R" => 1,
                    _ => unreachable!(),
                };

                let mut new_pos = current_pos;
                let mut remaining = dist;
                let mut zero_crosses = 0;
                while remaining > 0 {
                    new_pos = (new_pos + dir).rem_euclid(DIAL_SIZE);
                    zero_crosses += (new_pos == 0) as u64;

                    remaining -= 1;
                }

                (new_pos, zero_count + zero_crosses)
            });

    Some(zero_count)
}

fn main() {
    let input = include_str!("../../../input.txt");

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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.trim()), Some(3));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.trim()), Some(6));
    }
}
