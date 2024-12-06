use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
                        .expect("Should be able to read the file.");
    // println!("{}", contents);    // displaying the contents of the file 
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in contents.lines() {
        if let Some((first, second)) = line.split_whitespace().collect::<Vec<_>>().split_first() {
            if let Some(second) = second.first() {
                list1.push(first.parse::<i32>().unwrap());
                list2.push(second.parse::<i32>().unwrap());
            }
        }
    }

    // println!("List 1: {:?}", list1);
    // println!("List 2: {:?}", list2);

    // sort the lists
    list1.sort();
    list2.sort();

    // println!("Sorted List 1: {:?}", list1);
    // println!("Sorted List 2: {:?}", list2);

    // get the distance for each index
    let distance_list: Vec<i32> = list1.iter()
        .zip(&list2)
        .map(|(a, b)| (a - b).abs())
        .collect();

    //  add up the distances and print it 
    let total_distance: i32 = distance_list.iter().sum();
    println!("Total distance : {}", total_distance);

    // computing similarity score
    let mut frequency_map = HashMap::new();
    for &num in &list2 {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let similarity_scores: Vec<i32> = list1.iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .collect();

    // calculating the total similary score
    let total_similarity_score: i32 = similarity_scores.iter().sum();
    println!("Total similarity score : {}", total_similarity_score)
}
