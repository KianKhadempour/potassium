use crate::assembler::instruction::Instruction;

use super::token::{ParseError, Token};

pub fn parse(input: Vec<Token>) -> Result<Vec<Instruction>, ParseError> {
    use crate::assembler::token::Token as T;
    use crate::opcode::Opcode as O;
    let mut pos = 0;
    let len = input.len();

    let mut output = vec![];

    while pos < len {
        match &input[pos] {
            Token::Op(opcode) => match (
                opcode,
                input.get(pos + 1),
                input.get(pos + 2),
                input.get(pos + 3),
            ) {
                (O::HLT, _, _, _) => {
                    output.push(Instruction::HLT);
                    pos += 1;
                }
                (O::LOAD, Some(T::Register(reg)), Some(T::IntegerOperand(int)), _) => {
                    output.push(Instruction::LOAD(*reg, *int));
                    pos += 3;
                }
                (
                    O::ADD,
                    Some(T::Register(reg1)),
                    Some(T::Register(reg2)),
                    Some(T::Register(reg3)),
                ) => {
                    output.push(Instruction::ADD(*reg1, *reg2, *reg3));
                    pos += 4;
                }
                (
                    O::SUB,
                    Some(T::Register(reg1)),
                    Some(T::Register(reg2)),
                    Some(T::Register(reg3)),
                ) => {
                    output.push(Instruction::SUB(*reg1, *reg2, *reg3));
                    pos += 4;
                }
                (
                    O::MUL,
                    Some(T::Register(reg1)),
                    Some(T::Register(reg2)),
                    Some(T::Register(reg3)),
                ) => {
                    output.push(Instruction::MUL(*reg1, *reg2, *reg3));
                    pos += 4;
                }
                (
                    O::DIV,
                    Some(T::Register(reg1)),
                    Some(T::Register(reg2)),
                    Some(T::Register(reg3)),
                ) => {
                    output.push(Instruction::DIV(*reg1, *reg2, *reg3));
                    pos += 4;
                }
                (O::JMP, Some(T::Register(reg)), _, _) => {
                    output.push(Instruction::JMP(*reg));
                    pos += 2;
                }
                (O::JMPF, Some(T::Register(reg)), _, _) => {
                    output.push(Instruction::JMPF(*reg));
                    pos += 2;
                }
                (O::JMPB, Some(T::Register(reg)), _, _) => {
                    output.push(Instruction::JMPB(*reg));
                    pos += 2;
                }
                (O::EQ, Some(T::Register(reg1)), Some(T::Register(reg2)), _) => {
                    output.push(Instruction::EQ(*reg1, *reg2));
                    pos += 3;
                }
                (O::NEQ, Some(T::Register(reg1)), Some(T::Register(reg2)), _) => {
                    output.push(Instruction::NEQ(*reg1, *reg2));
                    pos += 3;
                }
                (O::GT, Some(T::Register(reg1)), Some(T::Register(reg2)), _) => {
                    output.push(Instruction::GT(*reg1, *reg2));
                    pos += 3;
                }
                (O::LT, Some(T::Register(reg1)), Some(T::Register(reg2)), _) => {
                    output.push(Instruction::LT(*reg1, *reg2));
                    pos += 3;
                }
                (O::GTQ, Some(T::Register(reg1)), Some(T::Register(reg2)), _) => {
                    output.push(Instruction::GTQ(*reg1, *reg2));
                    pos += 3;
                }
                (O::LTQ, Some(T::Register(reg1)), Some(T::Register(reg2)), _) => {
                    output.push(Instruction::LTQ(*reg1, *reg2));
                    pos += 3;
                }
                (O::JEQ, Some(T::Register(reg)), _, _) => {
                    output.push(Instruction::JEQ(*reg));
                    pos += 2;
                }
                (O::JNEQ, Some(T::Register(reg)), _, _) => {
                    output.push(Instruction::JNEQ(*reg));
                    pos += 2;
                }
                _ => {
                    return Err(ParseError::InvalidOpcodeError(
                        "sequence of opcodes could not be parsed to instruction".to_owned(),
                    ))
                }
            },
            _ => {
                return Err(ParseError::InvalidOpcodeError(
                    "instruction must start with an opcode".to_owned(),
                ))
            }
        }
    }

    return Ok(output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assembler::{instruction::Instruction, token::Token, ParseError},
        opcode::Opcode,
    };

    #[test]
    fn test_parse() {
        let input = vec![
            Token::Op(Opcode::LOAD),
            Token::Register(0),
            Token::IntegerOperand(500),
        ];
        let expected_output = vec![Instruction::LOAD(0, 500)];

        assert_eq!(parse(input), Ok(expected_output))
    }
    #[test]
    fn test_parse_failure_1() {
        let input = vec![Token::Register(0), Token::IntegerOperand(500)];

        assert_eq!(
            parse(input),
            Err(ParseError::InvalidOpcodeError(
                "instruction must start with an opcode".to_owned(),
            ))
        )
    }
    #[test]
    fn test_parse_failure_2() {
        let input = vec![Token::Op(Opcode::LOAD), Token::IntegerOperand(500)];

        assert_eq!(
            parse(input),
            Err(ParseError::InvalidOpcodeError(
                "sequence of opcodes could not be parsed to instruction".to_owned(),
            ))
        )
    }
}
