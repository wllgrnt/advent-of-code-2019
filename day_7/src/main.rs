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

    let instruction_set: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", instruction_set);

    // Copy the instruction set to each Amplifier
    let mut amplifier_a: Amplifier = Amplifier::new(instruction_set.clone());

    let input_value = 0;
    let phase_value = 0;
    let output_value = amplifier_a.run_tape(input_value, phase_value);

    println!("{}", output_value);

    Ok(())
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

#[derive(Debug)]
struct Amplifier {
    memory: Vec<i32>,
    input_signal: Option<i32>,
    phase_signal: Option<i32>,
    output_signal: Option<i32>,
    cursor: usize,
}

impl Amplifier {
    fn new(mut memory: Vec<i32>) -> Amplifier {
        let input_signal = None;
        let phase_signal = None;
        let output_signal = None;
        let mut cursor: usize = 0;
        Amplifier {
            memory: memory,
            input_signal: input_signal,
            phase_signal: phase_signal,
            output_signal: output_signal,
            cursor,
        }
    }

    fn run_tape(&mut self, input_value: i32, phase_value: i32) -> () {
        loop {
            // Read the memory at the cursor position, and parse the opcode.
            println!("cursor position: {}", self.cursor);
            let instruction = Instruction::new(&self.memory, self.cursor);
            println!("{:?}", instruction);
            let prev_cursor = self.cursor;

            // We need a way to get the two inputs in when required, and extract the output.
            self.process_instruction(&mut self.memory, &instruction, &mut self.cursor);
            if self.cursor == prev_cursor {
                self.cursor += &instruction.parameters.len() + 1; // +1 to include the opcode
            }
        }
    }

    fn process_instruction(
        &mut self,
        memory: &mut Vec<i32>,
        instruction: &Instruction,
        cursor: &mut usize,
    ) -> () {
        // match of the opcode
        match instruction.opcode {
            OpcodeKind::Add => {
                // parameters are [noun, verb, target]
                let noun_mode = instruction.modes[0];
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
                let result = noun_value + verb_value;
                let result_position = instruction.parameters[2] as usize;
                memory[result_position] = result;
            }
            OpcodeKind::Multiply => {
                // parameters are [noun, verb, target]
                let noun_mode = instruction.modes[0];
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
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
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
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
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
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
}
