use std::fs;

fn main() {

    // Read the file into a 2D array 
    // #IAMLAZY I have transomed the file into numbers, ergo X -> 1; M -> 2; A -> 3; S -> 4
    // Numbers I can find, letters no...
    let content = fs::read_to_string("input_t.txt").expect("Failed to read file"); //input_t
    let grid: Vec<Vec<u8>> = content
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut xmas_pattern_count = 0; // XMAS counter
    let mut x_mas_pattern_count = 0; // MAS in X counter

    let rows = grid.len();
    let cols = grid[0].len();

    // Check Patterns
    for i in 0..rows {
        for j in 0..cols {
            // Check patterns for XMAS
            xmas_pattern_count += count_horizontal(&grid, i, j, &[1, 2, 3, 4]);
            xmas_pattern_count += count_horizontal(&grid, i, j, &[4, 3, 2, 1]);
            xmas_pattern_count += count_vertical(&grid, i, j, &[1, 2, 3, 4]);
            xmas_pattern_count += count_vertical(&grid, i, j, &[4, 3, 2, 1]);
            xmas_pattern_count += count_diagonal_down_right(&grid, i, j, &[1, 2, 3, 4]);
            xmas_pattern_count += count_diagonal_down_right(&grid, i, j, &[4, 3, 2, 1]);
            xmas_pattern_count += count_diagonal_up_right(&grid, i, j, &[1, 2, 3, 4]);
            xmas_pattern_count += count_diagonal_up_right(&grid, i, j, &[4, 3, 2, 1]);
            
            // MAS in X has to 'have' "A"
            if grid[i][j] == 3 {
                x_mas_pattern_count += count_x_patterns(&grid, i, j, rows, cols);
            }
        }
    }

    // Print results
    println!("PATTERNS:   {}", xmas_pattern_count);
    println!("X Patterns: {}", x_mas_pattern_count);
}

// Horizontal pattern - 1234
fn count_horizontal(grid: &Vec<Vec<u8>>, i: usize, j: usize, pattern: &[u8]) -> usize {
    if j + pattern.len() > grid[0].len() {
        return 0;
    }
    for (k, &val) in pattern.iter().enumerate() {
        if grid[i][j + k] != val {
            return 0;
        }
    }
    1
}

// Vertical pattern -   1
//                      2
//                      3
//                      4
fn count_vertical(grid: &Vec<Vec<u8>>, i: usize, j: usize, pattern: &[u8]) -> usize {
    if i + pattern.len() > grid.len() {
        return 0;
    }
    for (k, &val) in pattern.iter().enumerate() {
        if grid[i + k][j] != val {
            return 0;
        }
    }
    1
}

// Diagonal pattern -   1
//                      .2
//                      ..3
//                      ...4
fn count_diagonal_down_right(grid: &Vec<Vec<u8>>, i: usize, j: usize, pattern: &[u8]) -> usize {
    if i + pattern.len() > grid.len() || j + pattern.len() > grid[0].len() {
        return 0;
    }
    for (k, &val) in pattern.iter().enumerate() {
        if grid[i + k][j + k] != val {
            return 0;
        }
    }
    1
}

// Diagonal pattern -   ...4
//                      ..3
//                      .2
//                      1
fn count_diagonal_up_right(grid: &Vec<Vec<u8>>, i: usize, j: usize, pattern: &[u8]) -> usize {
    if i < pattern.len() - 1 || j + pattern.len() > grid[0].len() {
        return 0;
    }
    for (k, &val) in pattern.iter().enumerate() {
        if grid[i - k][j + k] != val {
            return 0;
        }
    }
    1
}


// 2.4      2.2     4.4     4.2
// .3.  or  .3. or  .3. or  .3.
// 2.4      4.4     2.2     4.2
//
//
fn count_x_patterns(grid: &Vec<Vec<u8>>, i: usize, j: usize, rows: usize, cols: usize, ) -> usize {
    if i == 0 || j == 0 || i+1 == rows || j+1 == cols {
        return 0;
    } // Out of bounds

    if grid[i-1][j-1] == 2 && grid[i-1][j+1] == 4 
        && grid[i+1][j-1] == 2 && grid[i+1][j+1] == 4 {
            return 1;
    }

    if grid[i-1][j-1] == 2 && grid[i-1][j+1] == 2 
        && grid[i+1][j-1] == 4 && grid[i+1][j+1] == 4 {
            return 1;
    }

    if grid[i-1][j-1] == 4 && grid[i-1][j+1] == 4 
        && grid[i+1][j-1] == 2 && grid[i+1][j+1] == 2 {
            return 1;
    }

    if grid[i-1][j-1] == 4 && grid[i-1][j+1] == 2 
        && grid[i+1][j-1] == 4 && grid[i+1][j+1] == 2 {
            return 1;
    }

    0
}
