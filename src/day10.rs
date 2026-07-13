// Template for Day XY of Advent of Code 2024

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let file = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
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
    sum as u32
}

pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let file = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let x = file.len();
    let y = file[0].len();
    // println!("x: {}, y: {}", x, y);
    for i in 0..x {
        for j in 0..y {
            if file[i][j] == 0 {
                sum += path_count((i, j), 0, &file);
            }
        }
    }
    println!("{}", sum);
    sum as u32
}

// Calculate the tileheads - ergo paths form 0 to 9 that do not overlap.
fn tailhead(
    (x, y): (usize, usize),
    current_height: u8,
    map: &Vec<Vec<u8>>,
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    // Peak reached - if not yet visited add one.
    if current_height == 9 && !visited.contains(&(x, y)) {
        visited.push((x, y));
        return 1;
    }
    let mut sum = 0;

    // println!("(x, y): ({}, {}), current_height: {} - {}", x, y, current_height, !visited.contains(&(x, y)));
    // Check all possible moves and height
    if x > 0 && map[x - 1][y] == current_height + 1 {
        sum += tailhead((x - 1, y), current_height + 1, map, visited);
    }

    if x + 1 < map.len() && map[x + 1][y] == current_height + 1 {
        sum += tailhead((x + 1, y), current_height + 1, map, visited);
    }

    if y > 0 && map[x][y - 1] == current_height + 1 {
        sum += tailhead((x, y - 1), current_height + 1, map, visited);
    }

    if y + 1 < map[0].len() && map[x][y + 1] == current_height + 1 {
        sum += tailhead((x, y + 1), current_height + 1, map, visited);
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

    if x > 0 && map[x - 1][y] == current_height + 1 {
        sum += path_count((x - 1, y), current_height + 1, map);
    }

    if x + 1 < map.len() && map[x + 1][y] == current_height + 1 {
        sum += path_count((x + 1, y), current_height + 1, map);
    }

    if y > 0 && map[x][y - 1] == current_height + 1 {
        sum += path_count((x, y - 1), current_height + 1, map);
    }

    if y + 1 < map[0].len() && map[x][y + 1] == current_height + 1 {
        sum += path_count((x, y + 1), current_height + 1, map);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 81);
    }
}
