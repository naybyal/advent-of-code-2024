use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let path = "input.txt";
    let mut sum_of_products = 0;
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
		            for captures in mul_regex.captures_iter(&content) {
                    let x: i32 = captures[1].parse().unwrap(); // parse the first number
                    let y: i32 = captures[2].parse().unwrap(); // parse the second number
                    sum_of_products += x * y;
                }
            }
        }
    }
    
    println!("Sum of all products: {}", sum_of_products);

}
