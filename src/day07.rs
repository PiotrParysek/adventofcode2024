// Template for Day XY of Advent of Code 2024

pub fn part1(input: &str) -> u32 {
    let mut vec: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.lines() {
        println!("l: {}", line);
        let (res, num) = line.split_once(":").unwrap();
        let res = res.parse().unwrap();
        let num = num.split_whitespace().map(|x| x.parse().unwrap()).collect();
        vec.push((res, num));
    }
    let mut part_1 = 0;
    for (goal, numbers) in vec {
        let (start, numbers) = numbers.split_first().unwrap();
        if solve_part_1(*start, goal, numbers) {
            part_1 += goal;
        }
    }
    println!("Part 1: {}", part_1);
    part_1 as u32
}

pub fn part2(input: &str) -> u32 {
    let mut vec: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.lines() {
        println!("l: {}", line);
        let (res, num) = line.split_once(":").unwrap();
        let res = res.parse().unwrap();
        let num = num.split_whitespace().map(|x| x.parse().unwrap()).collect();
        vec.push((res, num));
    }

    let mut part_2 = 0;
    for (goal, numbers) in vec {
        let (start, numbers) = numbers.split_first().unwrap();
        if solve_part_2(*start, goal, numbers) {
            part_2 += goal;
        }
    }
    println!("Part 2: {}", part_2);
    part_2 as u32
}

fn solve_part_1(result: u64, goal: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return result == goal;
    }
    if result > goal {
        return false;
    }
    let (first, rest) = numbers.split_first().unwrap();
    solve_part_1(result * first, goal, rest) || solve_part_1(result + first, goal, rest)
}

fn solve_part_2(result: u64, goal: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return result == goal;
    }
    if result > goal {
        return false;
    }
    let (first, rest) = numbers.split_first().unwrap();
    solve_part_2(concat_numbers(result, *first), goal, rest)
        || solve_part_2(result * first, goal, rest)
        || solve_part_2(result + first, goal, rest)
}

fn concat_numbers(num1: u64, num2: u64) -> u64 {
    let mut res = num1;
    let mut temp = num2;
    while temp > 0 {
        res *= 10;
        temp /= 10;
    }
    res + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 11387);
    }
}