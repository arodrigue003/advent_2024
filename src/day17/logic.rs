use crate::day17::models::{Instruction, Operand, Program, Register};
use itertools::Itertools;

impl Program {
    /// Run a program until it stops and return what was eventually printed
    pub fn run(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        loop {
            // Return the result when we run out of instructions
            if self.instruction_pointer >= self.parsed_instructions.len() {
                return result;
            }

            // Execute the instruction
            match &self.parsed_instructions[self.instruction_pointer] {
                Instruction::Adv(operand) => self.registers.a = self.registers.a >> self.get_value(operand),
                Instruction::Bxl(operand) => self.registers.b ^= self.get_value(operand),
                Instruction::Bst(operand) => self.registers.b = self.get_value(operand) % 8,
                Instruction::Jnz(operand) => {
                    if self.registers.a != 0 {
                        self.instruction_pointer = self.get_value(operand) as usize;
                        // We do not want to increase the instruction pointer in this case
                        continue;
                    }
                }
                Instruction::Bxc => self.registers.b ^= self.registers.c,
                Instruction::Out(operand) => result.push((self.get_value(operand) % 8) as u8),
                Instruction::Bdv(operand) => self.registers.b = self.registers.a >> self.get_value(operand),
                Instruction::Cdv(operand) => self.registers.c = self.registers.a >> self.get_value(operand),
            }

            // Increase the instruction pointer for the next instruction
            self.instruction_pointer += 1;
        }
    }
}

pub fn solve_part_one(program: &Program) -> u32 {
    // Make a program that can be modified
    let mut program = program.clone();

    // run it
    let result = program.run();

    // Print the result since we cannot return strings as result
    println!("{}", result.iter().map(|a| a.to_string()).join(","));

    0
}

impl Program {
    /// Check if the program we have is solvable for the part two with our algorithm
    ///
    /// Return Option<(value, print_a)> if this is possible:
    ///  * value correspond to the shift that is applied to A at each run of the full program
    ///  * print_a is true if the print instruction is associated to a, false otherwise.
    ///
    /// TODO: We should ensure that B and C are always written from A before being used.
    /// TODO: We should ensure that A is not shifted before being used for other instructions
    pub fn is_solvable(&self) -> Option<(u64, bool)> {
        let mut shift = 0;
        let print_a;

        for (pos, instruction) in self.parsed_instructions.iter().enumerate() {
            match instruction {
                Instruction::Adv(operand) => {
                    match operand {
                        Operand::Literal(value) => shift += *value as u64,
                        Operand::Register(_) => {
                            // We cannot allow modifying A by a non-literal value
                            println!("Cannot modify A with a non literal value!");
                            return None;
                        }
                    }
                }
                Instruction::Jnz(operand) => {
                    if pos != self.parsed_instructions.len() - 1 {
                        // We cannot admit a jump instruction that is not the last instruction of
                        // the program.
                        println!("Only one jump at the end is accepted");
                        return None;
                    }
                    if operand != &Operand::Literal(0) {
                        // We do not support not jumping to the beginning
                        println!("Jump operand must be 0");
                        return None;
                    }
                }
                Instruction::Out(_) => {
                    if pos != self.parsed_instructions.len() - 2 {
                        // We cannot admit an out instruction that is not the penultimate
                        // instruction of the program.
                        println!("Only one out operand is accepted");
                        return None;
                    }
                }
                _ => {}
            }
        }

        // The last two instruction must be a print and a jump
        if let Instruction::Out(operand) = &self.parsed_instructions[self.parsed_instructions.len() - 2] {
            print_a = operand == &Operand::Register(Register::A);
        } else {
            println!("Missing last out instruction");
            return None;
        }
        if !matches!(
            self.parsed_instructions[self.parsed_instructions.len() - 1],
            Instruction::Jnz(Operand::Literal(0))
        ) {
            println!("Missing last jump instruction");
            return None;
        }

        Some((shift, print_a))
    }
}

pub fn solve_part_two(program: &Program) -> u64 {
    // Get the max_shift of a for the bruteforce
    let (shift, print_a) = program.is_solvable().unwrap();

    // Create a copy of the program without the last instruction in order to allow for the
    // bruteforce
    let mut bruteforce_program = program.clone();
    bruteforce_program.parsed_instructions.pop();
    bruteforce_program.raw_instructions.pop();
    bruteforce_program.raw_instructions.pop();

    // Set the first value of a
    // We do not need to set the value of B and C since their value are always set from A at each
    // run of the bruteforce program.
    let mut a = 0;

    // Perform the bruteforce
    for target in program.raw_instructions.iter().rev() {
        let new_value = bruteforce_inner_loop(&mut bruteforce_program, a, *target, shift, print_a).unwrap();
        a = (a << shift) + new_value;

        println!("current A:{a:#b}");
    }

    // If we print a, we should add another shift to prepared for the operation
    if print_a {
        a <<= shift;
    }

    // Try it
    let mut test_program = program.clone();
    test_program.registers.a = a;
    let result = test_program.run();
    println!("{}", result.iter().map(|a| a.to_string()).join(","));

    a
}

fn bruteforce_inner_loop(
    bruteforce_program: &mut Program,
    a: u64,
    target: u8,
    shift: u64,
    print_a: bool,
) -> Option<u64> {
    for i in 0..2u64.pow(shift as u32) {
        // reset the bruteforce program for this run
        bruteforce_program.instruction_pointer = 0;

        // Set the bruteforce program value of a for the try
        bruteforce_program.registers.a = (a << shift) + i;

        // If we print a, we should add another shift to prepared for the operation
        if print_a {
            bruteforce_program.registers.a <<= shift;
        }

        // println!("target:{}, i:{}, a:{:#b}", target, i, bruteforce_program.registers.a);

        // Run the bruteforce program
        let result = bruteforce_program.run();

        // check the result
        if result[0] == target {
            // We found a new value of a that is compatible, update a with it
            return Some(i);
        }
    }

    unreachable!()
}
