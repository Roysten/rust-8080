macro_rules! r {
    ($opcode:expr) => {
        let register = $opcode as u8;
        $opcode`
    }
}
