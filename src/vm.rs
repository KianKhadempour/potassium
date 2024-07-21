use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn set_program(&mut self, program: Vec<[u8; 4]>) {
        self.program = program.into_iter().flatten().collect()
    }

    /// Executes the VM's entire program
    pub fn run(&mut self) -> i8 {
        loop {
            if let Some(code) = self.execute_instruction() {
                return code;
            }
        }
    }

    /// Executes one instruction for finer control over the VM
    pub fn run_once(&mut self) -> i8 {
        if let Some(code) = self.execute_instruction() {
            return code;
        } else {
            return 0;
        }
    }

    fn execute_instruction(&mut self) -> Option<i8> {
        if self.pc >= self.program.len() {
            eprintln!(
                "Program counter has exceeded program length! Did you forget to include an HLT?"
            );
            return Some(-1);
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered.");
                return Some(0);
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits();

                self.registers[register] = number as i32;
                return None;
            }
            Opcode::ADD => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                let result_reg = self.next_8_bits() as usize;

                self.registers[result_reg] = self.registers[reg1] + self.registers[reg2];
                return None;
            }
            Opcode::SUB => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                let result_reg = self.next_8_bits() as usize;

                self.registers[result_reg] = self.registers[reg1] - self.registers[reg2];
                return None;
            }
            Opcode::MUL => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                let result_reg = self.next_8_bits() as usize;

                self.registers[result_reg] = self.registers[reg1] * self.registers[reg2];
                return None;
            }
            Opcode::DIV => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                let result_reg = self.next_8_bits() as usize;

                let divmod = (
                    self.registers[reg1] / self.registers[reg2],
                    self.registers[reg1] % self.registers[reg2],
                );

                self.registers[result_reg] = divmod.0;
                self.remainder = divmod.1 as u32;

                return None;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;

                return None;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;

                return None;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;

                return None;
            }
            Opcode::EQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = val1 == val2;

                self.pc += 1;

                return None;
            }
            Opcode::NEQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = val1 != val2;

                self.pc += 1;

                return None;
            }
            Opcode::GT => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = val1 > val2;

                self.pc += 1;

                return None;
            }
            Opcode::LT => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = val1 < val2;

                self.pc += 1;

                return None;
            }
            Opcode::GTQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = val1 >= val2;

                self.pc += 1;

                return None;
            }
            Opcode::LTQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];

                self.equal_flag = val1 <= val2;

                self.pc += 1;

                return None;
            }
            Opcode::JEQ => {
                let value = self.registers[self.next_8_bits() as usize];

                if self.equal_flag {
                    self.pc = value as usize;
                } else {
                    self.pc += 2;
                }

                return None;
            }
            Opcode::JNEQ => {
                let value = self.registers[self.next_8_bits() as usize];

                if !self.equal_flag {
                    self.pc = value as usize;
                } else {
                    self.pc += 2;
                }

                return None;
            }
            _ => {
                eprintln!("Unrecognized opcode found. Terminating!");
                return Some(-1);
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let bits = self.program[self.pc];
        self.pc += 1;
        return bits;
    }

    fn next_16_bits(&mut self) -> u16 {
        let bits = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return bits;
    }

    fn next_24_bits(&mut self) -> u32 {
        let bits = ((self.program[self.pc] as u32) << 16)
            | ((self.program[self.pc + 1] as u32) << 8)
            | self.program[self.pc + 2] as u32;
        self.pc += 3;
        return bits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[0, 0, 0, 0]]);

        assert_eq!(test_vm.decode_opcode(), Opcode::HLT);
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[200, 0, 0, 0]]);

        assert_eq!(test_vm.decode_opcode(), Opcode::IGL);
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[1, 0, 1, 244], [0, 0, 0, 0]]);

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 1, 244],
            [1, 1, 0, 250],
            [2, 0, 1, 3],
            [0, 0, 0, 0],
        ]);

        test_vm.run();
        assert_eq!(test_vm.registers[3], 750);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 1, 244],
            [1, 1, 0, 250],
            [3, 0, 1, 3],
            [0, 0, 0, 0],
        ]);

        test_vm.run();
        assert_eq!(test_vm.registers[3], 250);
    }
    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[1, 0, 0, 8], [1, 1, 0, 6], [4, 0, 1, 3], [0, 0, 0, 0]]);

        test_vm.run();
        assert_eq!(test_vm.registers[3], 48);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[1, 0, 0, 8], [1, 1, 0, 6], [5, 0, 1, 3], [0, 0, 0, 0]]);

        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);
        assert_eq!(test_vm.remainder, 2);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[1, 0, 0, 0], [6, 0, 0, 0]]);

        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[1, 0, 0, 8], [7, 0, 0, 0]]);

        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 14);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![[1, 0, 0, 6], [8, 0, 0, 0]]);

        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 6],
            [9, 0, 1, 0],
            [1, 1, 0, 7],
            [9, 0, 1, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 6],
            [10, 0, 1, 0],
            [1, 1, 0, 7],
            [10, 0, 1, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 7],
            [11, 0, 1, 0],
            [1, 1, 0, 5],
            [11, 0, 1, 0],
            [1, 1, 0, 6],
            [11, 0, 1, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 7],
            [12, 0, 1, 0],
            [1, 1, 0, 5],
            [12, 0, 1, 0],
            [1, 1, 0, 6],
            [12, 0, 1, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gtq() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 7],
            [13, 0, 1, 0],
            [1, 1, 0, 5],
            [13, 0, 1, 0],
            [1, 1, 0, 6],
            [13, 0, 1, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_ltq() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 7],
            [14, 0, 1, 0],
            [1, 1, 0, 5],
            [14, 0, 1, 0],
            [1, 1, 0, 6],
            [14, 0, 1, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 7],
            [12, 0, 1, 0],
            [1, 2, 0, 24],
            [15, 2, 0, 0],
            [0, 0, 0, 0],
            [1, 1, 0, 5],
            [12, 0, 1, 0],
            [1, 2, 0, 4],
            [15, 2, 0, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 24);
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 40);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut test_vm = VM::new();
        test_vm.set_program(vec![
            [1, 0, 0, 6],
            [1, 1, 0, 7],
            [12, 0, 1, 0],
            [1, 2, 0, 24],
            [16, 2, 0, 0],
            [1, 1, 0, 5],
            [12, 0, 1, 0],
            [1, 2, 0, 4],
            [16, 2, 0, 0],
        ]);

        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 20);
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }
}
