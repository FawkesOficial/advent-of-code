use std::{collections::HashMap, fs};

fn p1(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut configuration: HashMap<&str, u32> = HashMap::new();
            let cube_sets = line.split(": ").collect::<Vec<&str>>()[1]
                .split("; ")
                .map(|cube_set| {
                    cube_set
                        .split(", ")
                        .map(|cube_info| {
                            let mut it = cube_info.split(" ");
                            let count: u32 = it
                                .next()
                                .expect("Expected a count")
                                .parse::<u32>()
                                .expect("Expected 'count' to be a number");
                            let color: &str = it.next().expect("Expected a color");

                            (count, color)
                        })
                        .collect::<Vec<(u32, &str)>>()
                })
                .collect::<Vec<Vec<(u32, &str)>>>();

            for cube_set in cube_sets {
                for (count, color) in cube_set {
                    let entry = configuration.entry(color).or_insert(count);
                    *entry = (*entry).max(count);
                }
            }

            (i as u32 + 1, configuration)
        })
        .filter(|(_id, config)| {
            *config.get("red").expect("Expected atleast one red cube") <= 12
                && *config.get("green").expect("Expected atleast one red cube") <= 13
                && *config.get("blue").expect("Expected atleast one blue cube") <= 14
        })
        .map(|(id, _config)| id)
        .sum()
}

fn p2(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut configuration: HashMap<&str, u32> = HashMap::new();
            let cube_sets = line.split(": ").collect::<Vec<&str>>()[1]
                .split("; ")
                .map(|cube_set| {
                    cube_set
                        .split(", ")
                        .map(|cube_info| {
                            let mut it = cube_info.split(" ");
                            let count: u32 = it
                                .next()
                                .expect("Expected a count")
                                .parse::<u32>()
                                .expect("Expected 'count' to be a number");
                            let color: &str = it.next().expect("Expected a color");

                            (count, color)
                        })
                        .collect::<Vec<(u32, &str)>>()
                })
                .collect::<Vec<Vec<(u32, &str)>>>();

            for cube_set in cube_sets {
                for (count, color) in cube_set {
                    let entry = configuration.entry(color).or_insert(count);
                    *entry = (*entry).max(count);
                }
            }

            (i as u32 + 1, configuration)
        })
        .map(|(_id, config)| {
            *config.get("red").expect("Expected atleast one red cube")
                * *config.get("green").expect("Expected atleast one red cube")
                * *config.get("blue").expect("Expected atleast one blue cube")
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
