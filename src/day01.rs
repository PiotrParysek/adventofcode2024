
pub fn part1(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new(); // 'Left' numbers
    let mut right: Vec<i32> = Vec::new(); // 'Right' numbers
    // println!("Input: {}", input);
    for line in input.lines() {
        println!("{}", line); // Print lines
        let (l,r) = line.split_once(char::is_whitespace).unwrap(); // Split lines 
        // println!("l:{} - r:{}", l.trim(), r.trim());
        left.push(l.trim().parse().unwrap()); // parse into vector
        right.push(r.trim().parse().unwrap()); // parse into vector
    }

    left.sort(); // Sort vector
    right.sort(); // Sort vector
    // println!("r: {:?}", right);
    // println!("l: {:?}", left);

    // Check if the vectors are the same 
    if left.len() != right.len() {
        eprintln!("Vectors are not the same length! {} <-> {}", left.len(), right.len());
        -1
    } else {
        // Calculate a sum of differences for sorted vectors l
        let diff: Vec<i32> = (0..left.len()).map(|i| i32::abs(right[i] - left[i])).collect(); 
        // println!("D: {:?}", diff);
        let sum: i32 = diff.iter().sum(); // Calculate the sum
        // println!("SUM: {}", sum);
        sum
    }
}

pub fn part2(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new(); // 'Left' numbers
    let mut right: Vec<i32> = Vec::new(); // 'Right' numbers
    // println!("Input: {}", input);
    for line in input.lines() {
        println!("{}", line); // Print lines
        let (l,r) = line.split_once(char::is_whitespace).unwrap(); // Split lines 
        // println!("l:{} - r:{}", l.trim(), r.trim());
        left.push(l.trim().parse().unwrap()); // parse into vector
        right.push(r.trim().parse().unwrap()); // parse into vector
    }
    // Check if the vectors are the same 
    if left.len() != right.len() {
        eprintln!("Vectors are not the same length! {} <-> {}", left.len(), right.len());
        -1
    } else {
        let mut sum: i32 = 0;
        for i in left.iter_mut() {
            sum += *i * (right.iter().filter(|&x| *x == *i).count() as i32);
        }
        println!("SUM: {}", sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 31);
    }
}