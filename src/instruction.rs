#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
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
            _ => return Opcode::IGL,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
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
