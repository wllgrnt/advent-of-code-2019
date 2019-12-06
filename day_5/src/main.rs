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
    // let contents = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    // let contents = "3,9,7,9,10,9,4,9,99,-1,8";
    let mut instruction_set: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", instruction_set);

    let mut cursor: usize = 0;
    run_tape(instruction_set, cursor);

    Ok(())
}

fn run_tape(mut memory: Vec<i32>, mut cursor: usize) -> () {
    loop {
        // Read the memory at the cursor position, and parse the opcode.
        println!("cursor position: {}", cursor);
        let instruction = Instruction::new(&mut memory, cursor);
        println!("{:?}", instruction);
        let prev_cursor = cursor;
        process_instruction(&mut memory, &instruction, &mut cursor);
        if cursor == prev_cursor {
            cursor += &instruction.parameters.len() + 1; // +1 to include the opcode
        }
    }
}

fn process_instruction(memory: &mut Vec<i32>, instruction: &Instruction, cursor: &mut usize) -> () {
    // match of the opcode

    match instruction.opcode {
        OpcodeKind::Add => {
            // parameters are [noun, verb, target]
            let noun_mode = instruction.modes[0];
            let mut noun_value = -1; // THIS IS DANGEROUS
            let mut verb_value = -1;

            if noun_mode == 0 {
                // then it is position mode
                let noun_position = instruction.parameters[0] as usize;
                noun_value = memory[noun_position];
            } else if noun_mode == 1 {
                // then it is immediate mode
                noun_value = instruction.parameters[0];
            } else {
                panic!("unexpected mode")
            }
            let verb_mode = instruction.modes[1];
            if verb_mode == 0 {
                // then it is position mode
                let verb_position = instruction.parameters[1] as usize;
                verb_value = memory[verb_position];
            } else if verb_mode == 1 {
                // then it is immediate mode
                verb_value = instruction.parameters[1];
            } else {
                panic!("unexpected mode")
            }

            let result = noun_value + verb_value;
            let result_position = instruction.parameters[2] as usize;
            memory[result_position] = result;
        }
        OpcodeKind::Multiply => {
            // parameters are [noun, verb, target]
            let noun_mode = instruction.modes[0];
            let mut noun_value = -1; // THIS IS DANGEROUS
            let mut verb_value = -1;

            if noun_mode == 0 {
                // then it is position mode
                let noun_position = instruction.parameters[0] as usize;
                noun_value = memory[noun_position];
            } else if noun_mode == 1 {
                // then it is immediate mode
                noun_value = instruction.parameters[0];
            } else {
                panic!("unexpected mode")
            }
            let verb_mode = instruction.modes[1];
            if verb_mode == 0 {
                // then it is position mode
                let verb_position = instruction.parameters[1] as usize;
                verb_value = memory[verb_position];
            } else if verb_mode == 1 {
                // then it is immediate mode
                verb_value = instruction.parameters[1];
            } else {
                panic!("unexpected mode")
            }

            let result = noun_value * verb_value;
            let result_position = instruction.parameters[2] as usize;
            memory[result_position] = result;
        }
        OpcodeKind::Input => {
            println!("Please give program input.");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            // Write input to the memory in position given by the parameter
            let position: usize = instruction.parameters[0] as usize;
            memory[position] = input.trim().parse().unwrap();
        }
        OpcodeKind::Output => {
            let mode = instruction.modes[0];
            if mode == 0 {
                let position: usize = instruction.parameters[0] as usize;
                let value = memory[position];
                println!("***********Instruction output: {}", value);
            } else if mode == 1 {
                let value = instruction.parameters[0];
                println!("***********Instruction output: {}", value);
            }
        }

        OpcodeKind::JumpIfTrue => {
            let mode = instruction.modes[0];
            let mut condition: bool = false;
            if mode == 0 {
                let position: usize = instruction.parameters[0] as usize;
                condition = memory[position] != 0;
            } else if mode == 1 {
                condition = instruction.parameters[0] != 0;
            }
            if condition {
                if instruction.modes[1] == 0 {
                    let position: usize = instruction.parameters[1] as usize;
                    *cursor = memory[position] as usize;
                } else if instruction.modes[1] == 1 {
                    *cursor = instruction.parameters[1] as usize;
                }
            }
        }
        OpcodeKind::JumpIfFalse => {
            let mode = instruction.modes[0];
            let mut condition: bool = false;
            if mode == 0 {
                let position: usize = instruction.parameters[0] as usize;
                condition = memory[position] == 0;
            } else if mode == 1 {
                condition = instruction.parameters[0] == 0;
            }
            if condition {
                if instruction.modes[1] == 0 {
                    let position: usize = instruction.parameters[1] as usize;
                    *cursor = memory[position] as usize;
                } else if instruction.modes[1] == 1 {
                    *cursor = instruction.parameters[1] as usize;
                }
            }
        }
        OpcodeKind::IsLessThan => {
            let noun_mode = instruction.modes[0];
            let mut noun_value = -1; // THIS IS DANGEROUS
            let mut verb_value = -1;

            if noun_mode == 0 {
                // then it is position mode
                let noun_position = instruction.parameters[0] as usize;
                noun_value = memory[noun_position];
            } else if noun_mode == 1 {
                // then it is immediate mode
                noun_value = instruction.parameters[0];
            } else {
                panic!("unexpected mode")
            }
            let verb_mode = instruction.modes[1];
            if verb_mode == 0 {
                // then it is position mode
                let verb_position = instruction.parameters[1] as usize;
                verb_value = memory[verb_position];
            } else if verb_mode == 1 {
                // then it is immediate mode
                verb_value = instruction.parameters[1];
            } else {
                panic!("unexpected mode")
            }

            if noun_value < verb_value {
                let result_position = instruction.parameters[2] as usize;
                memory[result_position] = 1;
            } else {
                let result_position = instruction.parameters[2] as usize;
                memory[result_position] = 0;
            }
        }
        OpcodeKind::IsEquals => {
            let noun_mode = instruction.modes[0];
            let mut noun_value = -1; // THIS IS DANGEROUS
            let mut verb_value = -1;

            if noun_mode == 0 {
                // then it is position mode
                let noun_position = instruction.parameters[0] as usize;
                noun_value = memory[noun_position];
            } else if noun_mode == 1 {
                // then it is immediate mode
                noun_value = instruction.parameters[0];
            } else {
                panic!("unexpected mode")
            }
            let verb_mode = instruction.modes[1];
            if verb_mode == 0 {
                // then it is position mode
                let verb_position = instruction.parameters[1] as usize;
                verb_value = memory[verb_position];
            } else if verb_mode == 1 {
                // then it is immediate mode
                verb_value = instruction.parameters[1];
            } else {
                panic!("unexpected mode")
            }

            if noun_value == verb_value {
                let result_position = instruction.parameters[2] as usize;
                memory[result_position] = 1;
            } else {
                let result_position = instruction.parameters[2] as usize;
                memory[result_position] = 0;
            }
        }
        OpcodeKind::Exit => {
            std::process::exit(0);
        }
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
    JumpIfTrue,
    JumpIfFalse,
    IsLessThan,
    IsEquals,
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
        int_to_operation_map.insert(5, OpcodeKind::JumpIfTrue);
        int_to_operation_map.insert(6, OpcodeKind::JumpIfFalse);
        int_to_operation_map.insert(7, OpcodeKind::IsLessThan);
        int_to_operation_map.insert(8, OpcodeKind::IsEquals);
        int_to_operation_map.insert(99, OpcodeKind::Exit);

        // Make immutable
        let int_to_operation_map = int_to_operation_map;

        match opcode_value {
            99 => {
                let opcode: OpcodeKind = *int_to_operation_map.get(&99).expect("Opcode not found!");
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
                let mut digits = get_digits(opcode_value).into_iter().rev();
                let opcode_int = digits.next().unwrap().clone();
                let opcode: OpcodeKind = *int_to_operation_map
                    .get(&opcode_int)
                    .expect("Opcode not found!");
                let zero = digits.next();
                match zero {
                    Some(0) => {
                        let mut modes: Vec<i32> = digits.collect();
                        while modes.len() < 3 {
                            modes.push(0);
                        }
                        let parameters: Vec<i32> = match opcode_int {
                            1 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            2 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            4 => vec![memory[cursor + 1]],
                            5 => vec![memory[cursor + 1], memory[cursor + 2]],
                            6 => vec![memory[cursor + 1], memory[cursor + 2]],
                            7 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            8 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            _ => panic!("opcode not found!"),
                        };

                        return Instruction {
                            opcode_value: opcode_value,
                            opcode: opcode,
                            modes: modes,
                            parameters: parameters,
                        };
                    }
                    None => {
                        let modes: Vec<i32> = vec![0, 0, 0];
                        let parameters: Vec<i32> = match opcode_int {
                            1 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            2 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            3 => vec![memory[cursor + 1]],
                            4 => vec![memory[cursor + 1]],
                            5 => vec![memory[cursor + 1], memory[cursor + 2]],
                            6 => vec![memory[cursor + 1], memory[cursor + 2]],
                            7 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            8 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            _ => panic!("opcode not found!"),
                        };

                        return Instruction {
                            opcode_value: opcode_value,
                            opcode: opcode,
                            modes: modes,
                            parameters: parameters,
                        };
                    }
                    _ => {
                        println!("{}", opcode_value);
                        panic!("crash and burn")
                    }
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
