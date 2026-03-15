use std::{env, fs::File, io::{self, BufRead}};

fn main() {
    env::set_var("RUST_BACKTRACE", "full"); // Full backtrace 
    let mut x = 0;
    let mut y = 0;
    let inupt = include_str!("../example.txt")
    .chars().fold(HashMap::new(), |mut acc, c| {
        match c {
            c if c.is_alphabetic() => {
                acc.insert((x, y), c);
                x += 1;
            },
            '\n' => { 
                x = 0;
                y += 1;
            },
            _ => { x += 1; },
        }
        acc
    });

    x = 0;
    y = 0;
}
