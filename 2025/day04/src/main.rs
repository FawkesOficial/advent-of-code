const PAPER_ROLL_CHAR: char = '@';

fn is_paper_roll(chr: &char) -> bool {
    *chr == PAPER_ROLL_CHAR
}

fn surroundings(r: usize, c: usize, grid: &[Vec<char>]) -> impl Iterator<Item = char> {
    let deltas = [
        (-1, -1), // top left
        (-1, 0),  // top middle
        (-1, 1),  // top right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // bottom left
        (1, 0),   // bottom middle
        (1, 1),   // bottom right
    ];

    deltas.into_iter().filter_map(move |(dr, dc)| {
        let nr = r.checked_add_signed(dr)?;
        let nc = c.checked_add_signed(dc)?;

        grid.get(nr)?.get(nc).copied()
    })
}

fn part1(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let accessed_paper_rolls = grid
        .iter()
        .enumerate()
        .flat_map(|(r, line)| line.iter().enumerate().map(move |(c, chr)| (r, c, chr)))
        .filter(|&(r, c, chr)| {
            is_paper_roll(chr) && surroundings(r, c, &grid).filter(is_paper_roll).count() < 4
        })
        .count() as u64;

    Some(accessed_paper_rolls)
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.trim()), Some(13));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.trim()), Some(43));
    }
}
