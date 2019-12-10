// Day 9

use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let input_filename = "input.txt";
    if let Err(e) = run(input_filename) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    // Read the input file
    let contents = fs::read_to_string(filename)?;
    let asteroid_map: Vec<&str> = contents.trim().split("\n").collect();
    // Parse into a binary matrix
    let mut asteroid_matrix: Vec<Vec<u8>> = Vec::new();
    for line in asteroid_map {
        let row: Vec<u8> = line.chars().map(|c| parse_char(c)).collect();
        asteroid_matrix.push(row);
    }
    // Run through asteroid positions. For each position, work out
    // how many asteroids can be seen
    // Run through the nearest asteroids by euclidean distance first.
    // Put a mask on the asteroid position array that zeros every blocked position

    let asteroid_matrix = asteroid_matrix; // immutable
    println!("{:?}", asteroid_matrix);

    let mut max_number_of_seen_asteroids = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    for (i, row) in asteroid_matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 1 {
                let number_of_seen_asteroids = get_visible_asteroids(&asteroid_matrix, i, j);
                if number_of_seen_asteroids > max_number_of_seen_asteroids {
                    max_number_of_seen_asteroids = number_of_seen_asteroids;
                    best_i = i;
                    best_j = j;
                }
            }
            println!("At co-ord [{}, {}] is value: {}", i, j, value);
        }
    }

    Ok(())
}

fn get_visible_asteroids(asteroid_matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    // Run over 
    0
}
fn parse_char(c: char) -> u8 {
    let hash: char = "#".chars().next().unwrap();
    let dot: char = ".".chars().next().unwrap();
    if c == hash {
        1
    } else if c == dot {
        0
    } else {
        panic!("Unexpected char!")
    }
}
