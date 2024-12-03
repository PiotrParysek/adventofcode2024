use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    println!("Day 3!");

    // let file_path = "test.txt";
    let file_path = "input.txt";

    let file = File::open(file_path).unwrap(); // Read file
    let reader = io::BufReader::new(file);

    // Regex: detect the "mul(D,D)" and "do()" and "don't()"
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do(?:n't)?\(\)").unwrap(); 
    // Regex: extract numbers
    let re_num = Regex::new(r"\d+").unwrap();

    let mut final_number = 0;
    let mut multiply:bool = true;

    for line in reader.lines().map(|s| s.unwrap()) {
        // println!("LINE: {}", line);
        let captured_values: Vec<&str> = re.find_iter(&line).map(|cn| cn.as_str()).collect();
        // println!("CN: {:?}", captured_values);
        for cn in captured_values {
            let c = &cn[..3];
            // based on three first characters allow/disable multiplication and multiply numbers, if it is 'allowed'
            match c {
                "don" => multiply = false,
                "do(" => multiply = true,
                "mul" => if multiply {
                    let numbers: Vec<i32> = re_num.find_iter(&cn).filter_map(|n| n.as_str().parse::<i32>().ok()).collect();
                    println!("NUM: {:?}", numbers);
                    final_number += numbers.iter().fold(1, |acc, &x| acc * x);
                },
                _ => println!("STH HAPPENED: {} - {}", cn, c)
            }
        }
    }
    println!("FN: {}", final_number);
}
