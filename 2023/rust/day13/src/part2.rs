use std::fs;

#[derive(PartialEq, Eq)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Eq)]
struct ReflectionLine {
    index: i64,
    orientation: Orientation,
}

fn get_column(col_idx: i64, grid: &Vec<Vec<char>>) -> Vec<char> {
    return (0..grid.len())
        .map(|i| grid[i][col_idx as usize])
        .collect::<Vec<char>>();
}

fn diff_count(v1: &Vec<char>, v2: &Vec<char>) -> i64 {
    v1.iter().zip(v2.iter()).filter(|(a, b)| a != b).count() as i64
}

fn find_vertical_reflection_lines(
    pattern: &Vec<Vec<char>>,
    allowed_smudges: i64,
) -> Vec<ReflectionLine> {
    let mut refl_lines: Vec<ReflectionLine> = Vec::new();

    let cols: i64 = pattern.first().expect("Expected a first line").len() as i64;
    for j in 0..(cols - 1) {
        let mut smudge_count: i64 = 0;
        let mut k: i64 = 0;
        let mut diff: i64 = 0;
        while (0..cols).contains(&(j - k)) && (0..cols).contains(&(j + 1 + k)) {
            diff = diff_count(&get_column(j - k, pattern), &get_column(j + 1 + k, pattern));
            if diff == 0 {
                k += 1;
            } else if diff == 1 && smudge_count < allowed_smudges {
                smudge_count += 1;
                k += 1;
            } else {
                break;
            }
        }

        if j - k == -1
            || j + 1 + k == cols && (diff == 0 || (diff == 1 && smudge_count <= allowed_smudges))
        {
            // println!("Found vertical reflection line in column {}", j + 1);
            refl_lines.push(ReflectionLine {
                index: j + 1,
                orientation: Orientation::Vertical,
            });
        }
    }

    refl_lines
}

fn find_horizontal_reflection_lines(
    pattern: &Vec<Vec<char>>,
    allowed_smudges: i64,
) -> Vec<ReflectionLine> {
    let mut refl_lines: Vec<ReflectionLine> = Vec::new();

    let rows: i64 = pattern.len() as i64;
    for i in 0..(rows - 1) {
        let mut smudge_count = 0;
        let mut k: i64 = 0;
        let mut diff = 0;
        while (0..rows).contains(&(i - k)) && (0..rows).contains(&(i + 1 + k)) {
            diff = diff_count(&pattern[(i - k) as usize], &pattern[(i + 1 + k) as usize]);
            if diff == 0 {
                k += 1;
            } else if diff == 1 && smudge_count < allowed_smudges {
                smudge_count += 1;
                k += 1;
            } else {
                break;
            }
        }
        // println!("i: {i} k: {k}");

        if i - k == -1
            || i + 1 + k == rows && (diff == 0 || (diff == 1 && smudge_count <= allowed_smudges))
        {
            // println!("Found vertical horizontal line in column {}", i + 1);
            refl_lines.push(ReflectionLine {
                index: i + 1,
                orientation: Orientation::Horizontal,
            });
        }
    }

    refl_lines
}

fn analyze(pattern: &Vec<Vec<char>>) -> i64 {
    let p1_v = find_vertical_reflection_lines(pattern, 0);
    let p1_h = find_horizontal_reflection_lines(pattern, 0);

    let p1_refl_line = if !p1_v.is_empty() {
        p1_v.first().unwrap()
    } else {
        p1_h.first().unwrap()
    };

    let p2_v = find_vertical_reflection_lines(pattern, 1);
    let p2_h = find_horizontal_reflection_lines(pattern, 1);

    let candidates = p2_v
        .into_iter()
        .chain(p2_h.into_iter())
        .filter(|relf_line| relf_line != p1_refl_line)
        .collect::<Vec<ReflectionLine>>();

    let p2_refl_line = if candidates.is_empty() {
        p1_refl_line
    } else {
        candidates.first().unwrap()
    };

    if p2_refl_line.orientation == Orientation::Vertical {
        p2_refl_line.index
    } else {
        100 * p2_refl_line.index
    }
}

fn p2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern_grid = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            analyze(&pattern_grid)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p2: {}", p2(&input));
}
