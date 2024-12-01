
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // println!("Day 1");
    let file_path = "input.txt"; // File path

    
    let mut left: Vec<i32> = Vec::new(); // 'Left' numbers
    let mut right: Vec<i32> = Vec::new(); // 'Right' numbers

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            // println!("{}", line); // Print lines
            let (l,r) = line.split_once(char::is_whitespace).unwrap(); // Split lines 
            // println!("l:{} - r:{}", l.trim(), r.trim());
            left.push(l.trim().parse().unwrap()); // parse into vector
            right.push(r.trim().parse().unwrap()); // parse into vector
        }
    }
    solution_1(left.clone(), right.clone()); // Solution Task 1
    solution_2(left.clone(), right.clone()); // Solution Task 2
}

fn solution_1(left_v: Vec<i32>, right_v: Vec<i32>) -> i32 {
    // println!("l: {:?} - r: {:?}", left_v, right_v);
    let mut left = left_v.clone();
    let mut right = right_v.clone();
    left.sort(); // Sort vector
    right.sort(); // Sort vector
    // println!("r: {:?}", right);
    // println!("l: {:?}", left);

    // Check if the vectors are the same 
    if left.len() != right.len() {
        eprintln!("Vectors are not the same length! {} <-> {}", left.len(), right.len());
        -1
    } else {
        // Calculate a sum of differences for sorted vectors l
        let diff: Vec<i32> = (0..left.len()).map(|i| i32::abs(right[i] - left[i])).collect(); 
        // println!("D: {:?}", diff);
        let sum: i32 = diff.iter().sum(); // Calculate the sum
        println!("SUM: {}", sum);
        sum
    }
}

fn solution_2(left_v: Vec<i32>, right_v: Vec<i32>) -> i32 {
    let mut left = left_v.clone(); // Clone vector
    let right = right_v.clone(); // Clone vector

    // Check if the vectors are the same 
    if left.len() != right.len() {
        eprintln!("Vectors are not the same length! {} <-> {}", left.len(), right.len());
        -1
    } else {
        let mut sum: i32 = 0;
        for i in left.iter_mut() {
            sum += *i * (right.iter().filter(|&x| *x == *i).count() as i32);
        }
        println!("SUM: {}", sum);
        sum
    }
}

/// Read file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
