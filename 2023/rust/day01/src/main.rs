use std::fs;

fn p1(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10)
            .unwrap())
            .collect();

        let first_digit: &u32 = digits.first().expect("Expected atleast one digit");
        let last_digit: &u32  = digits.last().unwrap_or(first_digit);

        total += first_digit*10 + last_digit;
    }
    
    return total;
}

fn p2(input: &str) -> u32 {

    let numstr_to_digit = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    let mut total: u32 = 0;
    for line in input.lines() {
        let mut digits: Vec<u32> = vec![];
        
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
            } else {
                for (numstr, digit) in numstr_to_digit.iter() {
                    if line[i..].starts_with(numstr) {
                        digits.push(*digit);
                        break;
                    }
                }
            }
        }

        let first_digit: &u32 = digits.first().expect("Expected atleast one digit");
        let last_digit: &u32  = digits.last().unwrap_or(first_digit);

        total += first_digit*10 + last_digit;
    }
    
    return total;
}

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Error while reading input file");

    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));

}
