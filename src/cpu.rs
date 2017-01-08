use register::{self, Reg};
use instruction::{self, Instruction, InstructionArgs};
use instruction::Instruction::*;
use instruction::InstructionArgs::*;

enum Flag {
    Sign = 0,
    Zero = 1,
    AuxCarry = 2,
    Parity = 3,
    Carry = 4,
}

pub struct CPU {
    af: u8,
    bc: u16,
    de: u16,
    hl: u16,
    flags: [bool; 5],

    pc: u16,
    mem: Vec<u8>,
}

impl CPU {
    
    pub fn new_with_mem(mem: Vec<u8>) -> CPU {
        CPU {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            pc: 0,
            mem: mem,
            flags: [false; 5],
        }
    }

    pub fn next_instruction(&mut self) -> bool {
        let instruction = instruction::parse_instruction_type(self.pc, &self.mem);
        self.handle_instruction(instruction);
        (self.pc as usize) < self.mem.len()
    }

    pub fn dump_mem(&self) {
        for (i, x) in self.mem.iter().enumerate() {
            print!("{:02X} ", x);
            if i == 79 {
                println!("");
            }
        }
    }

    pub fn dump_registers(&self)
    {
        println!("af: {:02X}", self.af as u8);
        println!("bc: {:02X} {:02X}", self.bc >> 8, self.bc as u8);
        println!("de: {:02X} {:02X}", self.de >> 8, self.de as u8);
        println!("hl: {:02X} {:02X}", self.hl >> 8, self.hl as u8);
        println!("pc: {:02X}", self.pc);
        println!("");
        println!("Sign: {}", self.flags[Flag::Sign as usize]);
        println!("Zero: {}", self.flags[Flag::Zero as usize]);
        println!("AuxCarry: {}", self.flags[Flag::AuxCarry as usize]);
        println!("Parity: {}", self.flags[Flag::Parity as usize]);
        println!("Carry: {}", self.flags[Flag::Carry as usize]);
    }

    fn handle_instruction(&mut self, instruction_data: (u16, Instruction)) {
        let (inc_pc, instruction) = instruction_data;
        match instruction {
            Mov(args) => self.mov(args),
            Add(args) => self.add(args),
            Adc(args) => self.adc(args),
            Sub(args) => self.sub(args),
            Sbc(args) => self.sbc(args),
            _ => (),
        }

        self.pc += inc_pc;
    }

    fn mov(&mut self, args: InstructionArgs) {
        match args {
            RegisterNumber(reg, val) => self.write_register_u8(reg, val as u8),
            _ => panic!("unhandled add operation {:?}", args),
        }
    }

    fn add(&mut self, args: InstructionArgs) {
        match args {
            Register(reg) => {
                let (val_1, val_2) = (self.read_register_u8(Reg::A), self.read_register_u8(reg));
                let (new_value, carry) = val_1.overflowing_add(val_2);
                if carry {
                    self.flags[Flag::Carry as usize] = true;
                }
                self.update_flags_u8(new_value);

                self.write_register_u8(Reg::A, new_value);
            }
            _ => panic!("unhandled add operation {:?}", args),
        }
    }

    fn adc(&mut self, args: InstructionArgs) {
        match args {
            Register(reg) => {
                let (val_1, val_2) = (self.read_register_u8(Reg::A), self.read_register_u8(reg));
                let (new_value, carry) = val_1.overflowing_add(val_2 + self.flags[Flag::Carry as usize] as u8);
                if carry {
                    self.flags[Flag::Carry as usize] = true;
                }
                self.update_flags_u8(new_value);

                self.write_register_u8(Reg::A, new_value);
            }
            _ => panic!("unhandled add operation {:?}", args),
        }
    }

    fn sub(&mut self, args: InstructionArgs) {
        match args {
            Register(reg) => {
                let (val_1, val_2) = (self.read_register_u8(Reg::A), self.read_register_u8(reg));
                let (new_value, carry) = val_1.overflowing_sub(val_2);
                if carry {
                    self.flags[Flag::Carry as usize] = true;
                }
                self.update_flags_u8(new_value);

                self.write_register_u8(Reg::A, new_value);
            }
            _ => panic!("unhandled add operation {:?}", args),
        }
    }

    fn sbc(&mut self, args: InstructionArgs) {
        match args {
            Register(reg) => {
                let (val_1, val_2) = (self.read_register_u8(Reg::A), self.read_register_u8(reg));
                let (new_value, carry) = val_1.overflowing_sub(val_2 - self.flags[Flag::Carry as usize] as u8);
                if carry {
                    self.flags[Flag::Carry as usize] = true;
                }
                self.update_flags_u8(new_value);

                self.write_register_u8(Reg::A, new_value);
            }
            _ => panic!("unhandled add operation {:?}", args),
        }
    }

    fn update_flags_u8(&mut self, value: u8) {
        self.flags[Flag::Sign as usize] = value >> 7 == 1;
        self.flags[Flag::Zero as usize] = value == 0;
        self.flags[Flag::Parity as usize] = value.count_ones() % 2 == 0;
        //TODO: AuxCarry flag
    }

    fn read_register_u8(&self, register: Reg) -> u8 {
        match register {
            Reg::A => self.af,
            Reg::B => (self.bc >> 8) as u8,
            Reg::C => self.bc as u8,
            Reg::D => (self.de >> 8) as u8,
            Reg::E => self.de as u8,
            _ => panic!("todo"),
        }
    }

    fn write_register_u8(&mut self, register: Reg, val: u8) {
        match register {
            Reg::A => self.af = val,
            Reg::B => self.bc = (val as u16) << 8,
            Reg::C => self.bc = (self.bc & 0xF0) | val as u16,
            Reg::D => self.de = (val as u16) << 8,
            Reg::E => self.de = (self.de & 0xF0) | val as u16,
            _ => panic!("todo"),
        };
    }
}
