use itertools::Itertools;
use std::{collections::HashSet, fs};

struct Point {
    r: i64,
    c: i64,
}

fn is_empty_space(row_col: &Vec<char>) -> bool {
    return !row_col.contains(&'#');
}

fn count_crossings(idx: usize, empty_spaces: &HashSet<usize>) -> i64 {
    return empty_spaces.iter().filter(|&&x| x < idx).count() as i64;
}

fn shortest_path(p1: &Point, p2: &Point) -> i64 {
    return (p1.r.abs_diff(p2.r) + p1.c.abs_diff(p2.c)) as i64;
}

fn p1(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    const EXPANSION_FACTOR: i64 = 2;

    let columns: Vec<Vec<char>> = (0..grid
        .first()
        .expect("Expected the grid to have a first line")
        .len())
        .map(|col| grid.iter().map(|row| row[col]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(r, row)| if is_empty_space(row) { Some(r) } else { None })
        .collect::<HashSet<usize>>();

    let empty_cols = columns
        .iter()
        .enumerate()
        .filter_map(|(c, col)| if is_empty_space(col) { Some(c) } else { None })
        .collect::<HashSet<usize>>();

    let galaxies: Vec<Point> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            let empty_rows_clone = empty_rows.clone();
            let empty_cols_clone = empty_cols.clone();

            line.iter()
                .enumerate()
                .filter(|(_, &chr)| chr == '#')
                .map(move |(c, _)| {
                    let row_crossings = count_crossings(r, &empty_rows_clone);
                    let scaled_row = if row_crossings != 0 {
                        r as i64 + (EXPANSION_FACTOR - 1) * row_crossings
                    } else {
                        r as i64
                    };
                    let col_crossings = count_crossings(c, &empty_cols_clone);
                    let scaled_col = if col_crossings != 0 {
                        c.to_owned() as i64 + (EXPANSION_FACTOR - 1) * col_crossings
                    } else {
                        c.to_owned() as i64
                    };

                    Point {
                        r: scaled_row,
                        c: scaled_col,
                    }
                })
        })
        .collect::<Vec<Point>>();

    return galaxies
        .iter()
        .combinations(2)
        .map(|pair| shortest_path(pair[0], pair[1]))
        .sum();
}

fn p2(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    const EXPANSION_FACTOR: i64 = 1000000;

    let columns: Vec<Vec<char>> = (0..grid
        .first()
        .expect("Expected the grid to have a first line")
        .len())
        .map(|col| grid.iter().map(|row| row[col]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(r, row)| if is_empty_space(row) { Some(r) } else { None })
        .collect::<HashSet<usize>>();

    let empty_cols = columns
        .iter()
        .enumerate()
        .filter_map(|(c, col)| if is_empty_space(col) { Some(c) } else { None })
        .collect::<HashSet<usize>>();

    let galaxies: Vec<Point> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            let empty_rows_clone = empty_rows.clone();
            let empty_cols_clone = empty_cols.clone();

            line.iter()
                .enumerate()
                .filter(|(_, &chr)| chr == '#')
                .map(move |(c, _)| {
                    let row_crossings = count_crossings(r, &empty_rows_clone);
                    let scaled_row = if row_crossings != 0 {
                        r as i64 + (EXPANSION_FACTOR - 1) * row_crossings
                    } else {
                        r as i64
                    };
                    let col_crossings = count_crossings(c, &empty_cols_clone);
                    let scaled_col = if col_crossings != 0 {
                        c.to_owned() as i64 + (EXPANSION_FACTOR - 1) * col_crossings
                    } else {
                        c.to_owned() as i64
                    };

                    Point {
                        r: scaled_row,
                        c: scaled_col,
                    }
                })
        })
        .collect::<Vec<Point>>();

    return galaxies
        .iter()
        .combinations(2)
        .map(|pair| shortest_path(pair[0], pair[1]))
        .sum();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
