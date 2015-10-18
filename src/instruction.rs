/* 
 * R = 8-bit register
 * RR = 16-bit register
 * N = 8-bit number
 * NN = 16-bit number
 * I_ = indirect (pointer to address)
 */
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

    ADC_R,
    ADC_R_I_HL,
    ADC_R_R,
    ADC_N,

    SUB_R,
    SUB_I_HL,
    SUB_N,

    SBC_R_R,
    SBC_R_I_HL,

    AND_R,
    AND_I_RR,
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
    JR,
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
