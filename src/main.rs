mod register;
mod cpu;

use cpu::CPU;

fn main() {
    let program = include_bytes!("../ld.bin");
    let cpu = CPU::new_with_mem(program.to_vec());
}
