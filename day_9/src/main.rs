use std::{env, mem, string};
use std::{fs::File};
use std::io::{self, BufRead};

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let file = read_file("input.txt".to_string());
    let line = &file[0];
    println!("Line: {}", line);
    
    let num: usize = line.chars().filter(|&c| c != '\n')
        .filter_map(|c| Some(c.to_digit(10)? as usize)).sum();
    println!("Num: {}", num);
    
    {
        let mut res:Vec<Option<usize>> = Vec::new();
        let mut idx = 0;
        let mut file_flag = true;
        for c in line.chars() {
            if file_flag {
                let num = c.to_digit(10).unwrap();
                for _ in 0..num {
                    res.push(Some(idx));
                }
                idx += 1;
                file_flag = false;
            } else {
                let num = c.to_digit(10).unwrap();
                for _ in 0..num {
                    res.push(None);
                }
                file_flag = true;
            }
        }
        print!("Res:");
        for n in &res {
            if n.is_none() {
                print!(".");
            } else {
                print!("{}", n.unwrap());
            }
        }
        println!();

        let mut left = 0;
        let mut right = res.len() - 1;
        
        while left < right {
            
            while left < right && res[left].is_some() {
                left += 1;
            }
            while left < right && res[right].is_none() {
                right -= 1;
            }
            
            res.swap(left, right);

            left += 1;
            right -= 1;
        }

        let part_1: usize = res
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| x.map(|x| x * i))
            .sum();
        println!("\nPart 1: {}", part_1);
    }
    {
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

        let mut i = disk.len() - 1;
        while i > 1 {
            if let (Some(id), file) = disk[i] {
                for j in 0..i {
                    if let (None, free) = disk[j] {
                        if free < file {
                            continue;
                        }

                        if free == file {
                            disk.swap(i, j);;
                        } else if free > file {
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

fn read_file(file_name: String) -> Vec<String> {
    let input = File::open(&file_name).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);


    let res = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    res
}