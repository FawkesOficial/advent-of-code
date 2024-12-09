use std::fs;

fn extract_number(line: &Vec<char>, index: usize) -> Option<u32> {

    let mut end_forward: usize = index;
    while end_forward < line.len() && line[end_forward].is_digit(10) {
        end_forward += 1;
    }

    let mut start_backward: usize = index;
    while start_backward > 0 && line[start_backward-1].is_digit(10) {
        start_backward -= 1;
    }

    let number_str: String = line[start_backward..end_forward].iter().collect();
    
    return number_str.parse::<u32>().ok();
}

fn p1(input: &str) -> u32 {

    let symbols: &str = "=*/-+%&@$#";

    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let rows: usize = lines.len();
    let cols: usize = lines.first().expect("Expected atleast one line").len();

    let mut part_numbers: Vec<u32> = Vec::new();
    for (r, line) in grid.iter().enumerate() {
        for (c, chr) in line.iter().enumerate() {
            if symbols.contains(*chr) {

                let mut checked_up: bool = false;
                let mut checked_down: bool = false;

                for (nr, nc) in vec![(r-1, c), (r+1, c), (r, c+1), (r, c-1), (r+1, c+1), (r-1, c-1), (r+1, c-1), (r-1, c+1)] {
                    if (0..rows).contains(&nr) && (0..cols).contains(&nc) && grid[nr][nc] != '.' {

                        if ( ( (nr, nc) == (r-1, c-1) || (nr, nc) == (r-1, c+1) ) && checked_up ) || ( ( (nr, nc) == (r+1, c-1) || (nr, nc) == (r+1, c+1) ) && checked_down ) {
                            continue;
                        }

                        if let Some(number) = extract_number(&grid[nr], nc) {
                            // println!("Extracted number: {}", number);
                            part_numbers.push(number);

                            if (nr, nc) == (r-1, c) {
                                checked_up = true;
                            }
                            if (nr, nc) == (r+1, c) {
                                checked_down = true;
                            }
                        }

                    }
                }
            }
        }
    }

    return part_numbers.iter().sum();
}

fn p2(input: &str) -> u32 {

    let lines = input.lines().collect::<Vec<&str>>();
    let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let rows = lines.len();
    let cols = lines.first().expect("Expected atleast one line").len();

    let mut gear_ratios: Vec<u32> = Vec::new();
    for (r, line) in grid.iter().enumerate() {
        for (c, chr) in line.iter().enumerate() {
            if chr == &'*' {

                let mut checked_up = false;
                let mut checked_down = false;

                let mut adjacent_numbers: Vec<u32> = Vec::new();

                for (nr, nc) in vec![(r-1, c), (r+1, c), (r, c+1), (r, c-1), (r+1, c+1), (r-1, c-1), (r+1, c-1), (r-1, c+1)] {
                    if (0..rows).contains(&nr) && (0..cols).contains(&nc) && grid[nr][nc] != '.' {

                        if ( ( (nr, nc) == (r-1, c-1) || (nr, nc) == (r-1, c+1) ) && checked_up ) || ( ( (nr, nc) == (r+1, c-1) || (nr, nc) == (r+1, c+1) ) && checked_down ) {
                            continue;
                        }

                        if let Some(number) = extract_number(&grid[nr], nc) {
                            // println!("Extracted number: {}", number);
                            adjacent_numbers.push(number);

                            if (nr, nc) == (r-1, c) {
                                checked_up = true;
                            }
                            if (nr, nc) == (r+1, c) {
                                checked_down = true;
                            }
                        }
                    }
                }

                if let [gear1, gear2] = &adjacent_numbers[..] {
                    gear_ratios.push(gear1 * gear2)
                }

                // Bad?
                // if adjacent_numbers.len() == 2 {
                //     gear_ratios.push(adjacent_numbers[0] * adjacent_numbers[1])
                // }
            }
        }
    }

    return gear_ratios.iter().sum();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
