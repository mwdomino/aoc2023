use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "input.txt";
    let lines = read_lines(path);
    let mut solution = 0;

    for line in lines {
        solution += extract_digits(&line);
    }

    println!("Solution: {}", solution)
}

fn read_lines(path: &str) -> Vec<String> {
    let file_contents = fs::read_to_string(path).expect("Failed to read from file");
    file_contents.lines().map(String::from).collect()
}

fn extract_digits(line: &str) -> i32 {
    let translations: HashMap<&str, i32> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect();

    let substrings: Vec<&str> = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    // find all substrings
    let mut positions = Vec::new();
    for substring in substrings {
        let mut start = 0;
        while let Some(found) = line[start..].find(substring) {
            let actual_pos = start + found;
            positions.push((actual_pos, substring));
            start = actual_pos + 1
        }
    }

    // sort substrings by order
    positions.sort_by_key(|&(pos, _)| pos);

    // extract into digits
    let mut digits: Vec<i32> = Vec::new();
    for pos in positions {
        let val = pos.1;

        if let Some(digit) = translations
            .get(val)
            .copied()
            .or_else(|| val.parse::<i32>().ok())
        {
            digits.push(digit);
        }
    }

    // concatenate first and last into a string
    let digit_string = match (digits.first(), digits.last()) {
        (Some(first), Some(last)) => format!("{}{}", first, last),
        _ => "".to_string(),
    };

    digit_string.parse::<i32>().unwrap_or(0)
}
