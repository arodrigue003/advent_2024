use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Register {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operand {
    Literal(u8),
    Register(Register),
}

impl Operand {
    pub fn new_literal(value: u8) -> Self {
        Self::Literal(value)
    }

    pub fn new_combo(value: u8) -> Self {
        match value {
            0 => Self::Literal(0),
            1 => Self::Literal(1),
            2 => Self::Literal(2),
            3 => Self::Literal(3),
            4 => Self::Register(Register::A),
            5 => Self::Register(Register::B),
            6 => Self::Register(Register::C),
            7 => unreachable!("Combo operand cannot be 7"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Instruction {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc,
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl Instruction {
    pub fn new(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Self::Adv(Operand::new_combo(operand)),
            1 => Self::Bxl(Operand::new_literal(operand)),
            2 => Self::Bst(Operand::new_combo(operand)),
            3 => Self::Jnz(Operand::new_literal(operand)),
            4 => Self::Bxc,
            5 => Self::Out(Operand::new_combo(operand)),
            6 => Self::Bdv(Operand::new_combo(operand)),
            7 => Self::Cdv(Operand::new_combo(operand)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represent register.
///
/// **Notes**
/// Registers types must be unsigned integers because of following bits operating (xor and shift)
pub struct Registers {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl Registers {
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }

    pub fn get_value(&self, register: &Register) -> u64 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Program {
    pub registers: Registers,
    pub instruction_pointer: usize,
    pub raw_instructions: Vec<u8>,
    pub parsed_instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(a: u64, b: u64, c: u64, raw_instructions: Vec<u8>) -> Self {
        let parsed_instructions = raw_instructions
            .iter()
            .tuples()
            .map(|(opcode, operand)| Instruction::new(*opcode, *operand))
            .collect();

        Self {
            registers: Registers::new(a, b, c),
            instruction_pointer: 0,
            raw_instructions,
            parsed_instructions,
        }
    }

    pub fn get_value(&self, operand: &Operand) -> u64 {
        match operand {
            Operand::Literal(literal) => *literal as u64,
            Operand::Register(register) => self.registers.get_value(register)
        }
    }
}
