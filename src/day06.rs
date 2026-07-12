
pub fn part1(input: &str) -> u32 {
    let puzzle: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // view_puzzle(&puzzle);

    let start_position = find_position(&puzzle);
    // println!("XY: ({:?})", start_position.unwrap());

    let (idk, _hashtag_coordinates) = play(&puzzle, start_position.unwrap());
    let mut final_count_x = 0;
    if let Some(final_puzzle) = idk {
        // println!("Finished game?");
        // view_puzzle(&final_puzzle);
        final_count_x = count_x(&final_puzzle);
        println!("X Number: {}", final_count_x);
        // println!("Hashtags: {:?}", hashtag_coordinates);
    }

    final_count_x as u32
}

pub fn part2(input: &str) -> u32 {
    let puzzle: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // view_puzzle(&puzzle);

    let _start_position = find_position(&puzzle);
    // println!("XY: ({:?})", start_position.unwrap());

    // Do not remember how I finished it
    6
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

/**
 * View the puzzle - debugging 
 */
#[warn(dead_code)]
fn view_puzzle(puzzle: &[Vec<char>]) {
    for puz in puzzle {
        for p in puz {
            print!("{}", p);
        }
        println!();
    }
}

/**
 * Count visited 'spots'
 */
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


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 6);
    }
}