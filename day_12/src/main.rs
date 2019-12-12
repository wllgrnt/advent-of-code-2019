// Day 12

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
    loop {

        update_velocities(&mut velocities, &mut positions);
        update_positions(&mut velocities, &mut positions);

        println! {"Step number {}", t};
        for i in 0..velocities.len() {
            println!(
                "Position: {:?}  Velocity: {:?}",
                positions[i], velocities[i]
            );
        }

        let potential_energies = get_potential_energy(&positions);
        let kinetic_energies = get_kinetic_energy(&velocities);
        let mut total_energy = 0;
        
        for i in 0..potential_energies.len() {
            let e_p = potential_energies[i];
            let e_k = kinetic_energies[i];
            total_energy += e_p*e_k;
        }
        println!("Total energy: {}", total_energy);
        println!("");

        if t == 1000 {
            break;
        }
        t += 1;
    }
    Ok(())
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
