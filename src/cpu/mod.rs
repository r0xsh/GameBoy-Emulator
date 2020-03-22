use std::fmt;
use ::Memory;
pub mod opcodes;
pub mod opcode;

#[derive(Clone, Copy)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum Flag {
    Z,
    NZ,
    N,
    H,
    C,
    NC,
}

pub struct Cpu {
    /// Accumulator register
    a: u8,
    /// Flags register
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    timer: u8,
}


impl Cpu {
    /// Init a new Cpu instance
    pub fn new() -> Cpu {
        Cpu {
            a: 0x0,
            f: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            sp: 0x00,
            pc: 0x00,
            timer: 0x0,
        }
    }

    /// Get a 8bit value from register
    pub fn get_8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.a,
            Register8::F => self.f,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    /// Set a 8bit value to a register
    pub fn set_8(&mut self, reg: Register8, v: u8) {
        match reg {
            Register8::A => self.a = v,
            Register8::F => self.f = v,
            Register8::B => self.b = v,
            Register8::C => self.c = v,
            Register8::D => self.d = v,
            Register8::E => self.e = v,
            Register8::H => self.h = v,
            Register8::L => self.l = v,
        };
    }

    /// Get a 16bit value from a register
    pub fn get_16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => join_bytes!(self.a, self.f),
            Register16::BC => join_bytes!(self.b, self.c),
            Register16::DE => join_bytes!(self.d, self.e),
            Register16::HL => join_bytes!(self.h, self.l),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    /// Set a 16bit value to a register
    pub fn set_16(&mut self, reg: Register16, v: u16) {
        match reg {
            Register16::AF => {
                self.a = high_byte!(v);
                self.f = low_byte!(v);
            }
            Register16::BC => {
                self.b = high_byte!(v);
                self.c = low_byte!(v);
            }
            Register16::DE => {
                self.d = high_byte!(v);
                self.e = low_byte!(v);
            }
            Register16::HL => {
                self.h = high_byte!(v);
                self.l = low_byte!(v);
            }
            Register16::SP => self.sp = v,
            Register16::PC => self.pc = v,
        }
    }

    /// Set a flags
    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        let f = self.get_8(Register8::F);
        match (flag, set) {
            (Flag::Z, true) => self.set_8(Register8::F, f | 0b1000_0000),
            (Flag::N, true) => self.set_8(Register8::F, f | 0b0100_0000),
            (Flag::H, true) => self.set_8(Register8::F, f | 0b0010_0000),
            (Flag::C, true) => self.set_8(Register8::F, f | 0b0001_0000),
            (Flag::Z, false) => self.set_8(Register8::F, f & 0b0111_1111),
            (Flag::N, false) => self.set_8(Register8::F, f & 0b1011_1111),
            (Flag::H, false) => self.set_8(Register8::F, f & 0b1101_1111),
            (Flag::C, false) => self.set_8(Register8::F, f & 0b1110_1111),
            _ => unreachable!(),
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let f = self.get_8(Register8::F);
        match flag {
            Flag::C => ((f & 0b0001_0000) as u8 >> 4 ) == 1,
            Flag::H => ((f & 0b0010_0000) as u8 >> 5 ) == 1,
            Flag::N => ((f & 0b0100_0000) as u8 >> 6 ) == 1,
            Flag::Z => ((f & 0b1000_0000) as u8 >> 7 ) == 1,
            Flag::NC => ((f & 0b0001_0000) as u8 >> 4 ) == 0,
            Flag::NZ => ((f & 0b1000_0000) as u8 >> 7 ) == 0,
        }
    }

    /// Set all the flags return to false
    pub fn reset_flags(&mut self) {
        self.set_flag(Flag::Z, false);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
    }

    /// Inc 8bit register by 1
    pub fn inc_8(&mut self, reg: Register8) {
        match reg {
            Register8::A => self.a += 1,
            Register8::F => self.f += 1,
            Register8::B => self.b += 1,
            Register8::C => self.c += 1,
            Register8::D => self.d += 1,
            Register8::E => self.e += 1,
            Register8::H => self.h += 1,
            Register8::L => self.l += 1,
        };
    }

    /// Inc 16bit register by 1
    pub fn inc_16(&mut self, reg: Register16) {
        let v: u16 = self.get_16(reg) + 1;
        match reg {
            Register16::AF => {
                self.a = high_byte!(v);
                self.f = low_byte!(v);
            }
            Register16::BC => {
                self.b = high_byte!(v);
                self.c = low_byte!(v);
            }
            Register16::DE => {
                self.d = high_byte!(v);
                self.e = low_byte!(v);
            }
            Register16::HL => {
                self.h = high_byte!(v);
                self.l = low_byte!(v);
            }
            Register16::SP => self.sp = v,
            Register16::PC => self.pc = v,
        }

    }

