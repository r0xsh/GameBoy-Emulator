use std::fmt;
use bitlab::SingleBits;
use ::{high_byte, low_byte};
use join_bytes;
use ppu::{GpuMode, Ppu};

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

pub enum IterFlag {
    VBLANK,
    LCDSTAT,
    TIMER,
    SERIAL,
    JOYPAD
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
    iter_master: bool,
    iter_enable: u8,
    iter_flags: u8,
    ticks: u64,
    pub ppu: Ppu,
}


impl Cpu {
    /// Init a new Cpu instance
    pub fn new() -> Cpu {
        let mut c = Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            timer: 0,
            iter_master: false,
            iter_enable: 0,
            iter_flags: 0,
            ticks: 0,
            ppu: Ppu::new()
        };

        c
    }

    pub fn reset_no_bios(&mut self) {
        self.a = 0x01;
        self.f = 0xB0;
        self.b = 0x01;
        self.c = 0xB0;
        self.d = 0x01;
        self.e = 0xB0;
        self.h = 0x01;
        self.l = 0xB0;
        self.sp = 0x01;
        self.pc = 0xB0;
        self.iter_master = true;
        self.iter_flags = 0x0;
        self.iter_enable = 0x0;
    }

    pub fn ppu_step(&mut self) {
        match self.ppu.mode {
            GpuMode::VRAM => {
                if self.ppu.tick >= 172 {
                    self.ppu.tick %= 172;
                    self.ppu.mode = GpuMode::HBLANK;
                }
            }
            GpuMode::OAM => {
                if self.ppu.tick >= 80 {
                    self.ppu.mode = GpuMode::VRAM;
                    self.ppu.tick %= 80;
                }
            }
            GpuMode::VBLANK => {
                if self.ppu.tick >= 456 {
                    self.ppu.scanline += 1;

                    self.ppu.tick %= 456;

                    if self.ppu.scanline > 154 {
                        self.ppu.scanline = 0;
                        self.ppu.mode = GpuMode::OAM;
                    }

                }
            }
            GpuMode::HBLANK => {
                if self.ppu.tick >= 204 {
                    self.ppu_hblank();

                    if self.ppu.scanline == 144 {
                        self.set_iter_flag(IterFlag::VBLANK, true);
                        self.ppu.mode = GpuMode::VBLANK;
                    } else {
                        self.ppu.mode = GpuMode::OAM;
                    }

                    self.ppu.tick %= 204;
                }
            }
        }
    }

    pub fn ppu_hblank(&mut self) {
        self.ppu.scanline += 1;
    }

    pub fn interrupt_step(&mut self) {
        if self.get_iter_flag_enable(IterFlag::VBLANK) && self.get_iter_flag(IterFlag::VBLANK) {

        }
    }

    pub fn handle_vblank_inter(&mut self) {
        self.iter_master = false;
    }

    pub fn write_to_stack(&mut self, addr: u16) {

    }

    pub fn set_iter_master(&mut self, state: bool) {
        self.iter_master = state;
    }

    pub fn get_iter_master(&self) -> bool {
        self.iter_master
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

    pub fn set_iter_flag(&mut self, flag: IterFlag, set: bool) {
        match (flag, set) {
            (IterFlag::VBLANK, true) => self.iter_flags = self.iter_flags | 0b1000_0000,
            (IterFlag::LCDSTAT, true) => self.iter_flags = self.iter_flags | 0b0100_0000,
            (IterFlag::TIMER, true) => self.iter_flags = self.iter_flags | 0b0010_0000,
            (IterFlag::SERIAL, true) => self.iter_flags = self.iter_flags | 0b0001_0000,
            (IterFlag::JOYPAD, true) => self.iter_flags = self.iter_flags | 0b0000_1000,
            (IterFlag::VBLANK, false) => self.iter_flags = self.iter_flags & 0b1000_0000,
            (IterFlag::LCDSTAT, false) => self.iter_flags = self.iter_flags & 0b0100_0000,
            (IterFlag::TIMER, false) => self.iter_flags = self.iter_flags & 0b0010_0000,
            (IterFlag::SERIAL, false) => self.iter_flags = self.iter_flags & 0b0001_0000,
            (IterFlag::JOYPAD, false) => self.iter_flags = self.iter_flags & 0b0000_1000,
        }
    }

    pub fn get_iter_flag(&self, flag: IterFlag) -> bool {
        match flag {
            IterFlag::VBLANK => ((self.iter_flags & 0b1000_0000) as u8 >> 7 ) == 1,
            IterFlag::LCDSTAT => ((self.iter_flags & 0b0100_0000) as u8 >> 6 ) == 1,
            IterFlag::TIMER => ((self.iter_flags & 0b0010_0000) as u8 >> 5 ) == 1,
            IterFlag::SERIAL => ((self.iter_flags & 0b0001_0000) as u8 >> 4 ) == 1,
            IterFlag::JOYPAD => ((self.iter_flags & 0b0000_1000) as u8 >> 3 ) == 1,
        }
    }

    pub fn set_iter_flag_enable(&mut self, flag: IterFlag, set: bool) {
        match (flag, set) {
            (IterFlag::VBLANK, true) => self.iter_enable = self.iter_enable | 0b1000_0000,
            (IterFlag::LCDSTAT, true) => self.iter_enable = self.iter_enable | 0b0100_0000,
            (IterFlag::TIMER, true) => self.iter_enable = self.iter_enable | 0b0010_0000,
            (IterFlag::SERIAL, true) => self.iter_enable = self.iter_enable | 0b0001_0000,
            (IterFlag::JOYPAD, true) => self.iter_enable = self.iter_enable | 0b0000_1000,
            (IterFlag::VBLANK, false) => self.iter_enable = self.iter_enable & 0b1000_0000,
            (IterFlag::LCDSTAT, false) => self.iter_enable = self.iter_enable & 0b0100_0000,
            (IterFlag::TIMER, false) => self.iter_enable = self.iter_enable & 0b0010_0000,
            (IterFlag::SERIAL, false) => self.iter_enable = self.iter_enable & 0b0001_0000,
            (IterFlag::JOYPAD, false) => self.iter_enable = self.iter_enable & 0b0000_1000,
        }
    }

    pub fn get_iter_flag_enable(&self, flag: IterFlag) -> bool {
        match flag {
            IterFlag::VBLANK => ((self.iter_enable & 0b1000_0000) as u8 >> 7 ) == 1,
            IterFlag::LCDSTAT => ((self.iter_enable & 0b0100_0000) as u8 >> 6 ) == 1,
            IterFlag::TIMER => ((self.iter_enable & 0b0010_0000) as u8 >> 5 ) == 1,
            IterFlag::SERIAL => ((self.iter_enable & 0b0001_0000) as u8 >> 4 ) == 1,
            IterFlag::JOYPAD => ((self.iter_enable & 0b0000_1000) as u8 >> 3 ) == 1,
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

    pub fn inc_ticks(&mut self, inc: u8) {
        self.ticks += inc as u64;
        if self.ppu.control.get_bit(0).unwrap() {
            self.ppu.tick += inc as u64;
        }
        println!("added ticks +{} = {}", inc, self.ticks * 8);
    }
}


impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "=== CPU DEBUG ===\n\
            > AF        <{:#x}> ({0})\n\
            > BC        <{:#x}> ({1})\n\
            > DE        <{:#x}> ({2})\n\
            > HL        <{:#x}> ({3})\n\
            > SP       <{:#x}> ({4})\n\
            > PC       <{:#x}> ({5})\n\
            > C: {} H: {} N: {} Z: {} NC: {}, NZ: {}",
               self.get_16(Register16::AF),
               self.get_16(Register16::BC),
               self.get_16(Register16::DE),
               self.get_16(Register16::HL),
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

    cpu.reset_flags();

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
