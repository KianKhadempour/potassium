use repl::REPL;

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = REPL::new();
    repl.vm.set_program(vec![
        [1, 0, 0, 6],  // Set reg0 to 6
        [1, 1, 0, 7],  // Set reg1 to 7
        [12, 0, 1, 0], // Set equal_flag to reg0 < reg1
        [1, 2, 0, 24], // Set reg2 to 24
        [16, 2, 0, 0], // Jump to reg2 if !equal_flag
        [1, 1, 0, 5],  // Set reg1 to 5
        [12, 0, 1, 0], // Set equal_flag to reg0 < reg1
        [1, 2, 0, 4],  // Set reg2 to 4
        [16, 2, 0, 0], // Jump to reg2 if !equal_flag
    ]);
    repl.run();
}
