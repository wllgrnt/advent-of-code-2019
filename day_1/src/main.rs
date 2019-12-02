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
    let mut input_data = vec![];
    for line in lines {
        input_data.push(line.trim().parse::<i64>()?);
    }

    // for each mass in input_data, calculate the fuel required
    // then sum
    let total_fuel_required = input_data
        .into_iter()
        .fold(0, |acc, mass| acc + fuel_from_mass(mass));

    println!("Total fuel required: {}", total_fuel_required);
    Ok(())
}

fn fuel_from_mass(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    fuel
}
