use regex::Regex;

pub fn part1(input: &str) -> u32 {
    // Regex: detect the "mul(D,D)" and "do()" and "don't()"
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do(?:n't)?\(\)").unwrap(); 
    // Regex: extract numbers
    let re_num = Regex::new(r"\d+").unwrap();

    let mut final_number: u32 = 0;

    for line in input.lines() {
        // println!("LINE: {}", line);
        let captured_values: Vec<&str> = re.find_iter(&line).map(|cn| cn.as_str()).collect();
        // println!("CN: {:?}", captured_values);
        for cn in captured_values {
            let c = &cn[..3];
            // based on three first characters allow/disable multiplication and multiply numbers, if it is 'allowed'
            match c {
                "mul" => {
                    let numbers: Vec<i32> = re_num.find_iter(&cn).filter_map(|n| n.as_str().parse::<i32>().ok()).collect();
                    // println!("NUM: {:?}", numbers);
                    final_number += numbers.iter().fold(1, |acc, &x| acc * x) as u32;
                },
                _ => println!("STH HAPPENED: {} - {}", cn, c)
            }
        }
    }
    // println!("FN: {}", final_number);
    final_number
}

pub fn part2(input: &str) -> u32 {
    // Regex: detect the "mul(D,D)" and "do()" and "don't()"
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do(?:n't)?\(\)").unwrap(); 
    // Regex: extract numbers
    let re_num = Regex::new(r"\d+").unwrap();

    let mut final_number: u32 = 0;
    let mut multiply:bool = true;

    for line in input.lines() {
        // println!("LINE: {}", line);
        let captured_values: Vec<&str> = re.find_iter(&line).map(|cn| cn.as_str()).collect();
        // println!("CN: {:?}", captured_values);
        for cn in captured_values {
            let c = &cn[..3];
            // based on three first characters allow/disable multiplication and multiply numbers, if it is 'allowed'
            match c {
                "don" => multiply = false,
                "do(" => multiply = true,
                "mul" => if multiply {
                    let numbers: Vec<i32> = re_num.find_iter(&cn).filter_map(|n| n.as_str().parse::<i32>().ok()).collect();
                    // println!("NUM: {:?}", numbers);
                    final_number += numbers.iter().fold(1, |acc, &x| acc * x) as u32;
                },
                _ => println!("STH HAPPENED: {} - {}", cn, c)
            }
        }
    }
    // println!("FN: {}", final_number);
    final_number
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 161);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 48);
    }
}