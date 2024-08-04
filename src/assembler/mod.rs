use lexer::Lexer;
use token::ParseError;

mod instruction;
mod lexer;
mod token;

struct Assembler {
    lexer: Lexer,
}

impl Assembler {
    pub fn new(input: String) -> Self {
        Assembler {
            lexer: Lexer::new(input),
        }
    }

    pub fn assemble(&mut self) -> Result<Vec<[u8; 4]>, ParseError> {
        let tokens = self.lexer.lex()?;

        todo!()
    }
}
