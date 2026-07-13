use cached::proc_macro::cached;

pub fn part1(input: &str) -> u32 {
    let numbers = input.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{:?}", numbers);

    let part_1 = numbers.iter()
        .map(|n| blink(*n, 25))
        .sum::<usize>();
    println!("PART 1: {}", part_1);
    part_1 as u32
}

pub fn part2(input: &str) -> u32 {
    let numbers = input.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{:?}", numbers);

    let part_2 = numbers.iter()
        .map(|n| blink(*n, 75))
        .sum::<usize>();
    println!("PART 2: {}", part_2);

    part_2 as u32
}

#[cached]
fn blink(num: usize, remaining: usize) -> usize {
    if remaining == 0 {
        return 1;
    }

    let temp_str = num.to_string();
    
    if num == 0 {
        // Stone with number '0' -> replaced with '1'
        blink(1, remaining - 1)
    } else if temp_str.len() % 2 == 0 {
        // Stone with even numbers of digits -> split in half
        let half_len = temp_str.len() / 2;
        let (left, right) = temp_str.split_at(half_len);
        let left_num = left.parse::<usize>().unwrap();
        let right_num = right.parse::<usize>().unwrap();
        blink(left_num, remaining - 1) + blink(right_num, remaining - 1)
    } else {
        // Or multiply by 2024
        blink(num * 2024, remaining - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 4003138674);
    }
}