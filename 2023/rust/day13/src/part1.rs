use std::fs;

fn get_column(col_idx: i64, grid: &Vec<Vec<char>>) -> Vec<char> {
    return (0..grid.len())
        .map(|i| grid[i][col_idx as usize])
        .collect::<Vec<char>>();
}

fn find_vertical_reflection_line(pattern: &Vec<Vec<char>>) -> Option<i64> {
    let cols: i64 = pattern.first().expect("Expected a first line").len() as i64;

    for j in 0..cols {
        let mut k: i64 = 0;

        while ((0..cols).contains(&(j - k + 1)) && (0..cols).contains(&(j + k)))
            && get_column(j - k + 1, pattern) == get_column(j + k, pattern)
        {
            k += 1;
        }

        if j - k + 1 == -1 || j + k == cols {
            // println!("Found vertical reflection line in column {}", j + 1);
            return Some(j + 1);
        }
    }

    None
}

fn find_horizontal_reflection_line(pattern: &Vec<Vec<char>>) -> Option<i64> {
    let rows: i64 = pattern.len() as i64;

    for i in 0..rows {
        // println!("i: {i}");

        let mut k: i64 = 0;

        while ((0..rows).contains(&(i - k + 1)) && (0..rows).contains(&(i + k)))
            && pattern[(i - k + 1) as usize] == pattern[(i + k) as usize]
        {
            k += 1;
        }
        // println!("i: {i} k: {k}");

        if i - k + 1 == -1 || i + k == rows {
            // println!("Found horizontal reflection line in row {}", i + 1);
            return Some(i + 1);
        }
    }

    None
}

fn analyze(pattern: &Vec<Vec<char>>) -> i64 {
    return find_vertical_reflection_line(pattern)
        .unwrap_or_else(|| 100*find_horizontal_reflection_line(pattern)
        .expect("Expected to find a horizontol reflection line if a vertical reflection line was not previously found"));
}

fn p1(input: &str) -> i64 {
    return input
        .split("\n\n")
        .map(|pattern| {
            let pattern_grid = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            analyze(&pattern_grid)
        })
        .sum();
}

// fn p2(input: &str) -> i64 {

// }

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    // println!("p2: {}", p2(&input));
}
