use crate::vm::VM;
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

    pub fn run(&mut self) -> ! {
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
                ".quit" => {
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
                ".registers" => {
                    println!("pc: {}", self.vm.pc);
                    println!("rem: {}", self.vm.remainder);
                    println!("bool: {}", self.vm.equal_flag);
                    for (n, value) in self.vm.registers.into_iter().enumerate() {
                        println!("reg{}: {}", n, value);
                    }
                }
                _ => {
                    if let Ok(instruction) = parse_hex(buffer) {
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
