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

    let mut max_number_of_seen_asteroids = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    for (i, row) in asteroid_matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 1 {
                let number_of_seen_asteroids = get_visible_asteroids(asteroid_matrix.clone(), i, j);
                if number_of_seen_asteroids > max_number_of_seen_asteroids {
                    max_number_of_seen_asteroids = number_of_seen_asteroids;
                    best_i = i;
                    best_j = j;
                }
            }
        }
    }

    println!(
        "Top asteroid is at {}, {} and sees {} asteroids",
        best_i, best_j, max_number_of_seen_asteroids
    );

    // Part 2. We deploy a laser at the best_i, best_j co-ordinates.
    // It starts pointing vertically, then rotates clockwise vaporising any asteroids it can see.
    // What is the 200th asteroid to be vaporised?

    // Algo:
    // - Get the visible asteroids. Remove them in order.
    // - Calculate the newly visible asteroids. Remove them.
    // - Repeat.
    vaporise_asteroids(asteroid_matrix.clone(), best_i, best_j);
    Ok(())
}

fn vaporise_asteroids(mut asteroid_matrix: Vec<Vec<u8>>, laser_i: usize, laser_j: usize) -> () {
    while count_nonzero(&asteroid_matrix) > 1 {
        // Get the matrix of visible asteroids.
        let visible_asteroid_indices =
            get_visible_asteroid_indices(asteroid_matrix.clone(), laser_i, laser_j);
        
        // sort these visible asteroid indices according to the angle between them and the laser.

        let visible_asteroid_indices = sort_indices_by_angle(visible_asteroid_indices, laser_i, laser_j);

        for index_array in visible_asteroid_indices {
            asteroid_matrix[index_array[0]][index_array[1]] = 0;
        }
            
    }
    // Get the
}


fn sort_indices_by_angle(asteroid_indices: Vec<[usize;2]>, laser_i: usize, laser_j: usize) -> Vec<[usize;2]> {

    // For each [i,j] pair, get the angle between vertical and the [i,j] pair from the laser position.
    // Once we have these angles, sort according to them and return a sorted array.
    
    let mut angles: Vec<f32> = Vec::new();

    for index in &asteroid_indices {
        let relative_i: f32  = (laser_i as i32 - index[0] as i32) as f32;
        let relative_j: f32  = (laser_j as i32 - index[1] as i32) as f32;
        // Whats the orientation of this vector?
        // tan theta = -rel_i/rel_j. Tan monotonic so just store -rel_i/rel_j
        let angle:f32 = -relative_i/relative_j;
        angles.push(angle);
    }

    // Sort according to angle.
    

    asteroid_indices
}


fn gcd(m: i8, n: i8) -> i8 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}

fn get_visible_asteroids(
    mut asteroid_matrix: Vec<Vec<u8>>,
    asteroid_i: usize,
    asteroid_j: usize,
) -> u32 {
    /*  Spiral out from position asteroid_x, asteroid_y. If we spot an asteroid,
        Then zero out all the blocked  positions (by repeating the relative offset until we reach the edge)
        Iterate over rows, from row asteroid_i, asteroid_i+1, ... N then asteroid_i-1, asteroid_i-2, .. 0
    */
    // Check there is an asteroid at the specified position.
    assert_eq!(asteroid_matrix[asteroid_i][asteroid_j], 1);
    let n = asteroid_matrix.len();
    let m = asteroid_matrix[0].len();
    for i in get_indices(asteroid_i, n) {
        for j in get_indices(asteroid_j, m) {
            if i == asteroid_i && j == asteroid_j {
                continue;
            }

            if asteroid_matrix[i][j] == 1 {
                // Then calculate the offset between this asteroid and asteroid_i, asteroid_j.
                // Set all blocked positions to zero.
                hide_blocked_asteroids(&mut asteroid_matrix, asteroid_i, asteroid_j, i, j);
            }
        }
    }
    // Now we should have gotten rid of all asteroids that can't be seen from (asteroid_i, asteroid_j).
    // So just count the non-zero elements of the asteroid matrix (minus 1, for the asteroid itself)
    count_nonzero(&asteroid_matrix) - 1
}

