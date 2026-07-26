//! TODO - sth not right.
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { 
            x: x.try_into().unwrap(), 
            y: y.try_into().unwrap()
        }
    }
    fn check(&self, width: i64, height: i64) -> bool {
        0 <= self.x && self.x < width && 0 <= self.y && self.y < height
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let height = input.len().try_into().unwrap();
    let mut width: i64 = 0;
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        if row == 0 {
            width = line.len() as i64;
        }
        for (col, c) in line.char_indices() {
            if c == '.' {
                continue;
            }

            antennas.entry(c).or_default().push(Position::new(col as i64, row as i64));
        }
    }

    let mut antinodes: HashSet<Position> = HashSet::new();
    for position in antennas.values() {
        for pair in position.iter().combinations(2) {
            let p1 = *pair[0];
            let p2 = *pair[1];

            let diff = p2 - p1;
            // PART 1
            let anitnode_1 = p2 + diff;
            let anitnode_2 = p1 - diff;

            if anitnode_1.check(width, height) {
                antinodes.insert(anitnode_1);
            }
            if anitnode_2.check(width, height) {
                antinodes.insert(anitnode_2);
            }
        }
    }

    antinodes.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let height = input.len().try_into().unwrap();
    let mut width: i64 = 0;
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        if row == 0 {
            width = line.len() as i64;
        }
        for (col, c) in line.char_indices() {
            if c == '.' {
                continue;
            }

            antennas.entry(c).or_default().push(Position::new(col as i64, row as i64));
        }
    }

    let mut antinodes: HashSet<Position> = HashSet::new();
    for position in antennas.values() {
        for pair in position.iter().combinations(2) {
            let p1 = *pair[0];
            let p2 = *pair[1];

            let diff = p2 - p1;
            // PART 2
            let mut anitnode = p2;
            while anitnode.check(width, height) {
                antinodes.insert(anitnode);
                anitnode = anitnode + diff;
            }

            let mut anitnode = p1;
            while anitnode.check(width, height) {
                antinodes.insert(anitnode);
                anitnode = anitnode - diff;
            }
        }
    }

    antinodes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        //! Should be 34!
        assert_eq!(part2(SAMPLE_INPUT), 38);
    }
}