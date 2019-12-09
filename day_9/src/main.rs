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
    let instruction_set: Vec<i64> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let input_value = 2;
    let mut compiler: Compiler = Compiler::new(instruction_set.clone());
    compiler.run_tape(input_value);

    println!("Output: {:?}", compiler.output_signal);

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
    AdjustRelativeBase,
    Exit,
}

#[derive(Debug)]
struct Instruction {
    opcode_value: i64,
    opcode: OpcodeKind,
    modes: Vec<i64>,
    parameters: Vec<i64>,
}

impl Instruction {
    /* Read the opcode string.
    Parse the first two digits.
    Lookup the relevant OpcodeKind.
    Do stuff based on OpcodeKind.
    */
    fn new(memory: &Vec<i64>, cursor: usize) -> Instruction {
        let opcode_value: i64 = memory[cursor];

        let mut int_to_operation_map = std::collections::HashMap::new();
        int_to_operation_map.insert(1, OpcodeKind::Add);
        int_to_operation_map.insert(2, OpcodeKind::Multiply);
        int_to_operation_map.insert(3, OpcodeKind::Input);
        int_to_operation_map.insert(4, OpcodeKind::Output);
        int_to_operation_map.insert(5, OpcodeKind::JumpIfTrue);
        int_to_operation_map.insert(6, OpcodeKind::JumpIfFalse);
        int_to_operation_map.insert(7, OpcodeKind::IsLessThan);
        int_to_operation_map.insert(8, OpcodeKind::IsEquals);
        int_to_operation_map.insert(9, OpcodeKind::AdjustRelativeBase);
        int_to_operation_map.insert(99, OpcodeKind::Exit);

        // Make immutable
        let int_to_operation_map = int_to_operation_map;

        match opcode_value {
            99 => {
                let opcode: OpcodeKind = *int_to_operation_map.get(&99).expect("Opcode not found!");
                let modes: Vec<i64> = Vec::new();
                let parameters: Vec<i64> = Vec::new();
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
                        let mut modes: Vec<i64> = digits.collect();
                        while modes.len() < 3 {
                            modes.push(0);
                        }
                        let parameters: Vec<i64> = match opcode_int {
                            1 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            2 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            3 => vec![memory[cursor + 1]],
                            4 => vec![memory[cursor + 1]],
                            5 => vec![memory[cursor + 1], memory[cursor + 2]],
                            6 => vec![memory[cursor + 1], memory[cursor + 2]],
                            7 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            8 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            9 => vec![memory[cursor + 1]],
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
                        let modes: Vec<i64> = vec![0, 0, 0];
                        let parameters: Vec<i64> = match opcode_int {
                            1 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            2 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            3 => vec![memory[cursor + 1]],
                            4 => vec![memory[cursor + 1]],
                            5 => vec![memory[cursor + 1], memory[cursor + 2]],
                            6 => vec![memory[cursor + 1], memory[cursor + 2]],
                            7 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            8 => vec![memory[cursor + 1], memory[cursor + 2], memory[cursor + 3]],
                            9 => vec![memory[cursor + 1]],
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

fn get_digits(n: i64) -> Vec<i64> {
    fn x_inner(n: i64, xs: &mut Vec<i64>) {
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
struct Compiler {
    memory: Vec<i64>,
    output_signal: Option<i64>,
    cursor: usize,
    relative_base: i64,
}

impl Compiler {
    fn new(mut memory: Vec<i64>) -> Compiler {
        let output_signal = None;
        let cursor: usize = 0;
        let relative_base: i64 = 0;
        // Day9 Feature: Extend the memory ("much larger than the initial program")
        let mut memory_extension: Vec<i64> = vec![0; 1000];
        memory.append(&mut memory_extension);
        Compiler {
            memory: memory,
            output_signal: output_signal,
            cursor: cursor,
            relative_base: relative_base,
        }
    }

    fn run_tape(&mut self, input_value: i64) -> () {
        loop {
            // Read the memory at the cursor position, and parse the opcode.
            let instruction = Instruction::new(&self.memory, self.cursor);
            let prev_cursor = self.cursor;
            // We need a way to get the two inputs in when required, and extract the output.
            match instruction.opcode {
                OpcodeKind::Input => self.process_instruction(&instruction, Some(input_value)),
                OpcodeKind::Exit => break,
                _ => self.process_instruction(&instruction, None),
            }
            if self.cursor == prev_cursor {
                self.cursor += &instruction.parameters.len() + 1; // +1 to include the opcode
            }
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction, input_value: Option<i64>) -> () {
        // match of the opcode.
        // We have new mode '2' which refers to itself plus the current relative base.
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
                    2 => {
                        let noun_position = instruction.parameters[0] + self.relative_base;
                        self.memory[noun_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    2 => {
                        let verb_position = instruction.parameters[1] + self.relative_base;
                        self.memory[verb_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                let result = noun_value + verb_value;
                let result_position = match instruction.modes[2] {
                    0 => instruction.parameters[2] as usize,
                    2 => (instruction.parameters[2] + self.relative_base) as usize,
                    _ => panic!("unexpected mode!"),
                };
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
                    2 => {
                        let noun_position = instruction.parameters[0] + self.relative_base;
                        self.memory[noun_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    2 => {
                        let verb_position = instruction.parameters[1] + self.relative_base;
                        self.memory[verb_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                let result = noun_value * verb_value;
                let result_position = match instruction.modes[2] {
                    0 => instruction.parameters[2] as usize,
                    2 => (instruction.parameters[2] + self.relative_base) as usize,
                    _ => panic!("unexpected mode!"),
                };
                self.memory[result_position] = result;
            }
            OpcodeKind::Input => {
                let input = input_value.unwrap();
                // Write input to the memory in position given by the parameter
                let mode = instruction.modes[0];
                match mode {
                    0 => {
                        let position: usize = instruction.parameters[0] as usize;
                        self.memory[position] = input;
                    }
                    2 => {
                        let position = instruction.parameters[0] + self.relative_base;
                        self.memory[position as usize] = input;
                    }
                    _ => panic!("unexpected mode"),
                }
            }
            OpcodeKind::Output => {
                let mode = instruction.modes[0];
                self.output_signal = match mode {
                    0 => {
                        let position: usize = instruction.parameters[0] as usize;
                        let value = self.memory[position];
                        println!("***********Instruction output: {}", value);
                        Some(value)
                    }
                    1 => {
                        let value = instruction.parameters[0];
                        println!("***********Instruction output: {}", value);
                        Some(value)
                    }
                    2 => {
                        let position = instruction.parameters[0] + self.relative_base;
                        let value = self.memory[position as usize];
                        println!("***********Instruction output: {}", value);
                        Some(value)
                    }
                    _ => panic!("unexpected mode"),
                }
            }
            OpcodeKind::JumpIfTrue => {
                let mode = instruction.modes[0];
                let condition: bool = match mode {
                    0 => {
                        let position: usize = instruction.parameters[0] as usize;
                        self.memory[position] != 0
                    }
                    1 => instruction.parameters[0] != 0,
                    2 => {
                        let position = instruction.parameters[0] + self.relative_base;
                        self.memory[position as usize] != 0
                    }
                    _ => panic!("unexpected mode"),
                };
                if condition {
                    match instruction.modes[1] {
                        0 => {
                            let position: usize = instruction.parameters[1] as usize;
                            self.cursor = self.memory[position] as usize;
                        }
                        1 => {
                            self.cursor = instruction.parameters[1] as usize;
                        }
                        2 => {
                            let position = instruction.parameters[1] + self.relative_base;
                            self.cursor = self.memory[position as usize] as usize;
                        }
                        _ => panic!("unexpected mode"),
                    }
                }
            }
            OpcodeKind::JumpIfFalse => {
                let mode = instruction.modes[0];
                let condition: bool = match mode {
                    0 => {
                        let position: usize = instruction.parameters[0] as usize;
                        self.memory[position] == 0
                    }
                    1 => instruction.parameters[0] == 0,
                    2 => {
                        let position = instruction.parameters[0] + self.relative_base;
                        self.memory[position as usize] == 0
                    }
                    _ => panic!("unexpected mode"),
                };
                if condition {
                    match instruction.modes[1] {
                        0 => {
                            let position: usize = instruction.parameters[1] as usize;
                            self.cursor = self.memory[position] as usize;
                        }
                        1 => {
                            self.cursor = instruction.parameters[1] as usize;
                        }
                        2 => {
                            let position = instruction.parameters[1] + self.relative_base;
                            self.cursor = self.memory[position as usize] as usize;
                        }
                        _ => panic!("unexpected mode"),
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
                    2 => {
                        let noun_position = instruction.parameters[0] + self.relative_base;
                        self.memory[noun_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    2 => {
                        let verb_position = instruction.parameters[1] + self.relative_base;
                        self.memory[verb_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                if noun_value < verb_value {
                    let result_position = match instruction.modes[2] {
                        0 => instruction.parameters[2] as usize,
                        2 => (instruction.parameters[2] + self.relative_base) as usize,
                        _ => panic!("unexpected mode!"),
                    };
                    self.memory[result_position] = 1;
                } else {
                    let result_position = match instruction.modes[2] {
                        0 => instruction.parameters[2] as usize,
                        2 => (instruction.parameters[2] + self.relative_base) as usize,
                        _ => panic!("unexpected mode!"),
                    };
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
                    2 => {
                        let noun_position = instruction.parameters[0] + self.relative_base;
                        self.memory[noun_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                let verb_mode = instruction.modes[1];
                let verb_value = match verb_mode {
                    0 => {
                        let verb_position = instruction.parameters[1] as usize;
                        self.memory[verb_position]
                    }
                    1 => instruction.parameters[1],
                    2 => {
                        let verb_position = instruction.parameters[1] + self.relative_base;
                        self.memory[verb_position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                if noun_value == verb_value {
                    let result_position = match instruction.modes[2] {
                        0 => instruction.parameters[2] as usize,
                        2 => (instruction.parameters[2] + self.relative_base) as usize,
                        _ => panic!("unexpected mode!"),
                    };
                    self.memory[result_position] = 1;
                } else {
                    let result_position = match instruction.modes[2] {
                        0 => instruction.parameters[2] as usize,
                        2 => (instruction.parameters[2] + self.relative_base) as usize,
                        _ => panic!("unexpected mode!"),
                    };
                    self.memory[result_position] = 0;
                }
            }
            OpcodeKind::AdjustRelativeBase => {
                let mode = instruction.modes[0];
                let adjust_value: i64 = match mode {
                    0 => {
                        let position: usize = instruction.parameters[0] as usize;
                        self.memory[position]
                    }
                    1 => instruction.parameters[0],
                    2 => {
                        let position = instruction.parameters[0] + self.relative_base;
                        self.memory[position as usize]
                    }
                    _ => panic!("unexpected mode"),
                };
                self.relative_base += adjust_value;
            }
            OpcodeKind::Exit => {
                std::process::exit(0);
            }
        }
    }
}
