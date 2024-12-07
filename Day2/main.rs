use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to check if a report is safe (strictly increasing or decreasing, with valid differences)
fn check_safety(levels: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..levels.len() - 1 {
        let diff = levels[i + 1] - levels[i];

        if diff == 0 || diff.abs() > 3 {
            return false; // Invalid difference
        }

        if diff > 0 {
            decreasing = false; // Not decreasing
        } else if diff < 0 {
            increasing = false; // Not increasing
        }
    }

    increasing || decreasing // Safe if strictly increasing or decreasing
}

// Function to check if removing one level makes the report safe
fn check_safety_with_one_removal(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        // Create a temporary vector with one less element
        let mut temp_levels = Vec::with_capacity(levels.len() - 1);
        for j in 0..levels.len() {
            if j != i {
                temp_levels.push(levels[j]);
            }
        }

        // Check if the report becomes safe after removal
        if check_safety(&temp_levels) {
            return true;
        }
    }

    false // If no removal makes it safe, return false
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let mut safe_count = 0;

    if let Ok(lines) = read_lines(path) {
        for (index, line) in lines.enumerate() {
            if let Ok(content) = line {
                let levels: Vec<i32> = content
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();
                
                // Check if the report is safe or becomes safe after removing one level
                let is_safe = check_safety(&levels) || check_safety_with_one_removal(&levels);
                
                if is_safe {
                    safe_count += 1;
                }
            }
        }
    }

    println!("Total Safe Lines: {}", safe_count);
    Ok(())
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

