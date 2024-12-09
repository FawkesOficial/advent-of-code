use std::{collections::HashMap, fs};

fn hash(step: &str) -> i64 {
    let mut current: i64 = 0;
    for c in step.chars() {
        current += c.to_ascii_lowercase() as i64;
        current *= 17;
        current %= 256;
    }

    current
}

fn p1(input: &str) -> i64 {
    input
        .lines()
        .next()
        .expect("Expected a first line")
        .split(",")
        .map(|step| hash(step))
        .sum()
}

fn p2(input: &str) -> i64 {
    let mut label_to_number: HashMap<&str, i64> = HashMap::new();
    let mut boxes: Vec<Vec<&str>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for step in input
        .lines()
        .next()
        .expect("Expected a first line")
        .split(",")
    {
        if step.contains('-') {
            let label = &step[..(step.len() - 1)];
            let index = hash(label);

            if boxes[index as usize].contains(&label) {
                boxes[index as usize].retain(|&x| x != label);
            }
        } else {
            let mut splt = step.split("=");
            let label = splt.next().unwrap();
            let length = splt.next().unwrap().parse::<i64>().unwrap();
            let index = hash(label);

            if !boxes[index as usize].contains(&label) {
                boxes[index as usize].push(label);
            }

            label_to_number.insert(label, length);
        }
    }

    // // More readable but less rusty™️©️ ??
    // let mut result = 0;
    // for (i, boxx) in boxes.iter().enumerate() {
    //     for (j, label) in boxx.iter().enumerate() {
    //         result += (i as i64 + 1) * (j as i64 + 1) * (label_to_number.get(label).unwrap());
    //     }
    // }
    // return result;

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, boxx)| {
            let label_to_number = &label_to_number;
            boxx.iter().enumerate().map(move |(j, label)| {
                (i as i64 + 1) * (j as i64 + 1) * (label_to_number.get(label).unwrap())
            })
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
