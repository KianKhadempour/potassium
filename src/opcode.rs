use std::fmt::Display;

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
}

#[derive(Debug)]
pub struct InvalidOpcodeError<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> InvalidOpcodeError<T> {
    fn new(value: T) -> Self {
        InvalidOpcodeError { value }
    }
}

impl<T: std::fmt::Display> Display for InvalidOpcodeError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not defined as an opcode", self.value)
    }
}

impl TryFrom<u8> for Opcode {
    type Error = InvalidOpcodeError<u8>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::HLT),
            1 => Ok(Opcode::LOAD),
            2 => Ok(Opcode::ADD),
            3 => Ok(Opcode::SUB),
            4 => Ok(Opcode::MUL),
            5 => Ok(Opcode::DIV),
            6 => Ok(Opcode::JMP),
            7 => Ok(Opcode::JMPF),
            8 => Ok(Opcode::JMPB),
            9 => Ok(Opcode::EQ),
            10 => Ok(Opcode::NEQ),
            11 => Ok(Opcode::GT),
            12 => Ok(Opcode::LT),
            13 => Ok(Opcode::GTQ),
            14 => Ok(Opcode::LTQ),
            15 => Ok(Opcode::JEQ),
            16 => Ok(Opcode::JNEQ),
            n => Err(InvalidOpcodeError::new(n)),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::HLT => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MUL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTQ => 13,
            Opcode::LTQ => 14,
            Opcode::JEQ => 15,
            Opcode::JNEQ => 16,
        }
    }
}

impl TryFrom<&str> for Opcode {
    type Error = InvalidOpcodeError<String>;

    fn try_from(value: &str) -> Result<Self, InvalidOpcodeError<String>> {
        match value.to_ascii_lowercase().as_str() {
            "hlt" => Ok(Opcode::HLT),
            "load" => Ok(Opcode::LOAD),
            "add" => Ok(Opcode::ADD),
            "sub" => Ok(Opcode::SUB),
            "mul" => Ok(Opcode::MUL),
            "div" => Ok(Opcode::DIV),
            "jmp" => Ok(Opcode::JMP),
            "jmpf" => Ok(Opcode::JMPF),
            "jmpb" => Ok(Opcode::JMPB),
            "eq" => Ok(Opcode::EQ),
            "neq" => Ok(Opcode::NEQ),
            "gt" => Ok(Opcode::GT),
            "lt" => Ok(Opcode::LT),
            "gtq" => Ok(Opcode::GTQ),
            "ltq" => Ok(Opcode::LTQ),
            "jeq" => Ok(Opcode::JEQ),
            "jneq" => Ok(Opcode::JNEQ),
            _ => Err(InvalidOpcodeError::new(value.to_owned())),
        }
    }
}
