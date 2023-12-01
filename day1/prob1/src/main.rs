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
    let digits: Vec<i32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as i32)
        .collect();

    // concatenate first and last into a string
    let digit_string = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    let num: Result<i32, _> = digit_string.parse();

    match num {
        Ok(n) => n,
        Err(e) => {
            println!("Failed to parse: {}", e);
            0
        }
    }
}
