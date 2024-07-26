use std::{error::Error, fmt::Display, num::ParseIntError};

use crate::instruction::Opcode;

#[derive(Debug)]
pub enum ParseError {
    EmptyStringError,
    InvalidOpcodeError(String),
    MissingRegisterSignError,
    MissingIntegerSignError,
    ParseIntError(ParseIntError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;
        match self {
            EmptyStringError => write!(f, "The string you provided was empty"),
            InvalidOpcodeError(s) => write!(f, "The opcode '{}' does not exist", s),
            MissingRegisterSignError => write!(f, "Registers must start with '$'"),
            MissingIntegerSignError => write!(f, "Integers must start with '#'"),
            ParseIntError(e) => write!(f, "There was an error parsing the input: {}", e),
        }
    }
}

impl Error for ParseError {}

pub enum Token {
    Op(Opcode),
    Register(u8),
    IntegerOperand(i32),
}

impl Token {
    pub fn parse_line(string: &str) -> Result<Vec<Self>, ParseError> {
        todo!()
    }

    fn parse_opcode(string: &str) -> Result<Self, ParseError> {
        let opcode = Opcode::from(string);

        if opcode == Opcode::IGL {
            return Err(ParseError::InvalidOpcodeError(string.to_owned()));
        }

        Ok(Token::Op(opcode))
    }

    fn parse_register(string: &str) -> Result<Self, ParseError> {
        string
            .strip_prefix('$')
            .ok_or(ParseError::MissingRegisterSignError)?
            .parse::<u8>()
            .map(Token::Register)
            .map_err(|e| ParseError::ParseIntError(e))
    }

    fn parse_integer_operand(string: &str) -> Result<Self, ParseError> {
        string
            .strip_prefix('#')
            .ok_or(ParseError::MissingIntegerSignError)?
            .parse::<i32>()
            .map(Token::IntegerOperand)
            .map_err(|e| ParseError::ParseIntError(e))
    }
}
