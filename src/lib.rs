pub mod common;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

/// Solves the given day's puzzle.
///
/// This function acts as a dispatcher, calling the correct
/// part1 or part2 function based on the day and part.
///
/// It also handles converting the puzzle's return type
/// (e.g., u32, String) into a String for printing.
pub fn solve(day: u8, part: u8, input: &str) -> String {
    match day {
        // ACHTUNG For next  days, pattern:
        // DD => match part {
        //     1 => dayDD::part1(input).to_string(),
        //     2 => dayDD::part2(input).to_string(),
        //     _ => "Invalid part".to_string(),
        // },
        1 => match part {
            1 => day01::part1(input).to_string(),
            2 => day01::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        2 => match part {
            1 => day02::part1(input).to_string(),
            2 => day02::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        3 => match part {
            1 => day03::part1(input).to_string(),
            2 => day03::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        4 => match part {
            1 => day04::part1(input).to_string(),
            2 => day04::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        5 => match part {
            1 => day05::part1(input).to_string(),
            2 => day05::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        6 => match part {
            1 => day06::part1(input).to_string(),
            2 => day06::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        7 => match part {
            1 => day07::part1(input).to_string(),
            2 => day07::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        8 => match part {
            1 => day08::part1(input).to_string(),
            2 => day08::part2(input).to_string(),
            _ => "Invalid part".to_string(),
        },
        // 12 => match part {
        //     1 => day12::part1(input).to_string(),
        //     2 => day12::part2(input).to_string(),
        //     _ => "Invalid part".to_string(),
        // },
        // Add more days here
        _ => "Invalid day".to_string(),
    }
}