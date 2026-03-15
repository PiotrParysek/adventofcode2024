// Template for Day XY of Advent of Code 2024

pub fn part1(input: &str) -> u32 {
    let mut line_validator = 0;
    for line in input.lines(){
        // println!("{}", line);
        // Split line by whitespace, parse it into i32 and 'wrap' it into vec
        let level: Vec<i32> = line.split_whitespace()
        .map(|s| s.to_string().parse::<i32>().unwrap()).collect(); 
        // Check if the line is 'valid'
        if is_valid_line(&level) && is_valid_stable(&level) {
            // println!("Line: {} is valid", line);
            line_validator += 1;
        } else {
            // If line is invalid - remove ONE element
            let level_len = level.len();
            for i in 0..level_len {
                let mut tmp_level = level.clone();
                tmp_level.remove(i);
                if is_valid_line(&tmp_level) && is_valid_stable(&tmp_level) {
                    // println!("Line: {} is valid after remove index: {}", line, i);
                    line_validator += 1;
                    // If Removing one element 'solves' the riddle break the loop!
                    break;
                }
            }
        }
    }
    // println!("Valid lines: {}", line_validator);
    line_validator
}

pub fn part2(input: &str) -> u32 {
    0
}

/// Check if difference between next two elements is 1, 2 or 3
fn is_valid_line(line: &[i32]) -> bool {
    line.windows(2).all(|pair| {
        if let [a, b] = pair {
            let diff = (a - b).abs();
            diff == 1 || diff == 2 || diff == 3
        } else {
            false
        }
    })
}

/// Check if the line is 'stable' ergo, decreasing or increasing
fn is_valid_stable(line: &[i32]) -> bool {
    let sign = (line[0] - line[1]).signum();
    line.windows(2).all(|pair| {
        if let [a, b] = pair {
            if sign == (a - b).signum() {
                true
            } else {
                false
            }
        } else {
            false      
        }
    }) 
}


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add sample input from the problem
    const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 4);
    }
}