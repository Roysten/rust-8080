macro_rules! r {
    ($opcode:expr) => {
        {
            let register = $opcode as u8;
            match register {
                0x07 | 0x0E => Reg::A,
                0x00 | 0x08 => Reg::B,
                0x01 | 0x09 => Reg::C,
                0x02 | 0x0A => Reg::D,
                0x03 | 0x0B => Reg::E,
                0x04 | 0x0C => Reg::H,
                0x05 | 0x0E => Reg::L,
            }
        }
    }
}
