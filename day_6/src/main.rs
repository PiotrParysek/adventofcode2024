use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    println!("Day 6");
    // Path to the file
    let path = "test.txt"; // input.txt
    
    let puzzle = read_file(path.to_string());
    view_puzzle(&puzzle);

    let start_position = find_position(&puzzle);
    println!("XY: ({:?})", start_position.unwrap());

    let (idk, test) = play(&puzzle, start_position.unwrap());
    if let Some(final_puzzle) = idk {
        println!("Finished game?");
        // view_puzzle(&final_puzzle);
        println!("X Number: {}", count_x(&final_puzzle));
    }

    if let Some(hashtag_coordinates) = test {
        println!("Hashtags: {:?}", hashtag_coordinates);
    }
}

/**
 * Read file -> return the content of it.
 */
fn read_file(file_name: String) -> Vec<Vec<char>> {
    let input = File::open(&file_name).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);


    let res = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    res
}

/**
 * Find the position of '^' in the puzzle
 */
fn find_position(puzzle: &[Vec<char>]) -> Option<(isize, isize)> {
    for (row_idx, row) in puzzle.iter().enumerate() {

        if let Some(col_idx) = row.iter().position(|&ch| ch == '^') {
            return Some((row_idx as isize, col_idx as isize));
        }
    }
    None
}

/**
 * Play the game until the player goes out of the board or there is no more moves.
 * Return the final state of the puzzle and the coordinates of all found '#'.
 */
fn play(puzzle: &[Vec<char>], start_position: (isize, isize)) -> (Option<Vec<Vec<char>>>, Option<Vec<(usize, usize)>>) {
    let mut play_puzzle = puzzle.to_vec(); // Puzzle ?
    let mut direction = 0; // direction of the player 0 - up, 1 - right, 2 - down, 3 - left
    let mut position = (start_position.0 as usize, start_position.1 as usize);
    let mut hashtag_puzzle= Vec::new(); // 
    loop {
        match direction {
            0 => {
                play_puzzle[position.0][position.1] = 'X';
                if (position.0 as isize) - 1 < 0 {
                    // Out of board -> end game
                    return (Some(play_puzzle.to_vec()), Some(hashtag_puzzle));
                } else if play_puzzle[position.0-1][position.1] == '#' {
                    hashtag_puzzle.push((position.0-1, position.1));
                    direction += 1;
                    continue;
                }
                position.0 -= 1;
            }
            1 => {
                play_puzzle[position.0][position.1] = 'X';
                if (position.1 + 1) == play_puzzle[position.0].len() {
                    // Out of board -> end game
                    return (Some(play_puzzle.to_vec()), Some(hashtag_puzzle));
                } else if play_puzzle[position.0][position.1+1] == '#' {
                    hashtag_puzzle.push((position.0, position.1+1));
                    direction += 1;
                    continue;
                }
                position.1 += 1;
            }
            2 => {
                play_puzzle[position.0][position.1] = 'X';
                if position.0 + 1 == play_puzzle.len() {
                    // Out of board -> end game
                    return (Some(play_puzzle.to_vec()), Some(hashtag_puzzle));
                } else if play_puzzle[position.0+1][position.1] == '#' {
                    hashtag_puzzle.push((position.0+1, position.1));
                    direction += 1;
                    continue;
                }
                position.0 += 1;
            }
            3 => {
                play_puzzle[position.0][position.1] = 'X';
                if (position.1 as isize) - 1 < 0 {
                    // Out of board -> end game
                    return (Some(play_puzzle.to_vec()), Some(hashtag_puzzle));
                } else if play_puzzle[position.0][position.1-1] == '#' {
                    hashtag_puzzle.push((position.0, position.1-1));
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
    (None, None)
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

/*
 EXAMPLE:
  .#..
  ...#
  .^..
  ..#.

  1. Find all the '#' and save it's location (x,y)
  2. Detect the loops:
  2.1. For each found '#' (x0, y0) check if there is a '#' with location (x0-1, y>y0) as (x1, y1)
  2.2. If there is, find a '#' (x>x1, y1-1) as (x2, y2)
  2.3. Place 'O' [new '#'] in (x2+1, y0-1)
  2.4. TaDa loop counter += 1
 */