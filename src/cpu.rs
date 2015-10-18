use register::{Register};
use instruction::{self, InstructionType};

pub struct CPU {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,

    pc: u16,

    mem: Vec<u8>,
}

impl CPU {
    
    pub fn new_with_mem(mem: Vec<u8>) -> CPU {
        CPU {
            af: Register { value: 0 },
            bc: Register { value: 0 },
            de: Register { value: 0 },
            hl: Register { value: 0 },
            pc: 0,
            mem: mem,
        }
    }

    //Assumes PC hasn't been moved yet
    pub fn get_arg_u16(&self) -> u16 {
        let pc = self.pc as usize;
        (self.mem[pc + 1] as u16) << 8 | (self.mem[pc + 2] as u16)
    }

    //Assumes PC hasn't been moved yet
    pub fn get_arg_u8(&self) -> u8 {
        self.mem[self.pc as usize + 1]
    }

    pub fn next_instruction(&mut self) {
        let opcode = self.mem[self.pc as usize];
        let instruction_type = instruction::parse_instruction_type(opcode);
        self.handle_instruction(opcode, instruction_type);
    }

    pub fn handle_instruction(&mut self, opcode: u8, instruction_type: InstructionType) {
        match instruction_type {
            _ => (),
        }
    }

    pub fn dump_mem(&self) {
        for x in self.mem.iter() {
            println!("{:X} ", x);
        }
    }

}
