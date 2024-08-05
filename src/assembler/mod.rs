use lexer::lex;
use parser::parse;
use token::ParseError;

mod instruction;
mod lexer;
mod parser;
mod token;

pub fn assemble(input: &str) -> Result<Vec<[u8; 4]>, ParseError> {
    let tokens = lex(input)?;
    let instructions = parse(tokens)?;

    let mut output = vec![];

    for instruction in instructions {
        match instruction {
            instruction::Instruction::HLT => output.push([0, 0, 0, 0]),
            instruction::Instruction::LOAD(reg, int) => {
                output.push([1, reg, (int >> 8) as u8, int as u8]);
            }
            instruction::Instruction::ADD(reg1, reg2, reg3) => {
                output.push([2, reg1, reg2, reg3]);
            }
            instruction::Instruction::SUB(reg1, reg2, reg3) => {
                output.push([3, reg1, reg2, reg3]);
            }
            instruction::Instruction::MUL(reg1, reg2, reg3) => {
                output.push([4, reg1, reg2, reg3]);
            }
            instruction::Instruction::DIV(reg1, reg2, reg3) => {
                output.push([5, reg1, reg2, reg3]);
            }
            instruction::Instruction::JMP(reg) => {
                output.push([6, reg, 0, 0]);
            }
            instruction::Instruction::JMPF(reg) => {
                output.push([7, reg, 0, 0]);
            }
            instruction::Instruction::JMPB(reg) => {
                output.push([8, reg, 0, 0]);
            }
            instruction::Instruction::EQ(reg1, reg2) => {
                output.push([9, reg1, reg2, 0]);
            }
            instruction::Instruction::NEQ(reg1, reg2) => {
                output.push([10, reg1, reg2, 0]);
            }
            instruction::Instruction::GT(reg1, reg2) => {
                output.push([11, reg1, reg2, 0]);
            }
            instruction::Instruction::LT(reg1, reg2) => {
                output.push([12, reg1, reg2, 0]);
            }
            instruction::Instruction::GTQ(reg1, reg2) => {
                output.push([13, reg1, reg2, 0]);
            }
            instruction::Instruction::LTQ(reg1, reg2) => {
                output.push([14, reg1, reg2, 0]);
            }
            instruction::Instruction::JEQ(reg) => {
                output.push([15, reg, 0, 0]);
            }
            instruction::Instruction::JNEQ(reg) => {
                output.push([16, reg, 0, 0]);
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_hlt() {
        let expected_output = vec![[0, 0, 0, 0]];

        assert_eq!(assemble("HLT"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_load() {
        let expected_output = vec![[1, 0, 1, 244]];

        assert_eq!(assemble("LOAD $0 #500"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_add() {
        let expected_output = vec![[2, 0, 1, 2]];

        assert_eq!(assemble("ADD $0 $1 $2"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_sub() {
        let expected_output = vec![[3, 0, 1, 2]];

        assert_eq!(assemble("SUB $0 $1 $2"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_mul() {
        let expected_output = vec![[4, 0, 1, 2]];

        assert_eq!(assemble("MUL $0 $1 $2"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_div() {
        let expected_output = vec![[5, 0, 1, 2]];

        assert_eq!(assemble("DIV $0 $1 $2"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_jmp() {
        let expected_output = vec![[6, 0, 0, 0]];

        assert_eq!(assemble("JMP $0"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_jmpf() {
        let expected_output = vec![[7, 0, 0, 0]];

        assert_eq!(assemble("JMPF $0"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_jmpb() {
        let expected_output = vec![[8, 0, 0, 0]];

        assert_eq!(assemble("JMPB $0"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_eq() {
        let expected_output = vec![[9, 0, 1, 0]];

        assert_eq!(assemble("EQ $0 $1"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_neq() {
        let expected_output = vec![[10, 0, 1, 0]];

        assert_eq!(assemble("NEQ $0 $1"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_gt() {
        let expected_output = vec![[11, 0, 1, 0]];

        assert_eq!(assemble("GT $0 $1"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_lt() {
        let expected_output = vec![[12, 0, 1, 0]];

        assert_eq!(assemble("LT $0 $1"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_gtq() {
        let expected_output = vec![[13, 0, 1, 0]];

        assert_eq!(assemble("GTQ $0 $1"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_ltq() {
        let expected_output = vec![[14, 0, 1, 0]];

        assert_eq!(assemble("LTQ $0 $1"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_jeq() {
        let expected_output = vec![[15, 0, 0, 0]];

        assert_eq!(assemble("JEQ $0"), Ok(expected_output));
    }

    #[test]
    fn test_assemble_jneq() {
        let expected_output = vec![[16, 0, 0, 0]];

        assert_eq!(assemble("JNEQ $0"), Ok(expected_output));
    }
}
