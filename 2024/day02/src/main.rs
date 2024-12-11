use std::fs;

fn is_safe(report: &[u32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    report.windows(2).all(|w| {
        let [prev, next]: [u32; 2] = w
            .try_into()
            .expect("expected window to fit two sized slice");

        let diff = prev.abs_diff(next);

        is_increasing = prev < next && is_increasing;
        is_decreasing = prev > next && is_decreasing;

        (is_increasing || is_decreasing) && (1 <= diff && diff <= 3)
    })
}

fn is_safe_with_problem_dampener(report: &[u32]) -> bool {
    (0..report.len()).any(|idx| {
        let filtered_report = report
            .iter()
            .take(idx)
            .chain(report.iter().skip(idx + 1))
            .cloned()
            .collect::<Vec<_>>();

        is_safe(&filtered_report)
    })
}

fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().expect("expected a number"))
                .collect::<Vec<_>>()
        })
        .filter(|report| is_safe(report))
        .count() as u64
}

fn p2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().expect("expected a number"))
                .collect::<Vec<_>>()
        })
        .filter(|report| is_safe_with_problem_dampener(report))
        .count() as u64
}

fn main() {
    let input = fs::read_to_string("../../input.txt").expect("error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
