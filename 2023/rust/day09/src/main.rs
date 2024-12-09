use std::fs;

fn predict(history: Vec<i64>) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![history.clone()];
    let mut current_diff: Vec<i64> = history.clone();

    while diffs
        .last()
        .expect("Expected a last diff")
        .iter()
        .any(|&num| num != 0)
    {
        current_diff = current_diff
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<i64>>();

        diffs.push(current_diff.clone());
        // println!("{current_diff:?}");
        // break;
    }

    let relevant_size = diffs.len() - 2;
    let penultimate_diff = diffs
        .get_mut(relevant_size)
        .expect("Expected a penultimate diff to exist");

    penultimate_diff.push(
        *(penultimate_diff
            .get(penultimate_diff.len() - 1)
            .expect("Expected the penultimate diff to have a last element")),
    );

    // for pair in diffs.iter().rev().collect::<Vec<&Vec<i64>>>().chunks(2) {
    //     *pair[0].push(*pair[1].last().expect("Expected a last"))
    // }

    // println!("penultimate_diff: {penultimate_diff:?}");

    let mut i = relevant_size;
    while i > 0 {
        let diffs_clone = diffs.clone();
        let da_diff = diffs.get_mut(i - 1).expect("Expected a diff");

        da_diff.push(
            diffs_clone
                .get(i)
                .expect("Expected a diff")
                .to_owned()
                .last()
                .expect("Expected a last element")
                + da_diff
                    .last()
                    .expect("Expected da_diff to have a last element"),
        );

        i -= 1;
    }

    // println!("{history:?}");
    // println!("DIFFS: {diffs:?}");

    return *diffs
        .first()
        .expect("Expected a first diff to exist")
        .last()
        .expect("Expected the first diff to have a last element");
}

fn predict_backwards(history: Vec<i64>) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![history.clone()];
    let mut current_diff: Vec<i64> = history.clone();

    while diffs
        .last()
        .expect("Expected a last diff")
        .iter()
        .any(|&num| num != 0)
    {
        current_diff = current_diff
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<i64>>();

        diffs.push(current_diff.clone());
        // println!("{current_diff:?}");
        // break;
    }

    let relevant_size = diffs.len() - 2;
    let penultimate_diff = diffs
        .get_mut(relevant_size)
        .expect("Expected a penultimate diff to exist");

    penultimate_diff.insert(
        0,
        *penultimate_diff.first().expect("Expected a first element"),
    );

    // for pair in diffs.iter().rev().collect::<Vec<&Vec<i64>>>().chunks(2) {
    //     *pair[0].push(*pair[1].last().expect("Expected a last"))
    // }

    // println!("penultimate_diff: {penultimate_diff:?}");

    let mut i = relevant_size;
    while i > 0 {
        let diffs_clone = diffs.clone();
        let da_diff = diffs.get_mut(i - 1).expect("Expected a diff");

        da_diff.insert(
            0,
            *da_diff
                .first()
                .expect("Expected da_diff to have a first element")
                - diffs_clone
                    .get(i)
                    .expect("Expected a diff")
                    .first()
                    .expect("Expected a first element"),
        );

        i -= 1;
    }

    // println!("{history:?}");
    // println!("DIFFS: {diffs:?}");

    return *diffs
        .first()
        .expect("Expected a first diff to exist")
        .first()
        .expect("Expected the first diff to have a first element");
}

fn p1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let history = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Expected a number"))
                .collect::<Vec<i64>>();

            predict(history)
        })
        .sum()
}

fn p2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let history = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Expected a number"))
                .collect::<Vec<i64>>();

            predict_backwards(history)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
