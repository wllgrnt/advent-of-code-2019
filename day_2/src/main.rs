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
    // Convert comma-separated string to vector of ints
    let mut input_data: Vec<i64> = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // Replace position 1 with the value 12 and position 2 with the value 2
    // (see problem statement)

    input_data[1] = 12;
    input_data[2] = 2;

    /* Program logic
    - Read ints in sets of four
    - int 0 is the opcode (1 = add, 2=multiply, 99=exit)
    - int 1,2,3 are the references for the  two operands and the target for the sum/multiplication
    */

    for i in (0..input_data.len()).step_by(4) {
        let opcode = input_data[i];
        if opcode == 99 {
            break;
        }
        let left_op_ref: usize = input_data[i + 1] as usize;
        let right_op_ref: usize = input_data[i + 2] as usize;
        let target_ref: usize = input_data[i + 3] as usize;
        if opcode == 1 {
            input_data[target_ref] = input_data[left_op_ref] + input_data[right_op_ref];
        } else if opcode == 2 {
            input_data[target_ref] = input_data[left_op_ref] * input_data[right_op_ref];
        }
    }

    // Exercise wants to know what's at position 0 at the end

    println!("{}", input_data[0]);

    Ok(())
}