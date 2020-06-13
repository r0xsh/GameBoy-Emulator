use std::fmt;
use ::GameBoy;
use cpu::Register16;

static SIZE_TABLE: [u8; 256] = [
 // 0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1, // 0
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, // 1
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, // 2
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 8
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 9
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // a
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // b
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, // c
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1, // d
    2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1, // e
    2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1, // f
];

pub struct Opcode {
    pub opcode: u8,
    pub length: u8,
    pub param:  Option<u16>,
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub p: u8,
    pub q: u8
}

impl Opcode {
    pub fn fetch_param(&mut self, gb: &GameBoy) -> Option<u16> {
        let param = match self.length {
            1 => None,
            2 => Some(gb.cartridge.read_byte(gb.cpu.get_16(Register16::PC) + 1) as u16),
            3 => Some(gb.cartridge.read_word(gb.cpu.get_16(Register16::PC) + 1)),
            _ => unreachable!()
        };
        self.param = param;
        param
    }

    pub fn flag_slice(&self) -> (u8, u8, u8, u8, u8) {
        (
            self.x, self.y,
            self.z, self.p,
            self.q
        )
    }
}

impl From<u8> for Opcode {
    fn from(op: u8) -> Self {
        let y = (op & 0b0011_1000) as u8 >> 3;
        Opcode {
            opcode: op,
            length: SIZE_TABLE[op as usize],
            param: None,
            x: (op & 0b1100_0000) as u8 >> 6,
            y,
            z: (op & 0b0000_0111) as u8,
            p: (y & 0b0000_0110) as u8 >> 1,
            q: (y & 0b0000_0001) as u8
        }
    }
}


impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode ( op: {:x}, len: {}, param: {:#x} )",
            self.opcode,
            self.length,
            self.param.unwrap_or(0)
        )
    }
}