fn get_visible_asteroid_indices(
    mut asteroid_matrix: Vec<Vec<u8>>,
    asteroid_i: usize,
    asteroid_j: usize,
) -> Vec<[usize; 2]> {
    /*  Spiral out from position asteroid_x, asteroid_y. If we spot an asteroid,
        Then zero out all the blocked  positions (by repeating the relative offset until we reach the edge)
        Iterate over rows, from row asteroid_i, asteroid_i+1, ... N then asteroid_i-1, asteroid_i-2, .. 0
    */
    assert_eq!(asteroid_matrix[asteroid_i][asteroid_j], 1);
    let n = asteroid_matrix.len();
    let m = asteroid_matrix[0].len();
    for i in get_indices(asteroid_i, n) {
        for j in get_indices(asteroid_j, m) {
            if i == asteroid_i && j == asteroid_j {
                continue;
            }

            if asteroid_matrix[i][j] == 1 {
                // Then calculate the offset between this asteroid and asteroid_i, asteroid_j.
                // Set all blocked positions to zero.
                hide_blocked_asteroids(&mut asteroid_matrix, asteroid_i, asteroid_j, i, j);
            }
        }
    }
    // Now we should have gotten rid of all asteroids that can't be seen from (asteroid_i, asteroid_j).
    // So just get the indices of the non-zero elements

    let mut non_zero_indices: Vec<[usize; 2]> = Vec::new();

    for (i, row) in asteroid_matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if i == asteroid_i && j == asteroid_j {
                continue
            }
            if *value == 1 {
                non_zero_indices.push([i, j])
            }
        }
    }
    non_zero_indices
}

fn count_nonzero(asteroid_matrix: &Vec<Vec<u8>>) -> u32 {
    let mut nonzero = 0;
    for row in asteroid_matrix.iter() {
        for value in row.iter() {
            if *value != 0 {
                nonzero += 1;
            }
        }
    }
    nonzero
}

fn hide_blocked_asteroids(
    asteroid_matrix: &mut Vec<Vec<u8>>,
    asteroid_i: usize,
    asteroid_j: usize,
    i: usize,
    j: usize,
) -> () {
    // If the asteroid is on the same row or column, zero the remaining row and column.
    // Needs guard rails to stop accessing elements <0 or >m
    let n = asteroid_matrix.len();
    let m = asteroid_matrix[0].len();
    if asteroid_i == i {
        if j > asteroid_j && j < m {
            for blocked_j in j + 1..m {
                asteroid_matrix[i][blocked_j] = 0;
            }
        } else if j > 0 {
            for blocked_j in 0..j - 1 {
                asteroid_matrix[i][blocked_j] = 0;
            }
        }
    } else if asteroid_j == j {
        if i > asteroid_i && i < n {
            for blocked_i in i + 1..n {
                asteroid_matrix[blocked_i][j] = 0;
            }
        } else if i > 0 {
            for blocked_i in 0..i - 1 {
                asteroid_matrix[blocked_i][j] = 0;
            }
        }
    }
    // Otherwise, zero the positions at a repeat of the offset.
    // If the offsets have a common factor, then simplify them.
    // E.g. if the offset is (3,3) then everything (1,1) from there is blocked.
    let mut offset_i: i8 = i as i8 - asteroid_i as i8;
    let mut offset_j: i8 = j as i8 - asteroid_j as i8;
    let gcd = gcd(offset_i.abs(), offset_j.abs());
    offset_i /= gcd;
    offset_j /= gcd;
    // While within the limits of the asteroid matrix, zero things out
    let mut blocked_i: i8 = i as i8 + offset_i;
    let mut blocked_j: i8 = j as i8 + offset_j;
    while blocked_i >= 0 && blocked_i < n as i8 && blocked_j >= 0 && blocked_j < m as i8 {
        asteroid_matrix[blocked_i as usize][blocked_j as usize] = 0;
        blocked_i += offset_i;
        blocked_j += offset_j;
    }
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
