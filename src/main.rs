use clap::Parser;
use std::fs;
use std::time::Instant;

use aoc_2024::solve;

/// Solver for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The day to run
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// The part to run
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Use the sample input
    #[arg(long, short, action = clap::ArgAction::SetTrue)]
    sample: bool,
}

fn main() {
    let cli = Cli::parse();

    // Determine which input file to use
    let file_name = if cli.sample {
        format!("input/{:02}_sample.txt", cli.day)
    } else {
        format!("input/{:02}.txt", cli.day)
    };

    // Read the input file
    let input = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Could not read input file: {}", file_name));

    // Run the solver and time it
    let start = Instant::now();
    let solution = solve(cli.day, cli.part, &input);
    let duration = start.elapsed();

    // Print the solution
    println!("--- Day {} Part {} ---", cli.day, cli.part);
    println!("Solution: {}", solution);
    println!("Time: {:?}", duration);
}