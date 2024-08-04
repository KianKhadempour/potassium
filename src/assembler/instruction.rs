use crate::opcode::Opcode;

pub enum Instruction {
    HLT,
    LOAD(u8, i32),
    ADD(u8, u8, u8),
    SUB(u8, u8, u8),
    MUL(u8, u8, u8),
    DIV(u8, u8, u8),
    JMP(u8),
    JMPF(u8),
    JMPB(u8),
    EQ(u8, u8),
    NEQ(u8, u8),
    GT(u8, u8),
    LT(u8, u8),
    GTQ(u8, u8),
    LTQ(u8, u8),
    JEQ(u8),
    JNEQ(u8),
}

impl From<Instruction> for Opcode {
    fn from(value: Instruction) -> Self {
        use Instruction as I;
        match value {
            I::HLT => Opcode::HLT,
            I::LOAD(_, _) => Opcode::LOAD,
            I::ADD(_, _, _) => Opcode::ADD,
            I::SUB(_, _, _) => Opcode::SUB,
            I::MUL(_, _, _) => Opcode::MUL,
            I::DIV(_, _, _) => Opcode::DIV,
            I::JMP(_) => Opcode::JMP,
            I::JMPF(_) => Opcode::JMPF,
            I::JMPB(_) => Opcode::JMPB,
            I::EQ(_, _) => Opcode::EQ,
            I::NEQ(_, _) => Opcode::NEQ,
            I::GT(_, _) => Opcode::GT,
            I::LT(_, _) => Opcode::LT,
            I::GTQ(_, _) => Opcode::GTQ,
            I::LTQ(_, _) => Opcode::LTQ,
            I::JEQ(_) => Opcode::JEQ,
            I::JNEQ(_) => Opcode::JNEQ,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_instruction() {
        let test_instruction = Instruction::HLT;
        assert_eq!(Opcode::from(test_instruction), Opcode::HLT);
    }
}
