use super::token::{ParseError, Token};

pub fn lex(input: &str) -> Result<Vec<Token>, ParseError> {
    input
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| Token::try_from(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::opcode::Opcode;

    use super::*;

    #[test]
    fn test_lex_opcode() {
        let expected_output = vec![Token::Op(Opcode::HLT)];

        assert_eq!(lex("HLT"), Ok(expected_output));
    }

    #[test]
    fn test_lex_register() {
        let expected_output = vec![Token::Register(0)];

        assert_eq!(lex("$0"), Ok(expected_output));
    }

    #[test]
    fn test_lex_integer_operand() {
        let expected_output = vec![Token::IntegerOperand(500)];

        assert_eq!(lex("#500"), Ok(expected_output));
    }

    #[test]
    fn test_lex_instruction() {
        let expected_output = vec![
            Token::Op(Opcode::LOAD),
            Token::Register(0),
            Token::IntegerOperand(500),
        ];

        assert_eq!(lex("LOAD $0 #500"), Ok(expected_output));
    }
}
