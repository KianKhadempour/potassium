use crate::{assembler::assemble, vm::VM};
use std::{
    io::{self, Write},
    num::ParseIntError,
};

pub struct REPL {
    command_buffer: Vec<String>,
    pub vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    pub fn start(&mut self) -> ! {
        println!("Welcome to the potassium REPL");

        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout!");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from stdin!");

            let buffer = buffer.trim();

            match buffer {
                ".quit" | ".exit" => {
                    println!("Exiting the potassium REPL.");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    for instruction in self.vm.program.chunks(4) {
                        println!("{:02X?}", instruction);
                    }
                }
                ".run" => {
                    self.vm.run();
                }
                ".registers" => {
                    println!("pc: {}", self.vm.pc);
                    println!("rem: {}", self.vm.remainder);
                    println!("bool: {}", self.vm.equal_flag);
                    for (n, value) in self.vm.registers.into_iter().enumerate() {
                        println!("reg{}: {}", n, value);
                    }
                }
                _ => {
                    if let Some(filename) = buffer.strip_prefix(".load ") {
                        if let Ok(file) = std::fs::read_to_string(filename) {
                            if let Ok(instructions) = assemble(&file) {
                                self.vm.set_program(instructions);
                                self.vm.pc = 0;
                            } else {
                                println!("Failed to assemble program");
                            }
                        } else {
                            println!("Failed to read file");
                        }
                    } else if let Some(reg) = buffer.strip_prefix(".reg").map(|s| s.trim()) {
                        if let Ok(reg) = reg.parse::<usize>() {
                            println!("reg{}: {}", reg, self.vm.registers[reg]);
                        } else {
                            println!("Invalid register number");
                        }
                    } else if let Ok(instruction) = assemble(buffer) {
                        self.vm
                            .program
                            .append(&mut instruction.into_iter().flatten().collect());
                        self.vm.run_once();
                    } else if let Ok(instruction) = parse_hex(buffer) {
                        self.vm.program.append(&mut instruction.clone());
                        self.vm.run_once();
                    } else {
                        println!("Invalid input");
                    }
                }
            }

            self.command_buffer.push(buffer.to_owned());
        }
    }
}

fn parse_hex(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input
        .split(" ")
        .map(|b| u8::from_str_radix(b, 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        let result = parse_hex("01 00 00 06");

        assert_eq!(result, Ok(vec![1, 0, 0, 6]));

        let result = parse_hex("invalid");

        assert!(result.is_err());
    }
}
