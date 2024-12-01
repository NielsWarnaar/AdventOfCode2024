use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "puzzle-1-input.txt";

    // Open the file and create a buffered reader
    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    // Iterate over lines and parse them
    for line in reader.lines() {
        let line = line.expect("Could not read line"); // Handle potential IO errors
        let parts: Vec<&str> = line.split(' ').collect(); // Split by space
        println!("{:?}", parts); // Example: print parsed parts
    }
}
