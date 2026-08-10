// Template for Day XY of Advent of Code 2024

use std::cmp::Ordering::{Greater, Less};
use std::io::{self};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i32, i32);
#[derive(Debug, Clone)]
struct Velocity(i32, i32);

#[derive(Debug, Clone)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

impl Robot {
    fn next(&mut self, time: i32, width: i32, height: i32) {
        self.pos.0 = (self.pos.0 + self.vel.0 * time).rem_euclid(width);
        self.pos.1 = (self.pos.1 + self.vel.1 * time).rem_euclid(height);
    }
}

fn view_robots(robots: &[Robot], width: i32, height: i32) {
    for l in 0..height {
        for c in 0..width {
            let count = robots.iter().fold(0, |acc, r| {
                if r.pos.0 == c && r.pos.1 == l {
                    acc +1
                } else {
                    acc
                }
            });
            print!("{}", if count ==0 {
                String::from(".")
            } else {
                count.to_string()
            });
        }
        println!();
    }
}

pub fn part1(input: &str) -> u32 {
    let width = 101;
    let height = 103;
    let robots = input
                .lines()
                .map(|line| {
                    let mut parts = line.split(|c| c == '=' || c == ',' || c == ' ');
                    let x:  i32 = parts.nth(1).unwrap().parse().unwrap();
                    let y:  i32 = parts.next().unwrap().parse().unwrap();
                    let vx: i32 = parts.nth(1).unwrap().parse().unwrap();
                    let vy: i32 = parts.next().unwrap().parse().unwrap();
                    Robot {
                        pos: Position(x, y),
                        vel: Velocity(vx, vy),
                    }
                }).collect::<Vec<_>>();
    // for r in &robots {
    //     println!("{:?}", r);
    // }
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in robots {
        let pnt = (
            (r.pos.0 + r.vel.0 * 100).rem_euclid(width),
            (r.pos.1 + r.vel.1 * 100).rem_euclid(height),
        );
        // println!("PNT: {:?}", pnt);

        match (
            pnt.0.cmp(&(width / 2)),
            pnt.1.cmp(&(height / 2)),
        ) {
            (Less, Less) => q1 += 1,
            (Less, Greater) => q2 += 1,
            (Greater, Less) => q3 += 1,
            (Greater, Greater) => q4 += 1,
            _ => (),
        }
    }
    // println!("{}", q1 * q2 * q3 * q4);
    
    q1 * q2 * q3 * q4
}

pub fn part2(input: &str) -> u32 {
    let width = 101;
    let height = 103;
    let mut robots = input
                .lines()
                .map(|line| {
                    let mut parts = line.split(|c| c == '=' || c == ',' || c == ' ');
                    let x:  i32 = parts.nth(1).unwrap().parse().unwrap();
                    let y:  i32 = parts.next().unwrap().parse().unwrap();
                    let vx: i32 = parts.nth(1).unwrap().parse().unwrap();
                    let vy: i32 = parts.next().unwrap().parse().unwrap();
                    Robot {
                        pos: Position(x, y),
                        vel: Velocity(vx, vy),
                    }
                }).collect::<Vec<_>>();
    let mut res = 0;

    for steps in 1..10000 {
        let mut g = [[' '; 101]; 103];
        for r in robots.iter_mut() {
            r.next(steps, width, height);
            g[r.pos.1 as usize][r.pos.0 as usize] = '#';
        }

        let good = (0..height as usize).any(|x| (0..width as usize).filter(|&y| g[x][y] == '#').count() > 30);
        if !good {
            continue;
        } else {
            let mut buffer = String::new();
            println!("----{steps}----");
            view_robots(&robots, width, height);
            res = steps;
            println!("Keep running? Y/N");
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let test = buffer.chars().next().unwrap_or('N');
                    if test == 'N' {
                        break;
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                    break;
                }
            }
            
        }
    }
    res as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        // CHANGE 
        // let width = 11;
        // let height = 7;
        assert_eq!(part1(SAMPLE_INPUT), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 0);
    }
}