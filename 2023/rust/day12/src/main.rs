use std::{fs, collections::HashMap};

fn analyse(record: Vec<char>, nums: Vec<i64>, cache: &mut HashMap<(Vec<char>, Vec<i64>), i64>) -> i64 {
    // dbg!(&record);
    // dbg!(&nums);

    if record.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if record.contains(&'#') { 0 } else { 1 };
    }

    let key: (Vec<char>, Vec<i64>) = (record.clone(), nums.clone());

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut result: i64 = 0;

    if ['.', '?'].contains(&record.first().unwrap()) {
        result += analyse(record.get(1..).unwrap().to_vec(), nums.clone(), cache);
    }

    if ['#', '?'].contains(&record.first().unwrap()) {
        let first_num: usize = *nums.first().unwrap() as usize;
        if first_num <= record.len()
            && !record.get(..first_num).unwrap().contains(&'.')
            && (first_num == record.len() || *record.get(first_num).unwrap() != '#')
        {
            let new_record = match record.get(first_num + 1..) {
                Some(arr_slice) => arr_slice.to_vec(),
                None => {
                    let empty_vec: Vec<char> = Vec::new();
                    empty_vec
                }
            };

            result += analyse(new_record, nums.get(1..).unwrap().to_vec(), cache);
        }
    }

    cache.insert(key, result);

    result
}

fn p1(input: &str) -> i64 {
    let mut cache: HashMap<(Vec<char>, Vec<i64>), i64> = HashMap::new();
    
    input
        .lines()
        .map(|line| {
            let mut splt = line.split_whitespace();
            let record = splt.next().unwrap().chars().collect::<Vec<char>>();
            let nums = splt
                .next()
                .unwrap()
                .split(",")
                .filter_map(|num| num.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            analyse(record, nums, &mut cache)
        })
        .sum()
}

fn p2(input: &str) -> i64 {
    let mut cache: HashMap<(Vec<char>, Vec<i64>), i64> = HashMap::new();

    input
        .lines()
        .map(|line| {
            let mut splt = line.split_whitespace();
            let mut record = splt.next().unwrap().chars().collect::<Vec<char>>();
            let mut nums = splt
                .next()
                .unwrap()
                .split(",")
                .filter_map(|num| num.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            // let record = (0..5)
            //     .flat_map(|_| {
            //         ['?']
            //             .iter()
            //             .cloned()
            //             .chain(record.chars().collect::<Vec<char>>().iter().cloned())
            //             .collect::<Vec<char>>()
            //     })
            //     .collect::<String>()
            //     .chars()
            //     .collect::<Vec<char>>();
            // let nums = (0..5)
            //     .flat_map(|_| nums.iter().cloned())
            //     .collect::<Vec<i64>>();

            let cloned_record = record.clone();
            let cloned_nums = nums.clone();
            for _ in 0..4 {
                record.extend(['?'].iter().chain(cloned_record.iter()));
                nums.extend(cloned_nums.iter());
            }
            

            // dbg!(&record);
            // dbg!(&nums);

            analyse(record, nums, &mut cache)
        })
        .sum()
}



fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
