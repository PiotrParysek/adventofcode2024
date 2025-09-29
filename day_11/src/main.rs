use cached::proc_macro::cached;

fn main() {
    let input = include_str!("../input.txt");
    let numbers = input.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{:?}", numbers);
    let part_1 = numbers.iter()
        .map(|n| blink(*n, 25))
        .sum::<usize>();
    println!("PART 1: {}", part_1);
    let part_2 = numbers.iter()
        .map(|n| blink(*n, 75))
        .sum::<usize>();
    println!("PART 2: {}", part_2);

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read() {
        let input = include_str!("../test.txt");
        let numbers = input.split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(numbers, vec![125, 17]);
    }

    #[test]
    fn test_part_1() {
        let numbers = vec![125, 17];
        let part_1 = numbers.iter() 
            .map(|n| blink(*n, 6))
            .sum::<usize>();
        assert_eq!(part_1, 22);
        
    }
    #[test]
    fn test_part_2() {
        let numbers = vec![125, 17];
        let part_2 = numbers.iter()
            .map(|n| blink(*n, 25))
            .sum::<usize>();
        assert_eq!(part_2, 55312);
    }
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

