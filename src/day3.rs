use std::fs;
use regex::Regex;

pub(crate) fn day3() {
    let filename = "input/day3-input.txt";
    let input = fs::read_to_string(filename).expect("Failed to read the input file");

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut mul_enabled = true;
    let mut sum = 0;
    let mut index = 0;

    while index < input.len() {
        if let Some(do_match) = do_regex.find(&input[index..]) {
            if do_match.start() == 0 {
                mul_enabled = true;
                index += do_match.end();
                continue;
            }
        }
        if let Some(dont_match) = dont_regex.find(&input[index..]) {
            if dont_match.start() == 0 {
                mul_enabled = false;
                index += dont_match.end();
                continue;
            }
        }
        if let Some(mul_match) = mul_regex.captures(&input[index..]) {
            if mul_match.get(0).unwrap().start() == 0 {
                if mul_enabled {
                    let x: i32 = mul_match[1].parse().unwrap();
                    let y: i32 = mul_match[2].parse().unwrap();
                    sum += x * y;
                }
                index += mul_match.get(0).unwrap().end();
                continue;
            }
        }

        index += 1;
    }
    println!("The sum of all valid multiplications is: {}", sum);
}