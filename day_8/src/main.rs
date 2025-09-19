use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
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

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let file = read_file("input.txt".to_string());

    let height = file.len().try_into().unwrap();
    let width = file[0].len().try_into().unwrap();
    println!("Height: {}, Width: {}", height, width);

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for (row, line) in file.iter().enumerate() {
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
            // let anitnode_1 = p2 + diff;
            // let anitnode_2 = p1 - diff;

            // if anitnode_1.check(width, height) {
            //     antinodes.insert(anitnode_1);
            // }
            // if anitnode_2.check(width, height) {
            //     antinodes.insert(anitnode_2);
            // }
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
    // for (c, pos) in antennas   {
    //     println!("{}: {:?}\n", c, pos);
    // }


    println!("Antinodes: {:?}", antinodes.len());

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