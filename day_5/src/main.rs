use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Path to the file
    let path = "input.txt"; // index.txt
    let input = File::open(&path).expect("Sth wrong with file");
    let reader = io::BufReader::new(input);

    // Variables?
    let mut page_order: Vec<(i32, i32)> = Vec::new();
    let mut page_update: Vec<Vec<i32>> = Vec::new();
    let mut first_section = true;

    // Read file
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            // Look for empty line!
            // For future Me: maybe split by '\n\n'?
            first_section = false;
            continue;
        }

        if first_section {
            // Rules to read from line
            if let Some((a, b)) = line.split_once('|') {
                if let (Ok(a), Ok(b)) = (a.trim().parse::<i32>(), b.trim().parse::<i32>()) {
                    page_order.push((a, b));
                }
            }
        } else {
            // Read page updates
            let numbers: Vec<i32> = line
                .split(',')
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect();
            page_update.push(numbers);
        }
    }

    // println!("page_order:  {:?}", page_order);
    // println!("page_update: {:?}", page_update);

    // PART 1
    // let mut correct_order = 0;
    // for pu in page_update.iter() {
    //     if check_order(&page_order, pu) {
    //         correct_order += pu[pu.len() / 2]; // Only the middle element is interesting!
    //     }
    // }
    // println!("Correct Order: {}", correct_order);

    // PART 2
    // Todo: Revise the Rust complex data types!
    let mut order_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (i, j) in page_order.iter() {
        if !order_map.contains_key(i) {
            order_map.insert(*i, HashSet::new());
        }

        order_map.get_mut(i).unwrap().insert(*j);
    }

    let mut count_corrected_order = 0;

    for pu in page_update.iter() {
        if !check_order(&page_order, pu) {
            let fixed_order = correct_order(&order_map, pu);
            count_corrected_order += fixed_order[fixed_order.len() / 2];
        }
    }
    println!("Corrected Order: {}", count_corrected_order);


    Ok(())
}

fn correct_order(order_map: &HashMap<i32, HashSet<i32>>, update: &[i32]) -> Vec<i32> {
    let mut tmp: Vec<i32> = update.iter().map(|i| *i).collect();

    // Todo: master that magic sorting 
    tmp.sort_by(|a, b| {
        let a_before = order_map.get(a);
        let b_before = order_map.get(b);

        if let Some(before) = a_before {
            if before.contains(b) {
                return Ordering::Less;
            }
        }

        if let Some(before) = b_before {
            if before.contains(a) {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    });

    tmp
}

fn check_order(order: &[(i32, i32)], update: &[i32]) -> bool {
    let mut pos = HashMap::new();

    for (i, page) in update.iter().enumerate() {
        pos.insert(page, i);
    }

    for (first, second) in order.iter() {
        match (pos.get(first), pos.get(second)) {
            (Some(f), Some(s)) => {
                if f>=s {
                    return false;
                }
            },
            _ => {}
        }
    }

    true
}