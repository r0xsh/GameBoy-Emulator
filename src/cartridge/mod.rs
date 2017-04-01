use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;

pub struct Cartridge(pub Vec<u8>);

impl Cartridge {
    /// Load rom file
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Cartridge> {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(path)?;
        let _ = file.read_to_end(&mut rom)?;
        Ok(Cartridge(rom))
    }

    /// Read a byte from the rom
    pub fn read_byte(&self, addr: usize) -> u8 {
        self.0[addr]
    }

    /// Read a range of bytes from the rom
    pub fn read_range(&self, addr: usize, end: usize) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::new();
        for x in addr..end {
            a.push(self.0[x])
        }
        a
    }
}

