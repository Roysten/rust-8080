pub const MAX_WORD_SIZE: usize = 255;
pub const MAX_DWORD_SIZE: usize = 65535;

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}
