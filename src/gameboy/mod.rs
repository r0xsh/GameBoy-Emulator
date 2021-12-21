use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use bitlab::SingleBits;
use ::cpu::Cpu;
use ::cpu::{Register8, Register16, Flag};
use ::memory::Memory;
use cartridge::Cartridge;
use cpu::IterFlag;
use ::{high_byte, join_bytes};
use low_byte;

pub struct GameBoy {
    pub cpu: Box<Cpu>,
    pub boot_rom: Vec<u8>,
    pub cartridge: Box<Cartridge>,
    pub mem: Box<Memory>,
    pub stopped: bool,
}

impl GameBoy {
    pub fn new(
        cpu: Box<Cpu>,
        boot_rom: Vec<u8>,
        cartridge: Box<Cartridge>,
        mem: Box<Memory>
    ) -> GameBoy {
        GameBoy {
            cpu,
            boot_rom,
            cartridge,
            mem,
            stopped: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        return match addr {
            0..=0x3FFF => {
                if addr <= 0xFF {
                    return self.boot_rom[addr as usize];
                }
                return self.cartridge.read_byte(addr);
            },
            0x4000..=0x7FFF => self.cartridge.read_byte(addr), //self.cartridge[addr as usize],
            0xE000..=0xFDFF => self.mem.read_byte(addr - 0x2000), //- 0x2000
            0xFF04..=0xFF07 => {
                println!("UNIMPLEMENTED READ FROM TIMER");
                exit(101);
            }
            0xFF40 => self.cpu.ppu.control,
            0xFF42 => self.cpu.ppu.scroll_y,
            0xFF43 => self.cpu.ppu.scroll_x,
            0xFF44 => self.cpu.ppu.scanline,
            0xFF45 => {
                exit(1000);
            }
            _ => self.mem.read_byte(addr),
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let a = self.read_byte(addr);
        let b = self.read_byte(addr + 1);
        join_bytes!(b, a)
    }

    pub fn write_byte(&mut self, addr: u16, v: u8) {
        match addr {
            0..=0x3FFF => self.cartridge.write_byte(addr, v),
            0x4000..=0x7FFF => self.cartridge.write_byte(addr, v),
            0xE000..=0xFDFF => self.mem.write_byte(addr - 0x2000, v),
            0xFF40 => self.cpu.ppu.control = v,
            0xFF42 => self.cpu.ppu.scroll_y = v,
            0xFF43 => self.cpu.ppu.scroll_x = v,
            0xFF44 => self.cpu.ppu.scanline = 0,
            0xFF45 => {
                exit(1000);
            }
            0xFF46 => {
                self.mem.write_byte(addr, v);
                self.dma_transfer(v);
            }
            _ =>  self.mem.write_byte(addr, v),
        }
    }

    pub fn write_word(&mut self, addr: u16, v: u16) {
        self.write_byte(addr + 1, high_byte!(v));
        self.write_byte(addr, low_byte!(v));
    }

    pub fn dma_transfer(&mut self, addr: u8) {
        let offset_addr: u16 = addr as u16 * 0x100;

        for i in 0..0x9F {
            let from = offset_addr + i;
            let to = 0xFE00 + i;

            self.write_byte(to, self.read_byte(from));
        }
    }

    pub fn interrupt_step(&mut self) {
        let mut pc = 0;
        if self.cpu.get_iter_flag_enable(IterFlag::VBLANK) &&
            self.cpu.get_iter_flag(IterFlag::VBLANK) {
            pc = 0x40;
            self.cpu.set_iter_flag(IterFlag::VBLANK, false);
        }
        if self.cpu.get_iter_flag_enable(IterFlag::LCDSTAT) &&
            self.cpu.get_iter_flag(IterFlag::LCDSTAT) {
            pc = 0x48;
            self.cpu.set_iter_flag(IterFlag::LCDSTAT, false);
        }
        if self.cpu.get_iter_flag_enable(IterFlag::TIMER) &&
            self.cpu.get_iter_flag(IterFlag::TIMER) {
            pc = 0x50;
            self.cpu.set_iter_flag(IterFlag::TIMER, false);
        }
        if self.cpu.get_iter_flag_enable(IterFlag::SERIAL) &&
            self.cpu.get_iter_flag(IterFlag::SERIAL) {
            pc = 0x58;
            self.cpu.set_iter_flag(IterFlag::SERIAL, false);
        }
        if self.cpu.get_iter_flag_enable(IterFlag::JOYPAD) &&
            self.cpu.get_iter_flag(IterFlag::JOYPAD) {
            pc = 0x60;
            self.cpu.set_iter_flag(IterFlag::JOYPAD, false);
        }

        if pc != 0 {
            self.cpu.set_iter_master(false);
            self.write_to_stack(self.cpu.get_16(Register16::PC));
            self.cpu.set_16(Register16::PC, pc);
            self.cpu.inc_ticks(12);
        }
    }

    pub fn reti(&mut self) {
        self.cpu.set_iter_master(true);
        self.read_from_stack();
    }

    pub fn write_to_stack(&mut self, addr: u16) {
        self.cpu.set_16(Register16::SP, self.cpu.get_16(Register16::SP) - 2);
        self.mem.write_word(self.cpu.get_16(Register16::SP), addr);
    }

    pub fn read_from_stack(&mut self) {
        self.cpu.set_16(
            Register16::PC,
            self.mem.read_word(self.cpu.get_16(Register16::SP))
        );
        self.cpu.set_16(Register16::SP, self.cpu.get_16(Register16::SP) + 2);
    }

    pub fn get_table_cc(&self, index: u8) -> bool {
        match index {
            0 => self.cpu.get_flag(Flag::NZ),
            1 => self.cpu.get_flag(Flag::Z),
            2 => self.cpu.get_flag(Flag::NC),
            3 => self.cpu.get_flag(Flag::C),
            _ => unreachable!()
        }
    }

    pub fn get_table_r(&self, index: u8) -> u8 {
        match index {
            0 => self.cpu.get_8(Register8::B),
            1 => self.cpu.get_8(Register8::C),
            2 => self.cpu.get_8(Register8::D),
            3 => self.cpu.get_8(Register8::E),
            4 => self.cpu.get_8(Register8::H),
            5 => self.cpu.get_8(Register8::L),
            6 => self.mem.read_byte(self.cpu.get_16(Register16::HL)),
            7 => self.cpu.get_8(Register8::A),
            _ => unreachable!()
        }
    }

    pub fn get_table_rp(&self, index: u8) -> u16 {
        match index {
            0 => self.cpu.get_16(Register16::BC),
            1 => self.cpu.get_16(Register16::DE),
            2 => self.cpu.get_16(Register16::HL),
            3 => self.cpu.get_16(Register16::SP),
            _ => unreachable!()
        }
    }

    pub fn get_table_rp2(&self, index: u8) -> u16 {
        match index {
            0 => self.cpu.get_16(Register16::BC),
            1 => self.cpu.get_16(Register16::DE),
            2 => self.cpu.get_16(Register16::HL),
            3 => self.cpu.get_16(Register16::AF),
            _ => unreachable!()
        }
    }


    pub fn set_table_cc(&mut self, index: u8, value: bool) {
        match index {
            0 => self.cpu.set_flag(Flag::NZ, value),
            1 => self.cpu.set_flag(Flag::Z, value),
            2 => self.cpu.set_flag(Flag::NC, value),
            3 => self.cpu.set_flag(Flag::C, value),
            _ => unreachable!()
        }
    }

    pub fn set_table_r(&mut self, index: u8, value: u8) {
        match index {
            0 => self.cpu.set_8(Register8::B, value),
            1 => self.cpu.set_8(Register8::C, value),
            2 => self.cpu.set_8(Register8::D, value),
            3 => self.cpu.set_8(Register8::E, value),
            4 => self.cpu.set_8(Register8::H, value),
            5 => self.cpu.set_8(Register8::L, value),
            6 => self.mem.write_byte(self.cpu.get_16(Register16::HL), value),
            7 => self.cpu.set_8(Register8::A, value),
            _ => unreachable!()
        }
    }

    pub fn set_table_rp(&mut self, index: u8, value: u16) {
        match index {
            0 => self.cpu.set_16(Register16::BC, value),
            1 => self.cpu.set_16(Register16::DE, value),
            2 => self.cpu.set_16(Register16::HL, value),
            3 => self.cpu.set_16(Register16::SP, value),
            _ => unreachable!()
        }
    }

    pub fn set_table_rp2(&mut self, index: u8, value: u16) {
        match index {
            0 => self.cpu.set_16(Register16::BC, value),
            1 => self.cpu.set_16(Register16::DE, value),
            2 => self.cpu.set_16(Register16::HL, value),
            3 => self.cpu.set_16(Register16::AF, value),
            _ => unreachable!()
        }
    }

    pub fn exec_table_rot(&mut self, index: u8, value: u8) -> u8 {
        match index {
            0 => {
                let carry = value.get_bit(0).unwrap();
                let mut res = value << 1;

                self.cpu.set_flag(Flag::C, carry);
                if carry {
                    res = res + 1;
                }

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::N, false);
                res
            } // RLC,
            1 => {
                let carry = value.get_bit(7).unwrap();
                let mut res = value >> 1;

                self.cpu.set_flag(Flag::C, carry);
                if carry {
                    res = res | 0b1000_0000;
                }

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::N, false);
                res
            } // RRC,
            2 => {
                let carry = self.cpu.get_flag(Flag::C);
                let mut res = value << 1;

                self.cpu.set_flag(Flag::C, value.get_bit(0).unwrap());
                if carry {
                    res = res + 1;
                }

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::N, false);
                res
            } // RL,
            3 => {
                let mut res = value >> 1;
                if self.cpu.get_flag(Flag::C) {
                    res = res | 0b1000_0000;
                }

                self.cpu.set_flag(Flag::C, value.get_bit(7).unwrap());

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::N, false);
                res
            } // RR,
            4 => {
                self.cpu.set_flag(Flag::C, value.get_bit(0).unwrap());

                let mut res = value << 1;

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::N, false);
                res
            } // SLA,
            5 => {
                self.cpu.set_flag(Flag::C, value.get_bit(7).unwrap());

                let mut res = (value & 0x80) | (value >> 1);

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::N, false);
                res
            } // SRA,
            6 => {
                let mut res = ((value & 0xF) << 4) | ((value & 0xF0) >> 4);

                self.cpu.reset_flags();
                self.cpu.set_flag(Flag::Z, res == 0);
                res
            } // SLL,
            7 => {
                self.cpu.set_flag(Flag::C, value.get_bit(7).unwrap());
                let mut res = value >> 1;

                self.cpu.set_flag(Flag::Z, res == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                res
            } // SRL,
            _ => 0
        }
    }
}
