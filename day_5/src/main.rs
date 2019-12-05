// Day 5

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
    let mut instruction_set: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", instruction_set);

    let input_value: i32 = 1; // see problem statement
    let mut cursor: usize = 0;
    run_tape(instruction_set, cursor, input_value);

    Ok(())
}

fn run_tape(mut memory: Vec<i32>, mut cursor: usize, input_value: i32) -> () {
    loop {
        let instruction = Instruction::new(&memory, cursor);
        println!("{:?}", instruction);
        cursor += instruction.parameters.len() + 1; // +1 to include the opcode
    }
}

// Mapping of opcodes to instructions.
// Each opcode has a name and an associated
// number of instuctions
#[derive(Debug, Copy, Clone)]
enum OpcodeKind {
    Add,
    Multiply,
    Input,
    Output,
    Exit,
}

#[derive(Debug)]
struct Instruction {
    opcode_value: i32,
    opcode: OpcodeKind,
    modes: Vec<i32>,
    parameters: Vec<i32>,
}

impl Instruction {
    /* Read the opcode string.
    Parse the first two digits.
    Lookup the relevant OpcodeKind.
    Do stuff based on OpcodeKind.
    */
    fn new(memory: &Vec<i32>, cursor: usize) -> Instruction {
        let opcode_value: i32 = memory[cursor];

        let mut int_to_operation_map = std::collections::HashMap::new();
        int_to_operation_map.insert(1, OpcodeKind::Add);
        int_to_operation_map.insert(2, OpcodeKind::Multiply);
        int_to_operation_map.insert(3, OpcodeKind::Input);
        int_to_operation_map.insert(4, OpcodeKind::Output);
        int_to_operation_map.insert(99, OpcodeKind::Exit);

        // Make immutable
        let int_to_operation_map = int_to_operation_map;

        println!("{}", opcode_value );
        match opcode_value {
            3 => {
                let opcode: OpcodeKind = *int_to_operation_map.get(&3).expect("Opcode not found!");
                let modes: Vec<i32> = Vec::new();
                let parameters = vec![memory[cursor + 1]];
                return Instruction {
                    opcode_value: opcode_value,
                    opcode: opcode,
                    modes: modes,
                    parameters: parameters,
                };
            }
            4 => {
                let opcode: OpcodeKind = *int_to_operation_map.get(&4).expect("Opcode not found!");
                let modes: Vec<i32> = Vec::new();
                let parameters = vec![memory[cursor + 1]];
                return Instruction {
                    opcode_value: opcode_value,
                    opcode: opcode,
                    modes: modes,
                    parameters: parameters,
                };
            }
            99 => {
                let opcode: OpcodeKind = *int_to_operation_map.get(&4).expect("Opcode not found!");
                let modes: Vec<i32> = Vec::new();
                let parameters: Vec<i32> = Vec::new();
                return Instruction {
                    opcode_value: opcode_value,
                    opcode: opcode,
                    modes: modes,
                    parameters,
                };
            }
            _ => {
                // Then its 1 or 2.

                let mut digits = get_digits(opcode_value).into_iter().rev();
                let opcode = digits.next().unwrap().clone();
                let opcode: OpcodeKind = *int_to_operation_map
                    .get(&opcode)
                    .expect("Opcode not found!");
                
                let zero = digits.next();
                match zero {
                    Some(0) => {
                        let mut modes: Vec<i32> = digits.collect();
                        while modes.len() < 3 {
                            modes.push(0);
                        }
                        let parameters: Vec<i32> = vec![memory[cursor+1], memory[cursor+2], memory[cursor+3]];
                        return Instruction {
                            opcode_value: opcode_value,
                            opcode: opcode,
                            modes: modes,
                            parameters: parameters,
                        };
                    },
                    None => {
                        let modes: Vec<i32> = vec![0,0,0];
                        let parameters: Vec<i32> = vec![memory[cursor+1], memory[cursor+2], memory[cursor+3]];
                        return Instruction {
                            opcode_value: opcode_value,
                            opcode: opcode,
                            modes: modes,
                            parameters: parameters,
                        };
                    },
                    _ => panic!("crash and burn"),
                }
            }
        };

    }
}

fn get_digits(n: i32) -> Vec<i32> {
    fn x_inner(n: i32, xs: &mut Vec<i32>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}
