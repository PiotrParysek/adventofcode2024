// Template for Day XY of Advent of Code 2024

#[derive(Debug)]
struct ClawMachine {
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    px: u32,
    py: u32,
}

fn parse_clawmachine(input: &str) -> Vec<ClawMachine> {
    input.split("\n\n")
        .filter_map(|block| {
            let mut lines = block.lines();
            
            // Three lines of data 
            let a_line = lines.next()?;
            let b_line = lines.next()?;
            let p_line = lines.next()?;

            let parse_line = |line: &str, sep_x: &str, sep_y: &str| -> Option<(u32, u32)> {
                let parts: Vec<&str> = line.split(", ").collect();
                if parts.len() != 2 { return None; }
                
                let x = parts[0].split(sep_x).nth(1)?.parse().ok()?;
                let y = parts[1].split(sep_y).nth(1)?.parse().ok()?;
                
                Some((x, y))
            };

            let (ax, ay) = parse_line(a_line, "+", "+")?;
            let (bx, by) = parse_line(b_line, "+", "+")?;
            let (px, py) = parse_line(p_line, "=", "=")?;

            Some(ClawMachine { ax, ay, bx, by, px, py })
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    /*
     * 1. Parse input,
     * 2. `A` -> 3 tokens
     *    `B` -> 1 token
     * Ex.: Button A: X+94, Y+34
     *      Button B: X+22, Y+67
     *      Prize: X=8400, Y=5400
     * 3. Push `A` => X += 94 && Y += 34
     *    Push `B` => X += 22 && y += 67
     *    Prize => X=8400 && Y=5400
     * 
     *    So: by pushing `A` 80 times and `B` 40 times gets the prize
     *        80*94 + 40*22 = 8400
     *        80*34 + 40*67 = 5400
     *    With cost: 80*3 + 40*1 = 280
     * 4. Coefficient matrix and target vector
     * 
     *    px = i*ax + j*bx
     *    py = i*ay + j*by
     *            
     *    A = ⌈ax bx⌉  X = ⌈i⌉  AX = C = ⌈px⌉
     *        ⌊ay by⌋      ⌊j⌋  ⌊py⌋
     *            
     *    D = |A| = ax*by - ay*bx
     *    Di = px*by - py*bx
     *    Dj = py*ax - px*ay
     *            
     *    i = Di / D  ! CHECK IF IT IS INTEGER
     *    j = Dj / D  ! CHECK IF IT IS INTEGER
     *    answer = 3*i + j
     * 
     */
    let mut res: u32 = 0;
    for c in parse_clawmachine(input) {
        // println!("{:?}", c); // For debugging
        let d  = c.ax * c.by - c.ay * c.bx;
        let di = c.px * c.by - c.py * c.bx;
        let dj = c.py * c.ax - c.px * c.ay;

        if di % d == 0 && dj % d == 0 {
            res += (3 * di / d + dj / d) as u32; 
        }
    }
    
    res
}

pub fn part2(input: &str) -> u32 {
    let mut res: i64 = 0;
    for c in parse_clawmachine(input) {
        let px: i64 = (c.px as i64) + 10000000000000 ;
        let py: i64 = (c.py as i64) + 10000000000000;
        // println!("{:?}", c); // For debugging
        let d: i64  = (c.ax as i64) * (c.by as i64) - (c.ay as i64) * (c.bx as i64);
        let di: i64 = px * (c.by as i64) - py * (c.bx as i64);
        let dj: i64 = py * (c.ax as i64) - px * (c.ay as i64);

        if di % d == 0 && dj % d == 0 {
            res += 3 * di / d + dj / d; 
        }
    }
    println!("RES {}", res);
    res as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 0); // 875318608908
    }
}