#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
    IGL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GT,
            12 => return Opcode::LT,
            13 => return Opcode::GTQ,
            14 => return Opcode::LTQ,
            15 => return Opcode::JEQ,
            16 => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "hlt" => return Opcode::HLT,
            "load" => return Opcode::LOAD,
            "add" => return Opcode::ADD,
            "sub" => return Opcode::SUB,
            "mul" => return Opcode::MUL,
            "div" => return Opcode::DIV,
            "jmp" => return Opcode::JMP,
            "jmpf" => return Opcode::JMPF,
            "jmpb" => return Opcode::JMPB,
            "eq" => return Opcode::EQ,
            "neq" => return Opcode::NEQ,
            "gt" => return Opcode::GT,
            "lt" => return Opcode::LT,
            "gtq" => return Opcode::GTQ,
            "ltq" => return Opcode::LTQ,
            "jeq" => return Opcode::JEQ,
            "jneq" => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

pub struct Instruction {
    pub opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let test_instruction = Instruction::new(Opcode::HLT);
        assert_eq!(test_instruction.opcode, Opcode::HLT);
    }
}
