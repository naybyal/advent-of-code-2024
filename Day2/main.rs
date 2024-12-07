use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                
                let is_safe = check_safety(&levels);
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

