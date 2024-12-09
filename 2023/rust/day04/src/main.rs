use std::{collections::HashMap, fs};

fn parse_line(line: &str) -> Result<(Vec<u32>, Vec<u32>), &str> {
    if let [winning_numbers, personal_numbers] = line
        .split(": ")
        .nth(1)
        .expect("Expected two parts")
        .split(" | ")
        .map(|nums| {
            nums.split_whitespace()
                .map(|num| num.parse::<u32>().expect("Expected a number"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
        .as_slice()
    {
        Ok((winning_numbers.clone(), personal_numbers.clone()))
    } else {
        Err("Unexpected number of vectors in the input")
    }
}

fn get_matching(winning_numbers: Vec<u32>, personal_numbers: Vec<u32>) -> Option<Vec<u32>> {
    let result = personal_numbers
        .into_iter()
        .filter(|x| winning_numbers.contains(x))
        .collect::<Vec<u32>>();

    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line).expect("Error parsing line: {line}"))
        .map(|(winning_numbers, personal_numbers)| get_matching(winning_numbers, personal_numbers))
        .map(|matching_nums| {
            if let Some(nums) = matching_nums {
                2_u32.pow(nums.len() as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn p2(input: &str) -> u32 {
    let mut total = 0;
    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    let lines = input.lines().collect::<Vec<&str>>();
    for (i, line) in lines.iter().enumerate() {
        let card_id = i as u32 + 1;
        let (winning_numbers, personal_numbers) =
            parse_line(line).expect("Error parsing line: {line}");

        let matching_nums = get_matching(winning_numbers, personal_numbers);

        let amount_of_current_card = card_counts.get(&card_id).unwrap_or(&0) + 1;

        if let Some(nums) = matching_nums {
            let from = card_id+1;
            let to = card_id + nums.len() as u32;

            for j in from..to+1 {
                *card_counts.entry(j).or_insert(0) += amount_of_current_card;
            }
        }

        total += amount_of_current_card

    }

    // println!("{:#?}", card_counts);

    return total;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
