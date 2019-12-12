// Day 12

use std::cmp::{max, min};
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
    // Read the input file: this is the Intcode program
    let contents = fs::read_to_string(filename)?;
    let mut positions: Vec<Vec<i32>> = contents
        .trim()
        .split("\n")
        .map(|x| parse_input_string(x))
        .collect();

    /* 4 planets have the initial positions as given, and velocity 0.
    For each time step, update the velocity by applying gravity
    The update the position by applying velocity
    */

    let mut velocities = vec![vec![0; 3]; 4];
    let mut t = 1;

    // Part 2: Determine the number of steps that must occur before all of the moons' positions and velocities exactly match a previous point in time.
    // The x, y and z coordinates are independent of each other, so the length of the cycle is the least common multiple of the lengths of the x, y and z cycle
    let initial_x_positions: Vec<i32> = positions.clone().into_iter().map(|d| d[0]).collect();
    let initial_y_positions: Vec<i32> = positions.clone().into_iter().map(|d| d[1]).collect();
    let initial_z_positions: Vec<i32> = positions.clone().into_iter().map(|d| d[2]).collect();

    let initial_x_velocities: Vec<i32> = vec![0; 4];
    let initial_y_velocities: Vec<i32> = vec![0; 4];
    let initial_z_velocities: Vec<i32> = vec![0; 4];

    let mut periods_found = [false, false, false];
    let mut peroids = [0, 0, 0];
    loop {
        update_velocities(&mut velocities, &mut positions);
        update_positions(&mut velocities, &mut positions);

        // println! {"Step number {}", t};
        // for i in 0..velocities.len() {
        //     println!(
        //         "Position: {:?}  Velocity: {:?}",
        //         positions[i], velocities[i]
        //     );
        // }

        let potential_energies = get_potential_energy(&positions);
        let kinetic_energies = get_kinetic_energy(&velocities);
        let mut total_energy = 0;
        for i in 0..potential_energies.len() {
            let e_p = potential_energies[i];
            let e_k = kinetic_energies[i];
            total_energy += e_p * e_k;
        }
        if t == 1000 {
            println!("Total energy after 1000 steps: {}", total_energy);
            println!("");
        }
        let x_positions: Vec<i32> = positions.clone().into_iter().map(|d| d[0]).collect();
        let y_positions: Vec<i32> = positions.clone().into_iter().map(|d| d[1]).collect();
        let z_positions: Vec<i32> = positions.clone().into_iter().map(|d| d[2]).collect();
        let x_velocities: Vec<i32> = velocities.clone().into_iter().map(|d| d[0]).collect();
        let y_velocities: Vec<i32> = velocities.clone().into_iter().map(|d| d[1]).collect();
        let z_velocities: Vec<i32> = velocities.clone().into_iter().map(|d| d[2]).collect();

        if !periods_found[0]
            && x_positions == initial_x_positions
            && x_velocities == initial_x_velocities
        {
            println!("x-Period: {}", t);
            periods_found[0] = true;
            peroids[0] = t;
        }
        if !periods_found[1]
            && y_positions == initial_y_positions
            && y_velocities == initial_y_velocities
        {
            println!("y-Period: {}", t);
            periods_found[1] = true;
            peroids[1] = t;
        }
        if !periods_found[2]
            && z_positions == initial_z_positions
            && z_velocities == initial_z_velocities
        {
            println!("z-Period: {}", t);
            periods_found[2] = true;
            peroids[2] = t;
        }
        if periods_found == [true, true, true] {
            println!(
                "Steps taken before positions and velocities back to initial state: {}",
                lcm(lcm(peroids[0], peroids[1]), peroids[2]) as i64
            );
            break;
        }
        t += 1;
    }
    Ok(())
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn get_potential_energy(p: &Vec<Vec<i32>>) -> Vec<i32> {
    // The potential energy is the sum of the absolute x,y,z values
    let mut potential_energy: Vec<i32> = Vec::new();
    for position in p {
        let mut e_p = 0;
        for axis in 0..3 {
            e_p += position[axis].abs();
        }
        potential_energy.push(e_p);
    }
    potential_energy
}

fn get_kinetic_energy(v: &Vec<Vec<i32>>) -> Vec<i32> {
    // The kinetic energy is the sum of the absolute x,y,z values
    let mut kinetic_energy: Vec<i32> = Vec::new();
    for velocity in v {
        let mut e_k = 0;
        for axis in 0..3 {
            e_k += velocity[axis].abs();
        }
        kinetic_energy.push(e_k)
    }
    kinetic_energy
}

fn update_positions(v: &mut Vec<Vec<i32>>, p: &mut Vec<Vec<i32>>) -> () {
    // Add the velocity of each moon to its own position
    for moon_index in 0..v.len() {
        for axis in 0..3 {
            p[moon_index][axis] += v[moon_index][axis];
        }
    }
}

fn update_velocities(v: &mut Vec<Vec<i32>>, p: &mut Vec<Vec<i32>>) -> () {
    // On each axis, the velocity of each moons change by exactly +1 or -1 to pull
    // the moons together, for every pair of moons

    for moon_index_i in 0..v.len() {
        let moon_position_i = &p[moon_index_i];
        for moon_index_j in 0..moon_index_i {
            let moon_position_j = &p[moon_index_j];
            for axis in 0..3 {
                if moon_position_i[axis] < moon_position_j[axis] {
                    v[moon_index_i][axis] += 1;
                    v[moon_index_j][axis] -= 1;
                } else if moon_position_i[axis] > moon_position_j[axis] {
                    v[moon_index_i][axis] -= 1;
                    v[moon_index_j][axis] += 1;
                }
            }
        }
    }
}

fn parse_input_string(input_string: &str) -> Vec<i32> {
    // input takes the form <x=-6, y=-5, z=-8>
    // remove the first and last characters (< and >)
    let output: Vec<i32> = input_string
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect();
    output
}
