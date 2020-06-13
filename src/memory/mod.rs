/*
 * CPU   Main Memory (2)
 *  |         |
 *  ------------------------
 *       |                 |
 *   Registers (1)        ROM (3)
 *
 *
 */

const MEMORY_SIZE: usize = 65536; // 64 * 1024 (64KB)

#[derive(Debug)]
pub struct Memory(Vec<u8>);

impl Memory {
    /// Init a new memory unit
    pub fn new() -> Memory {
        let mut mem: Vec<u8> = Vec::with_capacity(MEMORY_SIZE);
        for _ in 0..MEMORY_SIZE {
            mem.push(0);
        }
        Memory(mem)
    }

    /// Read a byte from the memory
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let a = self.read_byte(addr);
        let b = self.read_byte(addr + 1);
        join_bytes!(b, a)
    }

    /*/// Read a range of bytes from the memory
    pub fn read_range(&self, addr: (u16, u16)) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::with_capacity((addr.1 - addr.0) as usize);
        for x in addr.0..addr.1 {
            a.push(self.0[x as usize])
        }
        a
    }*/

    /// Write a byte to the memory
    pub fn write_byte(&mut self, addr: u16, v: u8) {
        self.0[addr as usize] = v;
    }

    pub fn write_word(&mut self, addr: u16, v: u16) {
        self.write_byte(addr + 1, high_byte!(v));
        self.write_byte(addr, low_byte!(v));
    }
}

#[test]
fn read_write() {
    let mut mem = Memory::new();
    mem.write_byte(0xdeff, 0xb1);
    assert_eq!(mem.read_byte(0xdeff), 0xb1);
    mem.write_byte(0xffff, 0xff);
    assert_eq!(mem.read_byte(0xffff), 0xff);
    mem.write_byte(0x0000, 0xde);
    assert_eq!(mem.read_byte(0x0000), 0xde);
}
