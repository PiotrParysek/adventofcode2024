use std::fs::File;
use std::io::{self, BufRead};

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    println!("Day 6");
    // Path to the file
    let path = "input.txt"; // test.txt
    
    let puzzle = read_file(path.to_string());
    view_puzzle(&puzzle);

    let mut start_position = find_position(&puzzle);
    println!("XY: ({:?})", start_position.unwrap());

    let idk = play(&puzzle, start_position.unwrap());
    if let Some(final_puzzle) = idk {
        println!("Finished game?");
        // view_puzzle(&final_puzzle);
        println!("X Number: {}", count_x(&final_puzzle));
    }
}

fn read_file(file_name: String) -> Vec<Vec<char>> {
    let input = File::open(&file_name).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);


    let res = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    res
}

fn find_position(puzzle: &[Vec<char>]) -> Option<(isize, isize)> {
    for (row_idx, row) in puzzle.iter().enumerate() {

        if let Some(col_idx) = row.iter().position(|&ch| ch == '^') {
            return Some((row_idx as isize, col_idx as isize));
        }
    }
    None
}

fn play(puzzle: &[Vec<char>], start_position: (isize, isize)) -> Option<Vec<Vec<char>>> {
    let mut play_puzzle = puzzle.to_vec();
    let mut direction = 0; // direction of the player 0 - up, 1 - right, 2 - down, 3 - left
    let mut position = (start_position.0 as usize, start_position.1 as usize);
    loop {
        match direction {
            0 => {
                play_puzzle[position.0][position.1] = 'X';
                if (position.0 as isize) - 1 < 0 {
                    // Out of board -> end game
                    return Some(play_puzzle.to_vec());
                } else if play_puzzle[position.0-1][position.1] == '#' {
                    direction += 1;
                    continue;
                }
                position.0 -= 1;
            }
            1 => {
                play_puzzle[position.0][position.1] = 'X';
                if (position.1 + 1) == play_puzzle[position.0].len() {
                    // Out of board -> end game
                    return Some(play_puzzle.to_vec());
                } else if play_puzzle[position.0][position.1+1] == '#' {
                    direction += 1;
                    continue;
                }
                position.1 += 1;
            }
            2 => {
                play_puzzle[position.0][position.1] = 'X';
                if position.0 + 1 == play_puzzle.len() {
                    // Out of board -> end game
                    return Some(play_puzzle.to_vec());
                } else if play_puzzle[position.0+1][position.1] == '#' {
                    direction += 1;
                    continue;
                }
                position.0 += 1;
            }
            3 => {
                play_puzzle[position.0][position.1] = 'X';
                if (position.1 as isize) - 1 < 0 {
                    // Out of board -> end game
                    return Some(play_puzzle.to_vec());
                } else if play_puzzle[position.0][position.1-1] == '#' {
                    direction = 0;
                    continue;
                }
                position.1 -= 1;
            }
            _ => {
                println!("LOCATION: {:?} DIR: {}", position, direction);
                break;
            }
        }
        // println!("NEXT MOVE");
        // view_puzzle(&play_puzzle);
    }
    None
}

fn view_puzzle(puzzle: &[Vec<char>]) {
    for puz in puzzle {
        for p in puz {
            print!("{}", p);
        }
        println!();
    }
}

fn count_x(puzzle: &[Vec<char>]) -> i32 {
    let mut counter = 0;
    for puz in puzzle {
        for p in puz {
            if *p == 'X' {
                counter += 1;
            }
        }
    }
    counter
}
