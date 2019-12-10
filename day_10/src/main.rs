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

    get_visible_asteroids(asteroid_matrix.clone(), 5, 3);

    // let mut max_number_of_seen_asteroids = 0;
    // let mut best_i = 0;
    // let mut best_j = 0;
    // for (i, row) in asteroid_matrix.iter().enumerate() {
    //     for (j, value) in row.iter().enumerate() {
    //         if *value == 1 {
    //             let number_of_seen_asteroids = get_visible_asteroids(asteroid_matrix.clone(), i, j);
    //             if number_of_seen_asteroids > max_number_of_seen_asteroids {
    //                 max_number_of_seen_asteroids = number_of_seen_asteroids;
    //                 best_i = i;
    //                 best_j = j;
    //             }
    //         }
    //         println!("At co-ord [{}, {}] is value: {}", i, j, value);
    //     }
    // }

    Ok(())
}

fn get_visible_asteroids(
    mut asteroid_matrix: Vec<Vec<u8>>,
    asteroid_i: usize,
    asteroid_j: usize,
) -> u32 {
    // Spiral out from position asteroid_x, asteroid_y. If we spot an asteroid,
    // Then zero out all the blocked  positions (by repeating the relative offset until we reach the edge)

    // Iterate over rows, from row asteroid_i, asteroid_i+1, ... N then asteroid_i-1, asteroid_i-2, .. 0
    let n = asteroid_matrix.len();
    let m = asteroid_matrix[0].len();
    for i in get_indices(asteroid_i, n) {
        for j in get_indices(asteroid_j, m) {
            println!("{}, {}", i, j );
        }
        println!("");
    }
    0
}

fn get_indices(index: usize, limit: usize) -> Vec<usize> {
    let front_indices: Vec<usize> = (index..limit).collect();
    let mut back_indices: Vec<usize> = (0..index).collect();
    back_indices.reverse();
    [front_indices, back_indices].concat()
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
