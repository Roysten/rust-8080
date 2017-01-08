mod cpu;
mod register;
mod instruction;

use cpu::CPU;

fn main() {
    //let program = include_bytes!("../ld.bin");
    let program = include_bytes!("../hello.bin");
    let mut cpu = CPU::new_with_mem(program.to_vec());
    loop {
        if !cpu.next_instruction() {
            cpu.dump_registers();
            break;
        }
    }
}