    /// Inc PC by x
    pub fn inc_pc(&mut self, inc: u8) {
        self.pc += inc as u16;
    }
}


impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "=== CPU DEBUG ===\n\
            > A        <{:#x}> ({0})\n\
            > F (flag) <{:#x}> ({1})\n\
            > B        <{:#x}> ({2})\n\
            > C        <{:#x}> ({3})\n\
            > D        <{:#x}> ({4})\n\
            > E        <{:#x}> ({5})\n\
            > H        <{:#x}> ({6})\n\
            > L        <{:#x}> ({7})\n\
            > SP       <{:#x}> ({8})\n\
            > PC       <{:#x}> ({9})\n\
            > C: {} H: {} N: {} Z: {} NC: {}, NZ: {}",
               self.get_8(Register8::A),
               self.get_8(Register8::F),
               self.get_8(Register8::B),
               self.get_8(Register8::C),
               self.get_8(Register8::D),
               self.get_8(Register8::E),
               self.get_8(Register8::H),
               self.get_8(Register8::L),
               self.get_16(Register16::SP),
               self.get_16(Register16::PC),
               self.get_flag(Flag::C),
               self.get_flag(Flag::H),
               self.get_flag(Flag::N),
               self.get_flag(Flag::Z),
               self.get_flag(Flag::NC),
               self.get_flag(Flag::NZ))
    }
}

#[test]
fn set_get() {
    let mut cpu = Cpu::new();

    cpu.set_8(Register8::A, 50);
    cpu.set_8(Register8::F, 51);
    cpu.set_8(Register8::B, 52);
    cpu.set_8(Register8::C, 53);
    cpu.set_8(Register8::D, 54);
    cpu.set_8(Register8::E, 55);
    cpu.set_8(Register8::H, 56);
    cpu.set_8(Register8::L, 57);
    assert_eq!(cpu.get_8(Register8::A), 50);
    assert_eq!(cpu.get_8(Register8::F), 51);
    assert_eq!(cpu.get_8(Register8::B), 52);
    assert_eq!(cpu.get_8(Register8::C), 53);
    assert_eq!(cpu.get_8(Register8::D), 54);
    assert_eq!(cpu.get_8(Register8::E), 55);
    assert_eq!(cpu.get_8(Register8::H), 56);
    assert_eq!(cpu.get_8(Register8::L), 57);

    cpu.set_16(Register16::AF, 50_000);
    cpu.set_16(Register16::BC, 51_000);
    cpu.set_16(Register16::DE, 52_000);
    cpu.set_16(Register16::HL, 53_000);
    cpu.set_16(Register16::SP, 54_000);
    cpu.set_16(Register16::PC, 55_000);
    assert_eq!(cpu.get_16(Register16::AF), 50_000);
    assert_eq!(cpu.get_16(Register16::BC), 51_000);
    assert_eq!(cpu.get_16(Register16::DE), 52_000);
    assert_eq!(cpu.get_16(Register16::HL), 53_000);
    assert_eq!(cpu.get_16(Register16::SP), 54_000);
    assert_eq!(cpu.get_16(Register16::PC), 55_000);

    cpu.inc_16(Register16::AF);
    cpu.inc_16(Register16::BC);
    cpu.inc_16(Register16::DE);
    cpu.inc_16(Register16::HL);
    cpu.inc_16(Register16::SP);
    cpu.inc_16(Register16::PC);
    assert_eq!(cpu.get_16(Register16::AF), 50_001);
    assert_eq!(cpu.get_16(Register16::BC), 51_001);
    assert_eq!(cpu.get_16(Register16::DE), 52_001);
    assert_eq!(cpu.get_16(Register16::HL), 53_001);
    assert_eq!(cpu.get_16(Register16::SP), 54_001);
    assert_eq!(cpu.get_16(Register16::PC), 55_001);


}

#[test]
fn flags() {
    let mut cpu = Cpu::new();

    cpu.set_flag(Flag::Z, true);
    assert_eq!(cpu.get_8(Register8::F), 0b10000000);
    cpu.set_flag(Flag::N, true);
    assert_eq!(cpu.get_8(Register8::F), 0b11000000);
    cpu.set_flag(Flag::H, true);
    assert_eq!(cpu.get_8(Register8::F), 0b11100000);
    cpu.set_flag(Flag::C, true);
    assert_eq!(cpu.get_8(Register8::F), 0b11110000);

    cpu.set_flag(Flag::Z, false);
    assert_eq!(cpu.get_8(Register8::F), 0b01110000);
    cpu.set_flag(Flag::N, false);
    assert_eq!(cpu.get_8(Register8::F), 0b00110000);
    cpu.set_flag(Flag::H, false);
    assert_eq!(cpu.get_8(Register8::F), 0b00010000);
    cpu.set_flag(Flag::C, false);
    assert_eq!(cpu.get_8(Register8::F), 0b00000000);
}
