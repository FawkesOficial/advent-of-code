use std::{
    collections::{HashSet, VecDeque},
    fs,
};

const GOES_UP: [char; 4] = ['S', '|', 'J', 'L'];
const TAKES_UP: [char; 3] = ['|', '7', 'F'];
const GOES_DOWN: [char; 4] = ['S', '|', '7', 'F'];
const TAKES_DOWN: [char; 4] = ['S', '|', 'J', 'L'];
const GOES_LEFT: [char; 4] = ['S', '-', 'J', '7'];
const TAKES_LEFT: [char; 3] = ['-', 'L', 'F'];
const GOES_RIGHT: [char; 4] = ['S', '-', 'L', 'F'];
const TAKES_RIGHT: [char; 3] = ['-', 'J', '7'];

fn get_start_pos(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some((i, j));
        }
    }

    None
}

fn explore(
    pos: (usize, usize),
    seen: &HashSet<(usize, usize)>,
    s_aprox: &HashSet<char>,
    grid: &Vec<Vec<char>>,
) -> (Vec<(usize, usize)>, HashSet<char>) {
    let mut new_nodes: Vec<(usize, usize)> = Vec::new();
    let mut new_s_aprox: HashSet<char> = s_aprox.clone();

    let rows: usize = grid.len();
    let cols: usize = grid.first().unwrap().len();

    let (r, c) = pos;
    let pos_char: char = grid[r][c];

    if r > 0 && GOES_UP.contains(&pos_char) {
        let above: (usize, usize) = (r - 1, c);
        if TAKES_UP.contains(&grid[above.0][above.1]) && !seen.contains(&above) {
            new_nodes.push(above);
            if pos_char == 'S' {
                new_s_aprox = new_s_aprox
                    .intersection(&HashSet::from(['|', 'J', 'L']))
                    .cloned()
                    .collect::<HashSet<char>>();
            }
        }
    }

    if r < rows - 1 && GOES_DOWN.contains(&pos_char) {
        let below: (usize, usize) = (r + 1, c);
        if TAKES_DOWN.contains(&grid[below.0][below.1]) && !seen.contains(&below) {
            new_nodes.push(below);
            if pos_char == 'S' {
                new_s_aprox = new_s_aprox
                    .intersection(&HashSet::from(['|', '7', 'F']))
                    .cloned()
                    .collect::<HashSet<char>>();
            }
        }
    }

    if c > 0 && GOES_LEFT.contains(&pos_char) {
        let left: (usize, usize) = (r, c - 1);
        if TAKES_LEFT.contains(&grid[left.0][left.1]) && !seen.contains(&left) {
            new_nodes.push(left);
            if pos_char == 'S' {
                new_s_aprox = new_s_aprox
                    .intersection(&HashSet::from(['-', 'J', '7']))
                    .cloned()
                    .collect::<HashSet<char>>();
            }
        }
    }

    if c < cols - 1 && GOES_RIGHT.contains(&pos_char) {
        let right: (usize, usize) = (r, c + 1);
        if TAKES_RIGHT.contains(&grid[right.0][right.1]) && !seen.contains(&right) {
            new_nodes.push(right);
            if pos_char == 'S' {
                new_s_aprox = new_s_aprox
                    .intersection(&HashSet::from(['-', 'L', 'F']))
                    .cloned()
                    .collect::<HashSet<char>>();
            }
        }
    }

    (new_nodes, new_s_aprox)
}

fn p1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_pos: (usize, usize) =
        get_start_pos(&grid).expect("Expected to have a start position");

    // dbg!(start_pos);

    let mut explored: HashSet<(usize, usize)> = HashSet::from([start_pos]);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([start_pos]);

    while !queue.is_empty() {
        let current_pos: (usize, usize) = queue.pop_front().unwrap();

        for new_pos in explore(current_pos, &explored, &HashSet::<char>::new(), &grid).0 {
            explored.insert(new_pos);
            queue.push_back(new_pos);
        }
    }

    explored.len() as i64 / 2
}

fn p2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_pos: (usize, usize) =
        get_start_pos(&grid).expect("Expected to have a start position");

    // dbg!(start_pos);

    let mut explored: HashSet<(usize, usize)> = HashSet::from([start_pos]);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([start_pos]);

    let mut s_aprox: HashSet<char> = HashSet::from(['|', '-', 'J', 'L', '7', 'F']);

    while !queue.is_empty() {
        let current_pos: (usize, usize) = queue.pop_front().unwrap();

        let (to_explore, new_s_aprox) = explore(current_pos, &explored, &s_aprox, &grid);
        s_aprox = new_s_aprox;

        for new_pos in to_explore {
            explored.insert(new_pos);
            queue.push_back(new_pos);
        }
    }

    // dbg!(s_aprox);

    let mut clean_grid = grid.clone();
    clean_grid[start_pos.0][start_pos.1] = *s_aprox.iter().next().unwrap();
    clean_grid = clean_grid
        .into_iter()
        .enumerate()
        .map(|(r, line)| {
            line.into_iter()
                .enumerate()
                .map(|(c, chr)| if explored.contains(&(r, c)) { chr } else { '.' })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // for line in clean_grid.iter() {
    //     println!("{}", line.iter().collect::<String>());
    // }

    let outside: HashSet<(usize, usize)> = clean_grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            let mut within: bool = false;
            let mut up: bool = false;

            row.iter().enumerate().filter_map(move |(c, &chr)| {
                match chr {
                    '|' => {
                        within = !within;
                    }
                    'L' | 'F' => {
                        up = chr == 'L';
                    }
                    '7' | 'J' => {
                        if chr != if up { 'J' } else { '7' } {
                            within = !within;
                        }
                        up = false;
                    }
                    _ => {}
                }

                if !within {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<(usize, usize)>>();

    (clean_grid.len() * clean_grid.first().unwrap().len()
        - outside
            .union(&explored)
            .cloned()
            .collect::<HashSet<(usize, usize)>>()
            .len()) as i64
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
