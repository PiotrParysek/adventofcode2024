use std::{env, mem, string};
use std::{fs::File};
use std::io::{self, BufRead};

fn main() {
    env::set_var("RUST_BACKTRACE", "full"); // Full backtrace 
    let file = read_file("input.txt".to_string()); // Read the input or test file
    let line = &file[0]; // Get the only line for the task
    println!("Line: {}", line);
    
    let num: usize = line.chars().filter(|&c| c != '\n')
        .filter_map(|c| Some(c.to_digit(10)? as usize)).sum(); // JIC count the number of characters
    println!("Num: {}", num);
    
    { // Part 1
        let mut res:Vec<Option<usize>> = Vec::new();
        let mut idx = 0; // Index
        let mut file_flag = true; // File is every second digit
        for c in line.chars() {
            if file_flag {
                let num = c.to_digit(10).unwrap();
                for _ in 0..num {
                    res.push(Some(idx)); // Some value -> Some
                }
                idx += 1;
                file_flag = false;
            } else { // Empty
                let num = c.to_digit(10).unwrap();
                for _ in 0..num {
                    res.push(None); // No value -> None
                }
                file_flag = true;
            }
        }
        print!("Res:"); // Print
        for n in &res {
            if n.is_none() {
                print!(".");
            } else {
                print!("{}", n.unwrap());
            }
        }
        println!();

        let mut left = 0; // to find the dots
        let mut right = res.len() - 1; // to find the numbers
        
        while left < right { // Continue until they meet
            
            // Find 'new' dot
            while left < right && res[left].is_some() {
                left += 1;
            }
            // Find 'new' number
            while left < right && res[right].is_none() {
                right -= 1;
            }
            // Swap dot with number
            res.swap(left, right);

            // Continue...
            left += 1;
            right -= 1;
        }

        // Lambda to calculate the sum of numbers * their index
        let part_1: usize = res
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| x.map(|x| x * i))
            .sum();
        println!("\nPart 1: {}", part_1);
    }
    {
        // Read the input into a vector of (Option<disk_id>, size) -> Important for Some and None and size of each 'chunk'
        let mut disk = line.chars().enumerate()
            .filter_map(|(i, c)| {
                c.to_digit(10).and_then(|d| {
                    if d == 0 {
                        None
                    } else if i % 2 == 0 {
                        Some((Some(i as u32 / 2), d))
                    } else {
                        Some((None, d))
                    }
                })
            })
            .into_iter()
            .collect::<Vec<(Option<u32>, u32)>>();
        // Fragment the disk
        let mut i = disk.len() - 1;
        while i > 1 {
            if let (Some(id), file) = disk[i] {
                for j in 0..i {
                    if let (None, free) = disk[j] {
                        // Free size has to be bigger then the file size
                        if free < file {
                            continue;
                        }
                        // Free is equal to file size - easy swap
                        if free == file {
                            disk.swap(i, j);
                        } else if free > file {
                        // Free is bigger then file size - split free into file and free
                            disk[j] = (Some(id), file);
                            disk.insert(j+1, (None, free - file));
                            i += 1;
                            disk[i] = (None, file);
                        }

                        let mut new_free = file;
                        if let (None, previous) = disk[i-1] {
                            new_free += previous;
                            disk[i] = (None, new_free);
                            disk.remove(i-1);
                            i -= 1;
                        }

                        if let Some((None, next)) = disk.get(i+1) {
                            new_free += next;
                            disk[i] = (None, new_free);
                            disk.remove(i+1);
                        }
                        break;
                    }
                }
            }
            i -= 1;
        }
        
        let mut sum: i64 = 0;
        let mut idx: i64 = 0;
        // Calculate the sum of disk_id * index
        for d in disk {
            if let (Some(id), size) = d {
                for _ in 0..size {
                    sum += (id as i64 * idx) as i64;
                    idx += 1;
                }
            } else if let (None, size) = d {
                idx += size as i64;
            }
        }
        println!("Part 2: {}", sum);
    }
}

// Read file -> I'm lazy...
fn read_file(file_name: String) -> Vec<String> {
    let input = File::open(&file_name).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);


    let res = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    res
}