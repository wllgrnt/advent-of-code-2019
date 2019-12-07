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

    let contents = "3,225,1,225,6,6,1100,1,238,225,104,0,1102,46,47,225,2,122,130,224,101,-1998,224,224,4,224,1002,223,8,223,1001,224,6,224,1,224,223,223,1102,61,51,225,102,32,92,224,101,-800,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,61,64,225,1001,118,25,224,101,-106,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1102,33,25,225,1102,73,67,224,101,-4891,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,14,81,225,1102,17,74,225,1102,52,67,225,1101,94,27,225,101,71,39,224,101,-132,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1002,14,38,224,101,-1786,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1,65,126,224,1001,224,-128,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,81,40,224,1001,224,-121,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,226,224,1002,223,2,223,1005,224,329,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,344,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,374,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,419,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,434,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,449,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,677,226,224,1002,223,2,223,1006,224,479,1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,494,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,524,1001,223,1,223,7,226,226,224,102,2,223,223,1005,224,539,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,554,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,226,677,224,1002,223,2,223,1005,224,584,1001,223,1,223,1008,226,226,224,1002,223,2,223,1006,224,599,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,614,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,659,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";
    let instruction_set: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", instruction_set);

    // Copy the instruction set to each Amplifier
    let mut amplifier_a: Amplifier = Amplifier::new(instruction_set.clone());

    let input_value = 5;
    let phase_value = 0;
    let output_value = amplifier_a.run_tape(input_value, phase_value);

    // println!("{}", output_value);

    Ok(())
}

// Mapping of opcodes to instructions.
// Each opcode has a name and an associated
// number of instuctions
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    fn new(memory: Vec<i32>) -> Amplifier {
        let input_signal = None;
        let phase_signal = None;
        let output_signal = None;
        let cursor: usize = 0;
        Amplifier {
            memory: memory,
            input_signal: input_signal,
            phase_signal: phase_signal,
            output_signal: output_signal,
            cursor,
        }
    }

    fn run_tape(&mut self, input_value: i32, phase_value: i32) -> () {
        let mut phase_value_processed = true;
        loop {
            // Read the memory at the cursor position, and parse the opcode.
            println!("cursor position: {}", self.cursor);
            let instruction = Instruction::new(&self.memory, self.cursor);
            println!("{:?}", instruction);
            let prev_cursor = self.cursor;

            // We need a way to get the two inputs in when required, and extract the output.
            if instruction.opcode == OpcodeKind::Input && !phase_value_processed {
                self.process_instruction(&instruction, Some(phase_value));
                phase_value_processed = true;
            } else if instruction.opcode == OpcodeKind::Input && phase_value_processed {
                self.process_instruction(&instruction, Some(input_value));
            } else {
                self.process_instruction(&instruction, None);
            }
            if self.cursor == prev_cursor {
                self.cursor += &instruction.parameters.len() + 1; // +1 to include the opcode
            }
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction, input_value: Option<i32>) -> () {
        // match of the opcode
        match instruction.opcode {
            OpcodeKind::Add => {
                // parameters are [noun, verb, target]
                let noun_mode = instruction.modes[0];
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        self.memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
                let result = noun_value + verb_value;
                let result_position = instruction.parameters[2] as usize;
                self.memory[result_position] = result;
            }
            OpcodeKind::Multiply => {
                // parameters are [noun, verb, target]
                let noun_mode = instruction.modes[0];
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        self.memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
                let result = noun_value * verb_value;
                let result_position = instruction.parameters[2] as usize;
                self.memory[result_position] = result;
            }
            OpcodeKind::Input => {
                let input = input_value.unwrap();
                // Write input to the memory in position given by the parameter
                let position: usize = instruction.parameters[0] as usize;
                self.memory[position] = input;
            }
            OpcodeKind::Output => {
                let mode = instruction.modes[0];
                if mode == 0 {
                    let position: usize = instruction.parameters[0] as usize;
                    let value = self.memory[position];
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
                    condition = self.memory[position] != 0;
                } else if mode == 1 {
                    condition = instruction.parameters[0] != 0;
                }
                if condition {
                    if instruction.modes[1] == 0 {
                        let position: usize = instruction.parameters[1] as usize;
                        self.cursor = self.memory[position] as usize;
                    } else if instruction.modes[1] == 1 {
                        self.cursor = instruction.parameters[1] as usize;
                    }
                }
            }
            OpcodeKind::JumpIfFalse => {
                let mode = instruction.modes[0];
                let mut condition: bool = false;
                if mode == 0 {
                    let position: usize = instruction.parameters[0] as usize;
                    condition = self.memory[position] == 0;
                } else if mode == 1 {
                    condition = instruction.parameters[0] == 0;
                }
                if condition {
                    if instruction.modes[1] == 0 {
                        let position: usize = instruction.parameters[1] as usize;
                        self.cursor = self.memory[position] as usize;
                    } else if instruction.modes[1] == 1 {
                        self.cursor = instruction.parameters[1] as usize;
                    }
                }
            }
            OpcodeKind::IsLessThan => {
                let noun_mode = instruction.modes[0];
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        self.memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
                if noun_value < verb_value {
                    let result_position = instruction.parameters[2] as usize;
                    self.memory[result_position] = 1;
                } else {
                    let result_position = instruction.parameters[2] as usize;
                    self.memory[result_position] = 0;
                }
            }
            OpcodeKind::IsEquals => {
                let noun_mode = instruction.modes[0];
                let noun_value = match noun_mode {
                    0 => {
                        let noun_position = instruction.parameters[0] as usize;
                        self.memory[noun_position]
                    }
                    1 => instruction.parameters[0],
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    _ => panic!("unexpected mode"),
                };
                if noun_value == verb_value {
                    let result_position = instruction.parameters[2] as usize;
                    self.memory[result_position] = 1;
                } else {
                    let result_position = instruction.parameters[2] as usize;
                    self.memory[result_position] = 0;
                }
            }
            OpcodeKind::Exit => {
                std::process::exit(0);
            }
        }
    }
}
