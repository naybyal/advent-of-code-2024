use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let path = "/mnt/data/input.txt"; // Path to the uploaded input file
    let mut sum_of_products = 0;
    let mut mul_enabled = true; // Multiplications are enabled by default

    // Regex patterns
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"\bdo\(\)").unwrap();
    let dont_regex = Regex::new(r"\bdon't\(\)").unwrap();

    if let Ok(lines) = read_lines(path) {
        for (line_no, line) in lines.enumerate() {
            if let Ok(content) = line {
                // Check for `do()` and `don't()` to update the state
                if do_regex.is_match(&content) {
                    mul_enabled = true;
                    println!("Line {}: do() -> mul_enabled = true", line_no + 1);
                }
                if dont_regex.is_match(&content) {
                    mul_enabled = false;
                    println!("Line {}: don't() -> mul_enabled = false", line_no + 1);
                }

                // Process `mul(...)` instructions if enabled
                for caps in mul_regex.captures_iter(&content) {
                    let num1: i32 = caps[1].parse().unwrap();
                    let num2: i32 = caps[2].parse().unwrap();
                    let product = num1 * num2;

                    if mul_enabled {
                        sum_of_products += product;
                        println!("Line {}: mul({}, {}) -> +{} (enabled)", line_no + 1, num1, num2, product);
                    } else {
                        println!("Line {}: mul({}, {}) -> skipped (disabled)", line_no + 1, num1, num2);
                    }
                }
            }
        }
    }

    println!("Sum of products with conditions: {}", sum_of_products);
    Ok(())
}

