use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Day 2!");
    // let file_path = "test.txt";
    let file_path = "input.txt";

    let mut line_validator = 0;

    let file = File::open(file_path).unwrap(); // Read file
    let reader = io::BufReader::new(file);

    // Analyze line by line
    for line in reader.lines().map(|s| s.unwrap()) {
        // println!("{}", line);
        // Split line by whitespace, parse it into i32 and 'wrap' it into vec
        let level: Vec<i32> = line.split_whitespace()
        .map(|s| s.to_string().parse::<i32>().unwrap()).collect(); 
        // Check if the line is 'valid'
        if is_valid_line(&level) && is_valid_stable(&level) {
            // println!("Line: {} is valid", line);
            line_validator += 1;
        } else {
            // If line is invalid - remove ONE element
            let level_len = level.len();
            for i in 0..level_len {
                let mut tmp_level = level.clone();
                tmp_level.remove(i);
                if is_valid_line(&tmp_level) && is_valid_stable(&tmp_level) {
                    // println!("Line: {} is valid after remove index: {}", line, i);
                    line_validator += 1;
                    // If Removing one element 'solves' the riddle break the loop!
                    break;
                }
            }
        }
    }
    println!("Valid lines: {}", line_validator);
}

/// Check if difference between next two elements is 1, 2 or 3
fn is_valid_line(line: &[i32]) -> bool {
    line.windows(2).all(|pair| {
        if let [a, b] = pair {
            let diff = (a - b).abs();
            diff == 1 || diff == 2 || diff == 3
        } else {
            false
        }
    })
}

/// Check if the line is 'stable' ergo, decreasing or increasing
fn is_valid_stable(line: &[i32]) -> bool {
    let sign = (line[0] - line[1]).signum();
    line.windows(2).all(|pair| {
        if let [a, b] = pair {
            if sign == (a - b).signum() {
                true
            } else {
                false
            }
        } else {
            false      
        }
    }) 
}
