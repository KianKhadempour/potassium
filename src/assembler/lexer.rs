use super::token::{ParseError, Token};

pub struct Lexer {
    input: String,
    start: usize,
    end: usize,
    len: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            len: input.len(),
            input,
            start: 0,
            end: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            if self.end >= self.len {
                tokens.push(Token::try_from(
                    self.input.get(self.start..self.end).unwrap(),
                )?);

                break;
            }

            if self.peek() == ' ' {
                tokens.push(Token::try_from(
                    self.input.get(self.start..self.end).unwrap(),
                )?);
                self.start = self.end + 1;
            }

            self.end += 1;
        }

        Ok(tokens)
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.end).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Opcode;

    use super::*;

    #[test]
    fn test_create_lexer() {
        let lexer = Lexer::new(String::from("LOAD $0 #500"));

        assert_eq!(lexer.input, "LOAD $0 #500");
        assert_eq!(lexer.len, 12);
    }

    #[test]
    fn test_lex_opcode() {
        let mut lexer = Lexer::new(String::from("LOAD"));

        let expected_output = vec![Token::Op(Opcode::LOAD)];

        assert_eq!(lexer.lex(), Ok(expected_output))
    }

    #[test]
    fn test_lex_register() {
        let mut lexer = Lexer::new(String::from("$0"));

        let expected_output = vec![Token::Register(0)];

        assert_eq!(lexer.lex(), Ok(expected_output))
    }

    #[test]
    fn test_lex_integer_operand() {
        let mut lexer = Lexer::new(String::from("#500"));

        let expected_output = vec![Token::IntegerOperand(500)];

        assert_eq!(lexer.lex(), Ok(expected_output))
    }

    #[test]
    fn test_lex_instruction() {
        let mut lexer = Lexer::new(String::from("LOAD $0 #500"));

        let expected_output = vec![
            Token::Op(Opcode::LOAD),
            Token::Register(0),
            Token::IntegerOperand(500),
        ];

        assert_eq!(lexer.lex(), Ok(expected_output))
    }
}
