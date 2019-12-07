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

    // Copy the instruction set to each Amplifier

    // The phase values are [0,1,2,3,4], but in an unknown order.
    let mut max_output = 0;
    let mut max_config = (0, 0, 0, 0, 0);
    for a in 0..5 {
        for b in 0..5 {
            if a == b {
                continue;
            }
            for c in 0..5 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..5 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..5 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let input_value = 0;
                        let mut amplifier_a: Amplifier = Amplifier::new(instruction_set.clone());
                        let mut amplifier_b: Amplifier = Amplifier::new(instruction_set.clone());
                        let mut amplifier_c: Amplifier = Amplifier::new(instruction_set.clone());
                        let mut amplifier_d: Amplifier = Amplifier::new(instruction_set.clone());
                        let mut amplifier_e: Amplifier = Amplifier::new(instruction_set.clone());
                        amplifier_a.run_tape(input_value, a);
                        amplifier_b.run_tape(amplifier_a.output_signal.unwrap(), b);
                        amplifier_c.run_tape(amplifier_b.output_signal.unwrap(), c);
                        amplifier_d.run_tape(amplifier_c.output_signal.unwrap(), d);
                        amplifier_e.run_tape(amplifier_d.output_signal.unwrap(), e);
                        let output_signal = amplifier_e.output_signal.unwrap();
                        if output_signal > max_output {
                            max_output = output_signal;
                            max_config = (a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }

    println!("Largest possible output: {}", max_output);
    println!("Config: {:?}", max_config);

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
    output_signal: Option<i32>,
    cursor: usize,
}

impl Amplifier {
    fn new(memory: Vec<i32>) -> Amplifier {
        let output_signal = None;
        let cursor: usize = 0;
        Amplifier {
            memory: memory,
            output_signal: output_signal,
            cursor,
        }
    }

    fn run_tape(&mut self, input_value: i32, phase_value: i32) -> () {
        let mut phase_value_processed = false;
        loop {
            // Read the memory at the cursor position, and parse the opcode.
            let instruction = Instruction::new(&self.memory, self.cursor);
            let prev_cursor = self.cursor;
            // We need a way to get the two inputs in when required, and extract the output.
            match instruction.opcode {
                OpcodeKind::Input => {
                    if !phase_value_processed {
                        self.process_instruction(&instruction, Some(phase_value));
                        phase_value_processed = true;
                    } else {
                        self.process_instruction(&instruction, Some(input_value));
                    }
                }
                OpcodeKind::Exit => {
                    break;
                }
                _ => self.process_instruction(&instruction, None),
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
                    self.output_signal = Some(value);
                // println!("***********Instruction output: {}", value);
                } else if mode == 1 {
                    let value = instruction.parameters[0];
                    self.output_signal = Some(value);
                    // println!("***********Instruction output: {}", value);
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
