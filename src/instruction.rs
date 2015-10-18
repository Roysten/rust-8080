/* 
 * R = 8-bit register
 * RR = 16-bit register
 * N = 8-bit number
 * NN = 16-bit number
 * I_ = indirect (pointer to address)
 */
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum InstructionType {
    NOP,

    /* Load */
    LD_R_N,
    LD_R_R,
    LD_RR_NN,
    LD_R_I_NN,
    LD_I_RR_R,
    LD_I_HL_N,
    LD_I_HL_R,
    LD_HL_I_RR,
    LD_R_I_HL,

    /* Arithmetic */
    INC_R,
    INC_RR,
    INC_I_RR,

    DEC_R,
    DEC_RR,
    DEC_I_HL,

    ADD_R_R,
    ADD_RR_RR,
    ADD_R_I_HL,
    ADD_R_N,

    ADC_R_I_HL,
    ADC_R_R,
    ADC_N,

    SUB_R,
    SUB_I_HL,
    SUB_N,

    SBC_R_R,
    SBC_R_I_HL,

    AND_R,
    AND_I_HL,
    AND_N,

    OR_R,
    OR_I_HL,
    OR_N,

    XOR_R,
    XOR_I_HL,
    XOR_N,

    CP_R,
    CP_I_HL,
    CP_N,

    RLCA,
    RRCA,
    RRA,
    RLA,

    CPL,

    /* Exchange */
    EX_RR_RR,

    /* JUMP */
    JR_N,

    /* Conditional */
    JR_Z,
    JR_C,
    JR_NZ,
    JR_NC,
    DJNZ,

    /* Misc. */
    SCF,
    CCF,
    DAA,
    HALT,
}

pub fn parse_instruction_type(opcode: u8) -> InstructionType {
    use instruction::InstructionType::*;
    let instr_type = match opcode {
        0x00 => NOP,
        0x01 | 0x11 | 0x21 | 0x31 => LD_RR_NN,
        0x02 | 0x12 | 0x22 | 0x32 => LD_I_RR_R,
        0x03 | 0x13 | 0x23 | 0x33 => INC_RR,
        0x04 | 0x14 | 0x24 | 0x0C | 0x1C | 0x2C | 0x3C => INC_R, 
        0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => DEC_R,
        0x06 | 0x16 | 0x26 | 0x0E | 0x1E | 0x2E | 0x3E => LD_R_N,
        0x07 => RLCA,
        0x08 => EX_RR_RR,
        0x09 | 0x19 | 0x29 | 0x39 => ADD_RR_RR,
        0x0A | 0x1A | 0x46 | 0x56 | 0x66 | 0x4E | 0x5E | 0x6E | 0x7E => LD_R_I_HL,
        0x0B | 0x1B | 0x2B | 0x3B => DEC_RR,
        0x0F => RRCA,
        0x10 => DJNZ,
        0x17 => RLA,
        0x18 => JR_N,
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
        0xC6 => ADD_R_N,
        0xCE => ADC_N,
        0xD6 => SUB_N,
        0xE6 => AND_N,
        0xEE => XOR_N,
        0xF6 => OR_N,
        0xFE => CP_N,
        //TODO implement the remaining opcodes
        _ => {
            println!("Unimplemented opcode ({:0X})", opcode);
            HALT
        },
    };

    println!("Instruction type: {:?}", instr_type);
    instr_type
}
