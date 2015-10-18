use std::ops::Add;

pub enum Reg {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum Byte {
    LEFT, RIGHT
}

#[derive(Copy, Clone)]
pub struct Register {
    pub value: u16,
}

impl Add for Register {
    type Output = Register;
        
    fn add(self, other: Register) -> Register {
        Register { value: self.value + other.value }
    }

}

impl Register {
   
    pub fn get_byte(&self, pos: &Byte) -> u8 {
        match *pos {
            Byte::RIGHT => (self.value >> 8) as u8,
            Byte::LEFT => self.value as u8,
        }
    }

    pub fn set_byte(&mut self, pos: &Byte, byte: u8) {
        match *pos {
            Byte::RIGHT => self.value = (self.value & 0x00FF) | ((byte as u16) << 8),
            Byte::LEFT => self.value = (self.value & 0xFF00) | (byte as u16),
        }
    }

    pub fn ld(&mut self, value: u16) {
        self.value = value;
    }

    pub fn ld_8(&mut self, pos: &Byte, value: u8) {
        self.set_byte(pos, value);
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn inc_8(&mut self, pos: &Byte) {
        let byte = self.get_byte(pos) + 1;
        self.set_byte(pos, byte);
    }

    pub fn dec_8(&mut self, pos: &Byte) {
        let byte = self.get_byte(pos) - 1;
        self.set_byte(pos, byte);
    }
    
    pub fn rot_l_8(&mut self, pos: &Byte) {
        let byte = self.get_byte(pos);
        byte.rotate_left(1);
        self.set_byte(pos, byte);
    }

}

#[test]
fn test_get_byte() {
    let r = Register { value: 0xFF00 };
    assert_eq!(r.get_byte(Byte::RIGHT), 0xFF);
    assert_eq!(r.get_byte(Byte::LEFT), 0x00);
}

#[test]
fn test_set_byte() {
    let mut r = Register { value: 0x0000 };
    r.set_byte(Byte::RIGHT, 0xFF); 
    assert_eq!(r.get_byte(Byte::RIGHT), 0xFF);
    r.set_byte(Byte::RIGHT, 0x12); 
    assert_eq!(r.get_byte(Byte::RIGHT), 0x12);

    r.set_byte(Byte::LEFT, 0xFF); 
    assert_eq!(r.get_byte(Byte::LEFT), 0xFF);
    r.set_byte(Byte::LEFT, 0x12); 
    assert_eq!(r.get_byte(Byte::LEFT), 0x12);
}
