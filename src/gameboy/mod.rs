#![feature(optin_builtin_traits)]
use Cartridge;
use Cpu;
use Memory;
use cpu::{Register8, Register16, Flag};

pub struct GameBoy {
    pub cpu: Cpu,
    pub cartridge: Cartridge,
    pub mem: Memory,
}



impl GameBoy {
    pub fn new(cpu: Cpu, cartridge: Cartridge, mem: Memory) -> GameBoy {
        GameBoy {
            cpu,
            cartridge,
            mem,
        }
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

}
