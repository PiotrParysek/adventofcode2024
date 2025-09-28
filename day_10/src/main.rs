use std::{env, fs::File, io::{self, BufRead}};

fn main() {
    env::set_var("RUST_BACKTRACE", "full"); // Full backtrace 
    let file = read_file("input.txt".to_string());
    for row in &file {
        println!("{:?}", row);
    }
    // PART 1
    {
        let mut sum = 0;
        let x = file.len();
        let y = file[0].len();
        // println!("x: {}, y: {}", x, y);

        for i in 0..x {
            for j in 0..y {
                if file[i][j] == 0 {
                    let mut visited: Vec<(usize, usize)> = Vec::new();
                    sum += tailhead((i, j), 0, &file, &mut visited);
                }
            }
        }
        println!("{}", sum);
    }
    // PART 2
    {
        let mut sum = 0;
        let x = file.len();
        let y = file[0].len();
        for i in 0..x {
            for j in 0..y {
                if file[i][j] == 0 {
                    sum += path_count((i, j), 0, &file);
                }
            }
        }
        println!("{}", sum);
    }
}

// Calculate the tileheads - ergo paths form 0 to 9 that do not overlap.
fn tailhead((x, y): (usize, usize), current_height: u8,
             map: &Vec<Vec<u8>>, visited: &mut Vec<(usize, usize)>) -> usize {
    
    // Peak reached - if not yet visited add one.
    if current_height == 9 && !visited.contains(&(x, y)) {
        visited.push((x, y));
        return 1;
    }
    let mut sum = 0;

    // println!("(x, y): ({}, {}), current_height: {} - {}", x, y, current_height, !visited.contains(&(x, y)));
    // Check all possible moves and height
    if x > 0 && map[x-1][y] == current_height+1 {
        sum += tailhead((x-1, y), current_height+1, map, visited);
    }

    if x+1 < map.len() && map[x+1][y] == current_height+1 {
        sum += tailhead((x+1, y), current_height+1, map, visited);
    }

    if y > 0 && map[x][y-1] == current_height+1 {
        sum += tailhead((x, y-1), current_height+1, map, visited);
    }

    if y+1 < map[0].len() && map[x][y+1] == current_height+1 {
        sum += tailhead((x, y+1), current_height+1, map, visited);
    }
    
    sum
}

// Count all paths from 0 to 9 - paths may overlap.
fn path_count((x, y): (usize, usize), current_height: u8, map: &Vec<Vec<u8>>) -> usize {
    if current_height == 9 {
        return 1;
    }
    let mut sum = 0;

    // println!("(x, y) ({}, {}), current_height: {}", x, y, current_height);

    if x > 0 && map[x-1][y] == current_height+1 {
        sum += path_count((x-1, y), current_height+1, map);
    }

    if x+1 < map.len() && map[x+1][y] == current_height+1 {
        sum += path_count((x+1, y), current_height+1, map);
    }
    
    if y > 0 && map[x][y-1] == current_height+1 {
        sum += path_count((x, y-1), current_height+1, map);
    }

    if y+1 < map[0].len() && map[x][y+1] == current_height+1 {
        sum += path_count((x, y+1), current_height+1, map);
    }
    
    sum
}


// Read file -> I'm lazy...
fn read_file(file_name: String) -> Vec<Vec<u8>> {
    let input = File::open(&file_name).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);


    let res = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as u8 - b'0')
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    res
}