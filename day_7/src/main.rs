use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    println!("Day 7!");
    let file = read_file("input.txt".to_string());
    let mut vec: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in file {
        println!("l: {}", line);
        // let parts: Vec<&str> = line.split(":").collect();
        // println!("parts: {:?}", parts);
        // let res: i32 = parts[0].trim() // Remove white space
        //                        .parse()
        //                        .expect("how to parse i32");
        // println!("res: {}", res);
        // let vec_str = parts[1].split_whitespace()
        //                     .map(|x| x.parse().expect("Failed to parse to vec")) // Convert to i32
        //                     .collect::<VecDeque<i32>>();
        // println!("vec: {:?}", vec_str);
        let (res, num) = line.split_once(":").unwrap();
        let res = res.parse().unwrap();
        let num = num.split_whitespace().map(|x| x.parse().unwrap()).collect();
        vec.push((res, num));
    }
    // for (a, b) in vec {
    //     println!("a: {}, b: {:?}", a, b);
    // }

    // let mut part_1 = 0;
    // for (goal, numbers) in vec {
    //     let (start, numbers) = numbers.split_first().unwrap();
    //     if solve_part_1(*start, goal, numbers) {
    //         part_1 += goal;
    //     }
    // }
    // println!("Part 1: {}", part_1);

    let mut part_2 = 0;
    for (goal, numbers) in vec {
        let (start, numbers) = numbers.split_first().unwrap();
        if solve_part_2(*start, goal, numbers) {
            part_2 += goal;
        }
    }
    println!("Part 2: {}", part_2);
}

fn solve_part_1(result: u64, goal: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return result == goal;
    }
    if result > goal {
        return false;
    }
    let (first, rest) = numbers.split_first().unwrap();
    solve_part_1(result * first, goal, rest) || solve_part_1(result + first, goal, rest)
}

fn solve_part_2(result: u64, goal: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return result == goal;
    }
    if result > goal {
        return false;
    }
    let (first, rest) = numbers.split_first().unwrap();
    solve_part_2(concat_numbers(result, *first), goal, rest)
        || solve_part_2(result * first, goal, rest)
        || solve_part_2(result + first, goal, rest)
}

fn concat_numbers(num1: u64, num2: u64) -> u64 {
    let mut res = num1;
    let mut temp = num2;
    while temp > 0 {
        res *= 10;
        temp /= 10;
    }
    res + num2
}

fn read_file(file_name: String) -> Vec<String> {
    let input = File::open(&file_name).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);


    let res = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    res
}