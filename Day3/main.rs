use std::fs;
use regex::Regex;

// Function to read input from a file
fn read_input(input_file_dir: &str) -> String {
    fs::read_to_string(input_file_dir).expect("Failed to read file")
}

// PART 1: Function to sum multiplications
fn sum_multiplications(memory: &[String]) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;

    for element in memory {
        for cap in pattern.captures_iter(element) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            total += x * y;
        }
    }
    total
}

// PART 2: Function to remove inactive memory
fn remove_inactive_memory(memory: &str) -> Vec<String> {
    memory
        .split("do()")
        .filter_map(|e| e.split("don't()").next())
        .map(String::from)
        .collect()
}

// Function to sum multiplications in active memory
fn sum_multiplications_activation(memory: &str) -> i32 {
    let filtered_memory = remove_inactive_memory(memory);
    sum_multiplications(&filtered_memory)
}

fn main() {
    let input = read_input("./input.txt");
    let result = sum_multiplications_activation(&input);
    println!("{}", result);
}

