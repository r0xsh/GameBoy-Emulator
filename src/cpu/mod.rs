enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L
}

enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

struct Cpu {

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
    pc: u16

}

macro_rules! high_byte {
    ($x:expr) => { ($x >> 8) as u8 };
}

macro_rules! low_byte {
    ($x:expr) => { $x as u8 };
}

macro_rules! join_bytes {
    ($a:expr, $b:expr) => ({
        let mut join: u16 = $b as u16;
        join |= ($a as u16) << 8;
        join
    })
}

impl Cpu {

    /// Init a new Cpu instance
    fn new() -> Cpu {
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
            pc: 0x00
        }
    }

    /// Get a 8bit value from register
    fn get_8(&self, reg: Register8) -> u8 {
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
    fn set_8(&mut self, reg: Register8, v: u8) {
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
    fn get_16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => { join_bytes!(self.a, self.f) }
            Register16::BC => { join_bytes!(self.b, self.c) }
            Register16::DE => { join_bytes!(self.d, self.e) }
            Register16::HL => { join_bytes!(self.h, self.l) }
            Register16::SP => self.sp,
            Register16::PC => self.pc
        }
    }

    /// Set a 16bit value to a register
    fn set_16(&mut self, reg: Register16, v: u16) {
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
            Register16::PC => self.pc = v
        }
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
}
