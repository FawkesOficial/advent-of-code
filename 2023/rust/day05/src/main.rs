use std::{fs, ops::RangeInclusive};

#[derive(Debug)]
struct Mapping {
    source: RangeInclusive<i64>,
    dest: RangeInclusive<i64>,
}

impl Mapping {
    fn translate(&self, seed: &i64) -> Option<i64> {
        if self.source.contains(seed) {
            Some(*seed - self.source.start() + self.dest.start())
        } else {
            None
        }
    }
}

fn p1(input: &str) -> i64 {
    if let [seeds_str, blocks_str @ ..] = input.split("\n\n").collect::<Vec<&str>>().as_slice() {
        let seeds: Vec<i64> = seeds_str
            .split(": ")
            .nth(1)
            .expect("Expected seed numbers")
            .split(" ")
            .map(|num| num.parse::<i64>().expect("Expected a number"))
            .collect::<Vec<i64>>();

        let blocks = blocks_str
            .iter()
            .map(|block_str| {
                let mut block_lines = block_str.lines();
                block_lines.next();

                block_lines
                    .map(|line| {
                        let mut range_info = line
                            .split(" ")
                            .map(|num| num.parse::<i64>().expect("Expected a number"));

                        let dest_start = range_info.next().expect("Expected a first element");
                        let source_start = range_info.next().expect("Expected a second element");
                        let length = range_info.next().expect("Expected a third element");

                        Mapping {
                            source: source_start..=(source_start + length - 1),
                            dest: dest_start..=(dest_start + length - 1),
                        }
                    })
                    .collect::<Vec<Mapping>>()
            })
            .collect::<Vec<Vec<Mapping>>>();

        // println!("seeds: {:?}", seeds);
        // println!("blocks: {:?}", blocks[0]);

        seeds
            .iter()
            .map(|seed| {
                // println!("[SEED] Analyzing seed: {seed}");

                let mut new_seed = *seed;
                for block in blocks.as_slice() {
                    for mapping in block {
                        if let Some(result) = mapping.translate(&new_seed) {
                            new_seed = result;
                            break;
                        }
                        // println!("Mapped {seed} to {new_seed}!");
                    }
                }
                // println!("Mapped {seed} to {new_seed}!");
                new_seed
            })
            .min()
            .expect("Expected atleast one final value")
    } else {
        return 0;
    }
}

fn p2(input: &str) -> i64 {
    if let [seeds_str, blocks_str @ ..] = input.split("\n\n").collect::<Vec<&str>>().as_slice() {
        let seeds: Vec<RangeInclusive<i64>> = seeds_str
            .split(": ")
            .nth(1)
            .expect("Expected seed numbers")
            .split(" ")
            .map(|num| num.parse::<i64>().expect("Expected a number"))
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|pair| pair[0]..=(pair[0] + pair[1] - 1))
            .collect::<Vec<RangeInclusive<i64>>>();

        let blocks = blocks_str
            .iter()
            .map(|block_str| {
                let mut block_lines = block_str.lines();
                block_lines.next();

                block_lines
                    .map(|line| {
                        let mut range_info = line
                            .split(" ")
                            .map(|num| num.parse::<i64>().expect("Expected a number"));

                        let dest_start = range_info.next().expect("Expected a first element");
                        let source_start = range_info.next().expect("Expected a second element");
                        let length = range_info.next().expect("Expected a third element");

                        Mapping {
                            source: source_start..=(source_start + length - 1),
                            dest: dest_start..=(dest_start + length - 1),
                        }
                    })
                    .collect::<Vec<Mapping>>()
            })
            .collect::<Vec<Vec<Mapping>>>();

        // println!("seeds: {:?}", seeds);
        // println!("blocks: {:?}", blocks[0]);

        let mut current_translation = seeds.clone();

        for block in blocks.as_slice() {
            let mut new_translation = Vec::new();

            while !current_translation.is_empty() {
                let current_seed_range = current_translation.pop().unwrap();

                let mut intersected = false;
                for mapping in block {
                    let intersect_start = current_seed_range.start().max(mapping.source.start());
                    let intersect_end = current_seed_range.end().min(mapping.source.end());

                    if intersect_start < intersect_end {
                        new_translation.push(
                            mapping
                                .translate(intersect_start)
                                .expect("Expected direct translation")
                                ..=mapping
                                    .translate(intersect_end)
                                    .expect("Expected direct translation"),
                        );

                        if intersect_start > current_seed_range.start() {
                            current_translation
                                .push(*current_seed_range.start()..=*intersect_start);
                        }

                        if current_seed_range.end() > intersect_end {
                            current_translation.push(*intersect_end..=*current_seed_range.end());
                        }

                        intersected = true;
                        break;
                    }
                }

                if !intersected {
                    new_translation.push(current_seed_range.to_owned());
                }
            }

            current_translation = new_translation;
        }

        current_translation
            .into_iter()
            .map(|range| *range.start())
            .min()
            .unwrap()
    } else {
        panic!("Bad input format");
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
