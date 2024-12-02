use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub(crate) fn day2() {
    let file = File::open("input/day2-input.txt").expect("Could not open file");
    let reader = io::BufReader::new(file);

    let safe_reports = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let levels = parse_report(&line);
            let is_safe = is_safe_report(&levels);
            let can_be_safe = !is_safe && can_become_safe_with_removal(&levels);
            is_safe || can_be_safe
        })
        .filter(|&result| result)
        .count();

    println!("Safe reports: {}", safe_reports);
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let first_diff = levels[1] - levels[0];
    let should_increase = first_diff > 0;

    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }

    levels.windows(2).all(|window| {
        let diff = window[1] - window[0];
        diff.abs() >= 1 && diff.abs() <= 3 && (should_increase && diff > 0 || !should_increase && diff < 0)
    })
}

fn can_become_safe_with_removal(levels: &[i32]) -> bool {
    if levels.len() < 3 {
        return false;
    }

    (0..levels.len()).any(|skip_index| {
        let modified_levels: Vec<i32> = levels.iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_index)
            .map(|(_, &x)| x)
            .collect();
        is_safe_report(&modified_levels)
    })
}

fn parse_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|num| i32::from_str(num).ok())
        .collect()
}