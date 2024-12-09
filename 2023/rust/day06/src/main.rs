use std::fs;

fn p1(input: &str) -> i64 {
    let mut values = input.lines().map(|line| {
        line.split(": ")
            .nth(1)
            .expect("Expected an element after split")
            .split_whitespace()
            .map(|num| num.parse::<f64>().expect("Expected a number"))
            .collect::<Vec<f64>>()
    });

    let times = values.next().expect("");
    let distances = values.next().expect("");

    // println!("times: {times:?}");
    // println!("distances: {distances:?}");

    times
        .iter()
        .zip(distances.iter())
        .map(|(allowed_time, record_distance)| {
            let roots: (f64, f64) = (
                (allowed_time - (allowed_time * allowed_time - 4.0 * record_distance).sqrt()) / 2.0,
                (allowed_time + (allowed_time * allowed_time - 4.0 * record_distance).sqrt()) / 2.0,
            );

            let (start, mut end) = if roots.0 < roots.1 {
                (roots.0.ceil(), roots.1.ceil())
            } else {
                (roots.1.ceil(), roots.0.ceil())
            };

            if end == roots.1 {
                end -= 1.0;
            }

            (end - start) as i64
        })
        .fold(1, |accumulator, x| accumulator * x)
}

fn p2(input: &str) -> i64 {
    let mut values = input.lines().map(|line| {
        line.split(": ")
            .nth(1)
            .expect("Expected an element after split")
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<f64>().expect("Expected a number")
    });

    let time = values.next().expect("");
    let distance = values.next().expect("");

    // println!("times: {time:?}");
    // println!("distances: {distance:?}");

    
    let roots: (f64, f64) = (
        (time - (time * time - 4.0 * distance).sqrt()) / 2.0,
        (time + (time * time - 4.0 * distance).sqrt()) / 2.0,
    );

    let (start, mut end) = if roots.0 < roots.1 {
        (roots.0.ceil(), roots.1.ceil())
    } else {
        (roots.1.ceil(), roots.0.ceil())
    };

    if end == roots.1 {
        end -= 1.0;
    }

    (end - start) as i64

}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
