use std::collections::HashMap;

fn main() {
    // Read the input grid
    let input = include_str!("input.txt");

    // Parse the input into a 2D character lookup table (LUT)
    let lut = parse_grid(input);

    // Count occurrences of X-MAS patterns
    let xmas_count = count_xmas_patterns(&lut);

    println!("Occurrences of X-MAS patterns: {}", xmas_count);
}

/// Parses the input grid into a HashMap for efficient lookup
fn parse_grid(input: &str) -> HashMap<(i32, i32), char> {
    let mut lut = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    for c in input.chars() {
        match c {
            'X' | 'M' | 'A' | 'S' => {
                lut.insert((x, y), c);
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 0;
            }
            _ => x += 1,
        }
    }
    lut
}

/// Counts all X-MAS patterns in the LUT
fn count_xmas_patterns(lut: &HashMap<(i32, i32), char>) -> i32 {
    let mut count = 0;

    for (&pos, &ch) in lut {
        if ch == 'A' {
            count += is_xmas_pattern(pos, lut) as i32;
        }
    }

    count
}

/// Checks if the given position is the center of an X-MAS pattern
fn is_xmas_pattern(pos: (i32, i32), lut: &HashMap<(i32, i32), char>) -> bool {
    // Get diagonal characters
    let diagonals = [
        ((-1, -1), (-2, -2)), // Top-left
        ((1, 1), (2, 2)),     // Bottom-right
        ((-1, 1), (-2, 2)),   // Top-right
        ((1, -1), (2, -2)),   // Bottom-left
    ];

    // Check both diagonals for "MAS" or "SAM"
    let mut matches = 0;
    for &((step1_x, step1_y), (step2_x, step2_y)) in &diagonals {
        let first_char = get_char_at(pos, step1_x, step1_y, lut);
        let second_char = get_char_at(pos, step2_x, step2_y, lut);

        if matches_mas_or_sam(first_char, second_char) {
            matches += 1;
        }
    }

    matches == 2 // An "X-MAS" requires both diagonals to match
}

/// Fetches the character at the specified offset
fn get_char_at(
    pos: (i32, i32),
    offset_x: i32,
    offset_y: i32,
    lut: &HashMap<(i32, i32), char>,
) -> char {
    *lut.get(&(pos.0 + offset_x, pos.1 + offset_y)).unwrap_or(&'.')
}

/// Checks if the given characters form part of "MAS" or "SAM"
fn matches_mas_or_sam(first: char, second: char) -> bool {
    (first == 'M' && second == 'A') || (first == 'S' && second == 'A')
}
