
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

const TITLE: (u16, u16) = (0x0134, 0x0143);
const CARTRIDGE_TYPE: u16 = 0x0147;
const ROM_SIZE: u16 = 0x0148;
const RAM_SIZE: u16 = 0x0149;

#[derive(Debug)]
pub enum MBC {
    ROM,
    MBC1,
    MBC2,
    MBC3,
    MBC4,
    MBC5,
}

pub struct Cartridge(Vec<u8>, usize);

impl Cartridge {
    /// Load rom file
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Cartridge> {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(path)?;
        let size = file.read_to_end(&mut rom)?;
        Ok(Cartridge(rom, size))
    }

    /// Read a byte from the rom
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let a = self.read_byte(addr);
        let b = self.read_byte(addr + 1);
        join_bytes!(b, a)
    }

    /// Read a range of bytes from the rom
    pub fn read_range(&self, addr: (u16, u16)) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::with_capacity((addr.1 - addr.0) as usize);
        for x in addr.0..addr.1 {
            a.push(self.0[x as usize])
        }
        a
    }

    /// Read the rom's title
    pub fn read_title(&self) -> String {
        let mut title = String::with_capacity(16);
        for letter in self.read_range(TITLE) {
            if letter == 0 {
                break;
            }
            title.push(letter as char)
        }
        title
    }

    /// Read mem type
    pub fn cartridge_type(&self) -> MBC {
        match self.read_byte(CARTRIDGE_TYPE) {
            0x00 | 0x8 | 0x9 => MBC::ROM,
            0x1 | 0x2 | 0x3 => MBC::MBC1,
            0x5 | 0x6 => MBC::MBC2,
            0xF | 0x10 | 0x11 | 0x12 | 0x13 => MBC::MBC3,
            0x15 | 0x16 | 0x17 => MBC::MBC4,
            0x19 | 0x1B | 0x1C | 0x1D | 0x1E => MBC::MBC5,
            _ => unreachable!(),
        }
    }

    /// Return the size of the cartridge
    pub fn size(&self) -> usize {
        self.1
    }
}

impl fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "=== ROM DEBUG ===\n\
            > title game {}\n\
            > cartridge type: {:?}\n\
            > cartridge size: {}\n\
            > ROM size: {:#x}\n\
            > RAM size: {:#x}",
               self.read_title(),
               self.cartridge_type(),
               self.size(),
               self.read_byte(ROM_SIZE),
               self.read_byte(RAM_SIZE))
    }
}
