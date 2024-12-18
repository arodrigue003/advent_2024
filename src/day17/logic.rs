use crate::day17::models::{Instruction, Program};
use itertools::Itertools;

impl Program {
    /// Run a program until it stops and return what was eventually printed
    pub fn run(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        loop {
            // Return the result when we run out of instructions
            if self.instruction_pointer >= self.parsed_instructions.len() {
                return result
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
                        continue
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

pub fn solve_part_two(program: &Program) -> u32 {
    0
}
