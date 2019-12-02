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
    let contents = fs::read_to_string(filename)?;

    let lines = contents.lines();
    let input_data = &mut vec![];
    for line in lines {
        input_data.push(line.trim().parse::<i64>()?);
    }

    // for each mass in input_data, calculate the fuel required
    // then sum
    let total_fuel_required = input_data
        .into_iter()
        .fold(0, |acc, mass| acc + fuel_from_mass(*mass));

    println!("Total fuel required: {}", total_fuel_required);

    // Part two: fuel itself has mass, so needs fuel

    let total_fuel_required_refined = input_data.into_iter().fold(0, |acc, mass| {
        acc + fuel_from_mass_with_fuel_mass_included(*mass)
    });

    println!(
        "Total fuel required (refined): {}",
        total_fuel_required_refined
    );

    Ok(())
}

fn fuel_from_mass(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    fuel
}

fn fuel_from_mass_with_fuel_mass_included(mass: i64) -> i64 {
    // Calculate the fuel required for the module
    let mut required_fuel = fuel_from_mass(mass);
    // Then treat the fuel amount as the input mass, and repeat, until the
    // fuel amount is zero or negative
    let mut remaining_fuel = required_fuel;
    while remaining_fuel >= 0 {
        remaining_fuel = fuel_from_mass(remaining_fuel);
        if remaining_fuel > 0 {
            required_fuel += remaining_fuel;
        }
    }
    required_fuel
}
