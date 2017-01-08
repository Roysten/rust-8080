use register::Reg;

/* 
 * I = indirect (pointer to address)
 */
#[derive(Debug)]
pub enum InstructionArgs {
    Number(u16),
    Register(Reg),
    NumberRegister(u16, Reg),
    RegisterNumber(Reg, u16),
    RegisterRegister(Reg, Reg),
    Indirect(u16),
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Mov(InstructionArgs),
    Inc(InstructionArgs),
    Dec(InstructionArgs),
    Add(InstructionArgs),
    Adc(InstructionArgs),
    Sub(InstructionArgs),
    Sbc(InstructionArgs),
    And(InstructionArgs),
    Or(InstructionArgs),
    Xor(InstructionArgs),
    Cp(InstructionArgs),
    Rlca,
    Rrca,
    Rra,
    Rla,
    Cpl,

    /* Exchange */
    Ex_rr_rr,

    /* jump */

    Jrn,
    /* Conditional */
    Jrz,
    Jrnz,
    Jrnc,
    Djnz,

    /* Misc. */
    Scf,
    Ccf,
    Daa,
    Halt,
}

fn extract_argument(opcode: u8) -> Reg {
    match opcode & 0x0F {
        0x0 | 0x8 => Reg::B,
        0x1 | 0x9 => Reg::C,
        0x2 | 0xA => Reg::D,
        0x3 | 0xB => Reg::E,
        0x4 | 0xC => Reg::H,
        0x5 | 0xD => Reg::L,
        0x6 | 0xE => Reg::HL,
        0x7 | 0xF => Reg::A,
        _ => panic!("Could not resolve operating register for opcode {:x}"),
    }
}

pub fn parse_instruction_type(pc: u16, mem: &[u8]) -> (u16, Instruction) {
    use instruction::Instruction::*;
    use instruction::InstructionArgs::*;

    let opcode = mem[pc as usize];
    match opcode {
        0x00 => (1, Nop),
        0x3E => (2, Mov(RegisterNumber(Reg::A, mem[pc as usize + 1] as u16))),
        0x06 => (2, Mov(RegisterNumber(Reg::B, mem[pc as usize + 1] as u16))),
        0x40 ... 0x47 => (1, Mov(RegisterRegister(Reg::B, extract_argument(opcode)))),
        0x48 ... 0x4F => (1, Mov(RegisterRegister(Reg::C, extract_argument(opcode)))),
        0x50 ... 0x57 => (1, Mov(RegisterRegister(Reg::D, extract_argument(opcode)))),
        0x58 ... 0x5F => (1, Mov(RegisterRegister(Reg::E, extract_argument(opcode)))),
        0x60 ... 0x67 => (1, Mov(RegisterRegister(Reg::H, extract_argument(opcode)))),
        0x68 ... 0x6F => (1, Mov(RegisterRegister(Reg::L, extract_argument(opcode)))),
        0x70 ... 0x77 => (1, Mov(RegisterRegister(Reg::HL, extract_argument(opcode)))),
        0x78 ... 0x7F => (1, Mov(RegisterRegister(Reg::A, extract_argument(opcode)))),
        0x80 ... 0x87 => (1, Add(Register(extract_argument(opcode)))),
        0x88 ... 0x8F => (1, Adc(Register(extract_argument(opcode)))),
        0x90 ... 0x97 => (1, Sub(Register(extract_argument(opcode)))),
        0x98 ... 0x9F => (1, Sbc(Register(extract_argument(opcode)))),
        0xA0 ... 0xA7 => (1, And(Register(extract_argument(opcode)))),
        0xA8 ... 0xAF => (1, Xor(Register(extract_argument(opcode)))),
        0xB0 ... 0xB7 => (1, Or(Register(extract_argument(opcode)))),
        0xB8 ... 0xBF => (1, Cp(Register(extract_argument(opcode)))),
        //TODO implement the remaining opcodes
        _ => {
            println!("Unimplemented opcode ({:0X})", opcode);
            (1, Halt)
        },
    }
}
