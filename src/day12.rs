// Template for Day XY of Advent of Code 2024

use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> u32 {
    let garden: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    // view_garden(&garden);

    let mut visited_plots = HashSet::new();
    let mut result = 0;

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            // println!("{}-{}: {}", y, x, garden[y][x]);
            if !visited_plots.contains(&(y, x)) {
                let mut q = VecDeque::new();
                q.push_back((y, x));

                let (mut p, mut s) = (0, 0);
                while let Some((y, x)) = q.pop_front() {
                    if visited_plots.insert((y, x)) {
                        s += 1;
                        p += 4;
                        // up
                        if y > 0 {
                            if garden[y - 1][x] == garden[y][x] {
                                p -= 1;
                                q.push_back((y - 1, x));
                            }
                        }
                        // left
                        if x > 0 {
                            if garden[y][x - 1] == garden[y][x] {
                                p -= 1;
                                q.push_back((y, x - 1));
                            }
                        }
                        // down
                        if y < garden.len() - 1 {
                            if garden[y + 1][x] == garden[y][x] {
                                p -= 1;
                                q.push_back((y + 1, x));
                            }
                        }
                        //right
                        if x < garden[y].len() - 1 {
                            if garden[y][x + 1] == garden[y][x] {
                                p -= 1;
                                q.push_back((y, x + 1));
                            }
                        }
                    }
                }

                result += p * s;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> u32 {
    let garden: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    // view_garden(&garden);

    let mut visited_plots = HashSet::new();
    let mut result: u32 = 0;

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            // println!("{}-{}: {}", y, x, garden[y][x]);
            if !visited_plots.contains(&(y, x)) {
                let mut q = VecDeque::new();
                q.push_back((y, x));

                let (mut p, mut s) = (0, 0);
 
                                   while let Some((y, x)) = q.pop_front() {
                    if visited_plots.insert((y, x)) {
                        let a = garden[y][x];
                        s += 1;
                        // top-left corner
                        if y == 0 && x == 0 {
                            p += 1
                        }
                        // top-right corner
                        if y == 0 && x == garden[0].len() - 1 {
                            p += 1
                        }
                        // bottom-left corner
                        if y == garden.len() - 1 && x == 0 {
                            p += 1
                        }
                        // bottom-right corner
                        if y == garden.len() - 1 && x == garden[0].len() - 1 {
                            p += 1
                        }
                        // top border
                        if y == 0 {
                            // left
                            if x > 0 {
                                if a != garden[y][x - 1] {
                                    p += 1;
                                }
                            }
                            // right
                            if x < garden[0].len() - 1 {
                                if a != garden[y][x + 1] {
                                    p += 1;
                                }
                            }
                        }
                        // bottom border
                        if y == garden[0].len() - 1 {
                            // left
                            if x > 0 {
                                if a != garden[y][x - 1] {
                                    p += 1;
                                }
                            }
                            // right
                            if x < garden[0].len() - 1 {
                                if a != garden[y][x + 1] {
                                    p += 1;
                                }
                            }
                        }
                        // left border
                        if x == 0 {
                            // top
                            if y > 0 {
                                if a != garden[y - 1][x] {
                                    p += 1;
                                }
                            }
                            // bottom
                            if y < garden.len() - 1 {
                                if a != garden[y + 1][x] {
                                    p += 1;
                                }
                            }
                        }
                        // right border
                        if x == garden[0].len() - 1 {
                            // top
                            if y > 0 {
                                if a != garden[y - 1][x] {
                                    p += 1;
                                }
                            }
                            // bottom
                            if y < garden.len() - 1 {
                                if a != garden[y + 1][x] {
                                    p += 1;
                                }
                            }
                        }
                        //left-top
                        if x > 0 && y > 0 {
                            // dB
                            // C*
                            let b = garden[y - 1][x];
                            let c = garden[y][x - 1];
                            let d = garden[y - 1][x - 1];
                            if b != a && c != a {
                                p += 1;
                            }
                            // da
                            // a*
                            if d != a && b == a && c == a {
                                p += 1;
                            }
                        }
                        //left-bottom
                        if x > 0 && y < garden.len() - 1 {
                            // C*
                            // dB
                            let b = garden[y + 1][x];
                            let c = garden[y][x - 1];
                            let d = garden[y + 1][x - 1];
                            if b != a && c != a {
                                p += 1;
                            }
                            // a*
                            // da
                            if d != a && b == a && c == a {
                                p += 1;
                            }
                        }
                        // right-top
                        if x < garden[0].len() - 1 && y > 0 {
                            // bd
                            // *c
                            let b = garden[y - 1][x];
                            let d = garden[y - 1][x + 1];
                            let c = garden[y][x + 1];
                            if a != b && a != c {
                                p += 1;
                            }
                            // ad
                            // *a
                            if a != d && a == b && a == c {
                                p += 1;
                            }
                        }
                        // right-bottom
                        if x < garden[0].len() - 1 && y < garden.len() - 1 {
                            // *c
                            // bd
                            let b = garden[y + 1][x];
                            let d = garden[y + 1][x + 1];
                            let c = garden[y][x + 1];
                            if a != b && a != c {
                                p += 1;
                            }
                            // ad
                            // *a
                            if a != d && a == b && a == c {
                                p += 1;
                            }
                        }
                        if y > 0 {
                            if garden[y - 1][x] == garden[y][x] {
                                q.push_back((y - 1, x));
                            }
                        }
                        if x > 0 {
                            if garden[y][x - 1] == garden[y][x] {
                                q.push_back((y, x - 1));
                            }
                        }
                        if y < garden.len() - 1 {
                            if garden[y + 1][x] == garden[y][x] {
                                q.push_back((y + 1, x));
                            }
                        }
                        if x < garden[0].len() - 1 {
                            if garden[y][x + 1] == garden[y][x] {
                                q.push_back((y, x + 1));
                            }
                        }
                    }
                }
                
                result += p * s;
            }
        }
    }
    result
}

/**
 * View the 'garden'
 */
#[warn(dead_code)]
fn view_garden(garden: &[Vec<char>]) {
    for x in garden {
        for y in x {
            print!("{}", y);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";
    const SAMPLE_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const SAMPLE_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT_1), 140);
        assert_eq!(part1(SAMPLE_INPUT_2), 772);
        assert_eq!(part1(SAMPLE_INPUT_3), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT_1), 80);
        assert_eq!(part2(SAMPLE_INPUT_2), 436);
        assert_eq!(part2(SAMPLE_INPUT_3), 1206);
    }
}