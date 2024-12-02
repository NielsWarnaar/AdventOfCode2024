use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn day1() {
    let file = File::open("input/day1-input.txt").expect("Could not open file");
    let reader = io::BufReader::new(file);

    let (mut left_side, mut right_side) = (Vec::new(), Vec::new());

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split(' ').collect();
        left_side.push(parts[0].parse::<i32>().unwrap());
        right_side.push(parts[3].parse::<i32>().unwrap());
    }

    left_side.sort();
    right_side.sort();

    let total_difference: i32 = left_side.iter().zip(&right_side)
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Total difference: {}", total_difference);

    let similarity_score: i32 = left_side.iter()
        .map(|left| left * right_side.iter().filter(|&&x| x == *left).count() as i32)
        .sum();

    println!("Similarity score: {}", similarity_score);
}