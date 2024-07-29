use std::{error::Error, fmt::Display, num::ParseIntError};

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidOpcodeError(String),
    MissingRegisterSignError,
    MissingIntegerSignError,
    ParseIntError(ParseIntError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError as PE;
        match self {
            PE::InvalidOpcodeError(s) => write!(f, "The opcode '{}' does not exist", s),
            PE::MissingRegisterSignError => write!(f, "Registers must start with '$'"),
            PE::MissingIntegerSignError => write!(f, "Integers must start with '#'"),
            PE::ParseIntError(e) => write!(f, "There was an error parsing the input: {e}"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq)]
pub enum Token {
    Op(Opcode),
    Register(u8),
    IntegerOperand(i32),
}

impl TryFrom<&str> for Token {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('$') {
            value
                .strip_prefix('$')
                .ok_or(ParseError::MissingRegisterSignError)?
                .parse::<u8>()
                .map(Token::Register)
                .map_err(|e| ParseError::ParseIntError(e))
        } else if value.starts_with('#') {
            value
                .strip_prefix('#')
                .ok_or(ParseError::MissingIntegerSignError)?
                .parse::<i32>()
                .map(Token::IntegerOperand)
                .map_err(|e| ParseError::ParseIntError(e))
        } else {
            Opcode::try_from(value)
                .map(Token::Op)
                .map_err(|_| ParseError::InvalidOpcodeError(value.to_owned()))
        }
    }
}
