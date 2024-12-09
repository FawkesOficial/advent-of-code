use std::{collections::HashMap, fs};
use num::integer::lcm;

fn p1(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut instructions = lines
        .next()
        .expect("Expected a first line")
        .chars()
        // .map(|c| if c == 'L' { 0 } else { 1 })
        .cycle();

    // Ignore line
    lines.next();

    let graph = lines
        .map(|line| {
            let mut splt = line.split(" = ");
            let node = splt.next().expect("Expected a node");
            let edges = splt
                .next()
                .expect("Expected edges")
                .replace("(", "")
                .replace(")", "")
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (
                node.to_string(),
                (
                    edges.get(0).expect("Expected a first node").to_owned(),
                    edges.get(1).expect("Expected a second node").to_owned(),
                ),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    // println!("{instructions:?}");
    // println!("{graph:?}");

    let mut steps: i64 = 0;
    let mut current_node: String = "AAA".to_string();
    while current_node != "ZZZ" {
        let edges = graph.get(&current_node).expect("Key/node not in graph");
        let next_instruction = instructions.next().expect("Expected an instruction");
        current_node = if next_instruction == 'L' {edges.0.to_owned()} else {edges.1.to_owned()};
        steps += 1;
    }

    return steps;
}

fn p2(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut instructions = lines
        .next()
        .expect("Expected a first line")
        .chars()
        // .map(|c| if c == 'L' { 0 } else { 1 })
        .cycle();

    // Ignore line
    lines.next();

    let graph = lines
        .map(|line| {
            let mut splt = line.split(" = ");
            let node = splt.next().expect("Expected a node");
            let edges = splt
                .next()
                .expect("Expected edges")
                .replace("(", "")
                .replace(")", "")
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (
                node.to_string(),
                (
                    edges.get(0).expect("Expected a first node").to_owned(),
                    edges.get(1).expect("Expected a second node").to_owned(),
                ),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    // println!("{instructions:?}");
    // println!("{graph:?}");

    let starts = graph.keys().filter(|node| node.ends_with("A")).collect::<Vec<&String>>();
    
    let mut step_info = starts.iter().map(|start_node| {
        let mut steps: i64 = 0;
        let mut current_node: String = (*start_node).to_owned();
        while !current_node.ends_with("Z") {
            let edges = graph.get(&current_node).expect("Key/node not in graph");
            let next_instruction = instructions.next().expect("Expected an instruction");
            current_node = if next_instruction == 'L' {edges.0.to_owned()} else {edges.1.to_owned()};
            steps += 1;
        }
        
        steps
    });

    let init: i64 = step_info.next().expect("Expected a first value");
    
    step_info.fold(init, |accumulator, x| lcm(accumulator, x))

}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}
