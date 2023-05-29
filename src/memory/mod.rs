/*
 * CPU   Main Memory (2)
 *  |         |
 *  ------------------------
 *       |                 |
 *   Registers (1)        ROM (3)
 *
 *
 */

use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use ::{high_byte, low_byte};
use cartridge::Cartridge;
use cpu::Cpu;
use join_bytes;

const MEMORY_SIZE: usize = 65536; // 64 * 1024 (64KB)

#[derive(Debug)]
pub struct Memory{
    pub mem: Vec<u8>,
}

impl Memory {
    /// Init a new memory unit
    pub fn new() -> Memory {
        let mut mem: Vec<u8> = Vec::with_capacity(MEMORY_SIZE);
        for _ in 0..MEMORY_SIZE {
            mem.push(0);
        }

        let mut i = Memory{
            mem,
        };

        i
    }

    /// Read a byte from the memory
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let a = self.read_byte(addr);
        let b = self.read_byte(addr + 1);
        join_bytes!(b, a)
    }

    /// Write a byte to the memory
    pub fn write_byte(&mut self, addr: u16, v: u8) {
        self.mem[addr as usize] = v;
    }

    pub fn write_word(&mut self, addr: u16, v: u16) {
        self.write_byte(addr + 1, high_byte!(v));
        self.write_byte(addr, low_byte!(v));
    }

    pub fn read_range(&self, addr: (u16, u16)) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::with_capacity((addr.1 - addr.0) as usize);
        for x in addr.0..addr.1 {
            a.push(self.mem[x as usize])
        }
        a
    }

    pub fn get_from_file(path: String) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        let mut file = File::open(path).unwrap();
        let size = file.read_to_end(&mut data).unwrap();
        return data;
    }

    pub fn write_from_file(&mut self, addr: u16, path: String) {
        let mut rom: Vec<u8> = Self::get_from_file(path);
        let mut i = addr;
        for b in rom {
            self.write_byte(i, b);
            i = i + 1;
        }
    }
}

/*#[test]
fn read_write() {
    let rom = Cartridge::empty(0xFFFF).unwrap();
    let mut mem = Memory::new(rom);
    mem.write_byte(0xdeff, 0xb1);
    assert_eq!(mem.read_byte(0xdeff), 0xb1);
    mem.write_byte(0xffff, 0xff);
    assert_eq!(mem.read_byte(0xffff), 0xff);
    mem.write_byte(0x0000, 0xde);
    assert_eq!(mem.read_byte(0x0000), 0xde);
}*/
