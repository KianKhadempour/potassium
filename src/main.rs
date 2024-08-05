pub mod assembler;
pub mod opcode;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.start();
}
