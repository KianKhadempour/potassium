use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        }
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
        if self.pc > self.program.len() {
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
        test_vm.program = vec![0, 0, 0, 0];

        assert_eq!(test_vm.decode_opcode(), Opcode::HLT);
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200, 0, 0, 0];

        assert_eq!(test_vm.decode_opcode(), Opcode::IGL);
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
}
