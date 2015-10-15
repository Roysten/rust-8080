use register::{Byte, Register};

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
        match self.mem[self.pc as usize] {
            0x00 => (),
            0x01 => {
                let arg = self.get_arg_u16(); //TODO Write macro?
                self.bc.ld(arg);
            },
            0x02 => (), //TODO
            0x03 => self.bc.inc(),
            0x04 => self.bc.inc_8(&Byte::LEFT), //TODO update flags
            0x05 => self.bc.dec_8(&Byte::LEFT), //TODO update flags
            0x06 => {
                let arg = self.get_arg_u8();
                self.bc.ld_8(&Byte::LEFT, arg);
            },
            0x07 => self.af.rot_l_8(&Byte::LEFT), //TODO update flags
            0x08 => (), //TODO
            0x09 => self.hl = self.hl + self.bc,
            0x0A => (), //TODO
            0x0B => (), //TODO
            0x0C => (), //TODO
            0x0D => (), //TODO
            0x0E => (), //TODO
            0x0F => (), //TODO
            0x10 => (), //TODO
            0x11 => (), //TODO
            0x12 => (), //TODO
            0x13 => (), //TODO
            0x14 => (), //TODO
            0x15 => (), //TODO
            0x16 => (), //TODO
            0x17 => (), //TODO
            0x18 => (), //TODO
            0x19 => (), //TODO
            0x1A => (), //TODO
            0x1B => (), //TODO
            0x1C => (), //TODO
            0x1D => (), //TODO
            0x1E => (), //TODO
            0x1F => (), //TODO
            0x20 => (), //TODO
            0x21 => (), //TODO
            0x22 => (), //TODO
            0x23 => (), //TODO
            0x24 => (), //TODO
            0x25 => (), //TODO
            0x26 => (), //TODO
            0x27 => (), //TODO
            0x28 => (), //TODO
            0x29 => (), //TODO
            0x2A => (), //TODO
            0x2B => (), //TODO
            0x2C => (), //TODO
            0x2D => (), //TODO
            0x2E => (), //TODO
            0x2F => (), //TODO
            0x30 => (), //TODO
            0x31 => (), //TODO
            0x32 => (), //TODO
            0x33 => (), //TODO
            0x34 => (), //TODO
            0x35 => (), //TODO
            0x36 => (), //TODO
            0x37 => (), //TODO
            0x38 => (), //TODO
            0x39 => (), //TODO
            0x3A => (), //TODO
            0x3B => (), //TODO
            0x3C => (), //TODO
            0x3D => (), //TODO
            0x3E => (), //TODO
            0x3F => (), //TODO
            0x40 => (), //TODO
            0x41 => (), //TODO
            0x42 => (), //TODO
            0x43 => (), //TODO
            0x44 => (), //TODO
            0x45 => (), //TODO
            0x46 => (), //TODO
            0x47 => (), //TODO
            0x48 => (), //TODO
            0x49 => (), //TODO
            0x4A => (), //TODO
            0x4B => (), //TODO
            0x4C => (), //TODO
            0x4D => (), //TODO
            0x4E => (), //TODO
            0x4F => (), //TODO
            0x50 => (), //TODO
            0x51 => (), //TODO
            0x52 => (), //TODO
            0x53 => (), //TODO
            0x54 => (), //TODO
            0x55 => (), //TODO
            0x56 => (), //TODO
            0x57 => (), //TODO
            0x58 => (), //TODO
            0x59 => (), //TODO
            0x5A => (), //TODO
            0x5B => (), //TODO
            0x5C => (), //TODO
            0x5D => (), //TODO
            0x5E => (), //TODO
            0x5F => (), //TODO
            0x60 => (), //TODO
            0x61 => (), //TODO
            0x62 => (), //TODO
            0x63 => (), //TODO
            0x64 => (), //TODO
            0x65 => (), //TODO
            0x66 => (), //TODO
            0x67 => (), //TODO
            0x68 => (), //TODO
            0x69 => (), //TODO
            0x6A => (), //TODO
            0x6B => (), //TODO
            0x6C => (), //TODO
            0x6D => (), //TODO
            0x6E => (), //TODO
            0x6F => (), //TODO
            0x70 => (), //TODO
            0x71 => (), //TODO
            0x72 => (), //TODO
            0x73 => (), //TODO
            0x74 => (), //TODO
            0x75 => (), //TODO
            0x76 => (), //TODO
            0x77 => (), //TODO
            0x78 => (), //TODO
            0x79 => (), //TODO
            0x7A => (), //TODO
            0x7B => (), //TODO
            0x7C => (), //TODO
            0x7D => (), //TODO
            0x7E => (), //TODO
            0x7F => (), //TODO
            0x80 => (), //TODO
            0x81 => (), //TODO
            0x82 => (), //TODO
            0x83 => (), //TODO
            0x84 => (), //TODO
            0x85 => (), //TODO
            0x86 => (), //TODO
            0x87 => (), //TODO
            0x88 => (), //TODO
            0x89 => (), //TODO
            0x8A => (), //TODO
            0x8B => (), //TODO
            0x8C => (), //TODO
            0x8D => (), //TODO
            0x8E => (), //TODO
            0x8F => (), //TODO
            0x90 => (), //TODO
            0x91 => (), //TODO
            0x92 => (), //TODO
            0x93 => (), //TODO
            0x94 => (), //TODO
            0x95 => (), //TODO
            0x96 => (), //TODO
            0x97 => (), //TODO
            0x98 => (), //TODO
            0x99 => (), //TODO
            0x9A => (), //TODO
            0x9B => (), //TODO
            0x9C => (), //TODO
            0x9D => (), //TODO
            0x9E => (), //TODO
            0x9F => (), //TODO
            0xA0 => (), //TODO
            0xA1 => (), //TODO
            0xA2 => (), //TODO
            0xA3 => (), //TODO
            0xA4 => (), //TODO
            0xA5 => (), //TODO
            0xA6 => (), //TODO
            0xA7 => (), //TODO
            0xA8 => (), //TODO
            0xA9 => (), //TODO
            0xAA => (), //TODO
            0xAB => (), //TODO
            0xAC => (), //TODO
            0xAD => (), //TODO
            0xAE => (), //TODO
            0xAF => (), //TODO
            0xB0 => (), //TODO
            0xB1 => (), //TODO
            0xB2 => (), //TODO
            0xB3 => (), //TODO
            0xB4 => (), //TODO
            0xB5 => (), //TODO
            0xB6 => (), //TODO
            0xB7 => (), //TODO
            0xB8 => (), //TODO
            0xB9 => (), //TODO
            0xBA => (), //TODO
            0xBB => (), //TODO
            0xBC => (), //TODO
            0xBD => (), //TODO
            0xBE => (), //TODO
            0xBF => (), //TODO
            0xC0 => (), //TODO
            0xC1 => (), //TODO
            0xC2 => (), //TODO
            0xC3 => (), //TODO
            0xC4 => (), //TODO
            0xC5 => (), //TODO
            0xC6 => (), //TODO
            0xC7 => (), //TODO
            0xC8 => (), //TODO
            0xC9 => (), //TODO
            0xCA => (), //TODO
            0xCB => (), //TODO
            0xCC => (), //TODO
            0xCD => (), //TODO
            0xCE => (), //TODO
            0xCF => (), //TODO
            0xD0 => (), //TODO
            0xD1 => (), //TODO
            0xD2 => (), //TODO
            0xD3 => (), //TODO
            0xD4 => (), //TODO
            0xD5 => (), //TODO
            0xD6 => (), //TODO
            0xD7 => (), //TODO
            0xD8 => (), //TODO
            0xD9 => (), //TODO
            0xDA => (), //TODO
            0xDB => (), //TODO
            0xDC => (), //TODO
            0xDD => (), //TODO
            0xDE => (), //TODO
            0xDF => (), //TODO
            0xE0 => (), //TODO
            0xE1 => (), //TODO
            0xE2 => (), //TODO
            0xE3 => (), //TODO
            0xE4 => (), //TODO
            0xE5 => (), //TODO
            0xE6 => (), //TODO
            0xE7 => (), //TODO
            0xE8 => (), //TODO
            0xE9 => (), //TODO
            0xEA => (), //TODO
            0xEB => (), //TODO
            0xEC => (), //TODO
            0xED => (), //TODO
            0xEE => (), //TODO
            0xEF => (), //TODO
            0xF0 => (), //TODO
            0xF1 => (), //TODO
            0xF2 => (), //TODO
            0xF3 => (), //TODO
            0xF4 => (), //TODO
            0xF5 => (), //TODO
            0xF6 => (), //TODO
            0xF7 => (), //TODO
            0xF8 => (), //TODO
            0xF9 => (), //TODO
            0xFA => (), //TODO
            0xFB => (), //TODO
            0xFC => (), //TODO
            0xFD => (), //TODO
            0xFE => (), //TODO
            0xFF => (), //TODO
            _ => (),
        }
    }

    pub fn dump_mem(&self) {
        for x in self.mem.iter() {
            println!("{:X} ", x);
        }
    }

}
