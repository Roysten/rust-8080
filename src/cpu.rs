use register::{Byte, Register};
use instruction::InstructionType::*;

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
        let instruction = self.mem[self.pc as usize];
        let instr_type = match instruction {
            0x00 => NOP,
            0x01 | 0x11 | 0x21 | 0x31 => LD_RR_NN,
                //let arg = self.get_arg_u16(); //TODO Write macro?
                //self.bc.ld(arg);
            0x02 | 0x12 | 0x22 | 0x32 => LD_I_RR_R,
            0x03 | 0x13 | 0x23 | 0x33 => INC_RR, //self.bc.inc(),
            0x04 | 0x14 | 0x24 | 0x0C | 0x1C | 0x2C | 0x3C => INC_R, //self.bc.inc_8(&Byte::LEFT), //TODO update flags
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => DEC_R, //self.bc.dec_8(&Byte::LEFT), //TODO update flags
            0x06 | 0x16 | 0x26 | 0x0E | 0x1E | 0x2E | 0x3E => LD_R_N, //{
                //let arg = self.get_arg_u8();
                //self.bc.ld_8(&Byte::LEFT, arg);
            //},
            0x07 => RLCA,//self.af.rot_l_8(&Byte::LEFT), //TODO update flags
            0x08 => EX_RR_RR,
            0x09 | 0x19 | 0x29 | 0x39 => ADD_RR_RR,//self.hl = self.hl + self.bc,
            0x0A | 0x1A | 0x46 | 0x56 | 0x66 | 0x4E | 0x5E | 0x6E | 0x7E => LD_R_I_HL,
            0x0B | 0x1B | 0x2B | 0x3B => DEC_RR,
            0x0F => RRCA,
            0x10 => DJNZ,
            0x17 => RLA,
            0x18 => JR,
            0x1F => RRA,
            0x20 => JR_NZ,
            0x27 => DAA,
            0x28 => JR_Z,
            0x2A => LD_HL_I_RR,
            0x2F => CPL,
            0x30 => JR_NC,
            0x34 => INC_I_RR,
            0x35 => DEC_I_HL,
            0x36 => LD_I_HL_N,
            0x37 => SCF,
            0x38 => JR_C,
            0x3A => LD_R_I_NN,
            0x3F => CCF,
            0x40 ... 0x45 | 0x47 ... 0x4D | 0x4F ... 0x55 | 0x57 ... 0x5D |
            0x5F ... 0x65 | 0x67 ... 0x6D | 0x6F | 0x78 ... 0x7D | 0x7F => LD_R_R,
            0x70 ... 0x75 | 0x77 => LD_I_HL_R,
            0x76 => HALT,
            0x80 ... 0x85 | 0x87 => ADD_R_R,
            0x86 => ADD_R_I_HL,
            0x88 ... 0x8D | 0x8F => ADC_R_R,
            0x8E => ADC_R_I_HL,
            0x90 ... 0x95 | 0x97 => SUB_R,
            0x96 => SUB_I_HL,
            0x98 ... 0x9D | 0x9F => SBC_R_R,
            0x9E => SBC_R_I_HL,
            0xA0 ... 0xA5 | 0xA7 => AND_R,
            0xA6 => AND_I_HL,
            0xA8 ... 0xAD | 0xAF => XOR_R,
            0xAE => XOR_I_HL,
            0xB0 ... 0xB5 | 0xB7 => OR_R,
            0xB6 => OR_I_HL,
            0xB8 ... 0xBD | 0xBF => CP_R,
            0xBE => CP_I_HL,
            _ => {
                println!("Unimplemented instruction ({:0X})", instruction);
                HALT
            },
        };

        println!("Instruction type: {:?}", instr_type);
    }

    pub fn dump_mem(&self) {
        for x in self.mem.iter() {
            println!("{:X} ", x);
        }
    }

}
