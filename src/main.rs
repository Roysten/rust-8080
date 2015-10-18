mod macros;
mod instruction;
mod register;
mod cpu;

use cpu::CPU;

fn main() {
    let program = include_bytes!("../ld.bin");
    let mut cpu = CPU::new_with_mem(program.to_vec());
    cpu.next_instruction();
}
